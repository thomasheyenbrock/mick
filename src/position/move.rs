use super::{Position, State};
use crate::{
    board::{Board, EMPTY},
    castle::{Castle, CastlingRights},
    hash::DEFAULT_ZOBRISH_HASH,
    piece::{Piece, NULL_PIECE, PAWN},
    r#move::Move,
    side::{Side, BLACK},
    square::{Square, A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8},
};

impl Position {
    pub fn make(&mut self, m: Move) -> Option<(Piece, Square)> {
        let side_to_move = self.state.side_to_move;
        let initial_state = self.state.clone();
        let mut move_resets_half_move_clock = false;

        if self.state.side_to_move == BLACK {
            self.state.fullmove_number += 1;
        }
        self.state.side_to_move = !self.state.side_to_move;

        self.state.en_passant_target = None;
        let mut captured = None;

        let mut xor_key = 0;

        if let Some(castle) = m.castle() {
            self.state.castling_rights.clear_side(side_to_move);

            let (king_from, king_to, rook_from, rook_to) = castle_squares(side_to_move, castle);
            self.move_piece(king_from, king_to);
            self.move_piece(rook_from, rook_to);

            xor_key ^= DEFAULT_ZOBRISH_HASH.castle(castle, side_to_move);
        } else {
            let from = m.from();
            let to = m.to();

            if m.is_capture() {
                move_resets_half_move_clock = true;

                let capture_sq = if m.is_en_passant_capture() {
                    from.along_row_with_col(to)
                } else {
                    to
                };

                let captured_piece = self.at(capture_sq);

                self.remove(capture_sq);

                captured = Some((captured_piece, capture_sq));

                xor_key ^= DEFAULT_ZOBRISH_HASH.capture(captured_piece, capture_sq);
            }

            let mover = self.at(from);

            let move_mask = self.move_piece(from, to);
            let mut updated_mover = mover;

            if mover.kind() == PAWN {
                move_resets_half_move_clock = true;

                if m.is_double_pawn_push() {
                    self.state.en_passant_target = Some(Square((to.0 + from.0) >> 1));
                }
            }

            if let Some(kind) = m.promote_to() {
                updated_mover = kind.to_piece(side_to_move);
                self.promote_piece(to, updated_mover);
            }

            xor_key ^= DEFAULT_ZOBRISH_HASH.push(mover, from, updated_mover, to);

            for (i, mask) in CASTLE_MASKS.iter().enumerate() {
                if (move_mask & *mask) != EMPTY {
                    self.state.castling_rights.clear(CastlingRights(1 << i));
                }
            }
        }

        if move_resets_half_move_clock {
            self.state.halfmove_clock = 0;
        } else {
            self.state.halfmove_clock += 1;
        }

        xor_key ^= DEFAULT_ZOBRISH_HASH.state(&initial_state, &self.state);

        self.hash ^= xor_key;

        captured
    }

    pub fn unmake(
        &mut self,
        mv: Move,
        capture: Option<(Piece, Square)>,
        original_state: &State,
        original_hash: u64,
    ) {
        self.state = original_state.clone();
        self.hash = original_hash;

        if let Some(castle) = mv.castle() {
            let (king_to, king_from, rook_to, rook_from) =
                castle_squares(original_state.side_to_move, castle);
            self.move_piece(king_from, king_to);
            self.move_piece(rook_from, rook_to);

            return;
        }

        if mv.promote_to().is_some() {
            let mover = PAWN.to_piece(original_state.side_to_move);
            self.promote_piece(mv.to(), mover);
        }

        self.move_piece(mv.to(), mv.from());

        if let Some((captured_piece, capture_sq)) = capture {
            self.put(captured_piece, capture_sq);
        }
    }

    fn move_piece(&mut self, from: Square, to: Square) -> Board {
        let piece = self.at(from);
        let mask = Board::new(from) | Board::new(to);

        self.update_grid(from, NULL_PIECE);
        self.update_grid(to, piece);

        unsafe {
            *self.piece_boards.get_unchecked_mut(piece.0 as usize) ^= mask;
            *self.side_boards.get_unchecked_mut(piece.side().0 as usize) ^= mask;
        }

        mask
    }

    fn promote_piece(&mut self, square: Square, new_piece: Piece) {
        let old_piece = self.at(square);
        let mask = Board::new(square);

        self.update_grid(square, new_piece);

        unsafe {
            *(self.piece_boards.get_unchecked_mut(old_piece.0 as usize)) ^= mask;
            *(self.piece_boards.get_unchecked_mut(new_piece.0 as usize)) |= mask;
        }
    }

    fn put(&mut self, piece: Piece, square: Square) {
        let mask = Board::new(square);

        self.update_grid(square, piece);

        unsafe {
            *self.piece_boards.get_unchecked_mut(piece.0 as usize) ^= mask;
            *self.side_boards.get_unchecked_mut(piece.side().0 as usize) ^= mask;
        }
    }

    fn remove(&mut self, square: Square) {
        let piece = self.at(square);
        let mask = Board::new(square);

        self.update_grid(square, NULL_PIECE);

        unsafe {
            *self.piece_boards.get_unchecked_mut(piece.0 as usize) ^= mask;
            *self.side_boards.get_unchecked_mut(piece.side().0 as usize) ^= mask;
        }
    }

    fn update_grid(&mut self, square: Square, piece: Piece) {
        unsafe {
            *(self.pieces.get_unchecked_mut(square.0 as usize)) = piece;
        }
    }
}

const CASTLE_MASKS: [Board; 4] = [
    Board((1 << 4) | (1 << 7)),         // WHITE KS: E1 + H1
    Board(1 | (1 << 4)),                // WHITE QS: A1 + E1
    Board(((1 << 4) | (1 << 7)) << 56), // BLACK KS: E8 + H8
    Board((1 | (1 << 4)) << 56),        // BLACK QS: A8 + E8
];

const CASTLE_MOVES: [[(Square, Square, Square, Square); 2]; 2] = [
    [(E1, G1, H1, F1), (E8, G8, H8, F8)],
    [(E1, C1, A1, D1), (E8, C8, A8, D8)],
];

pub fn castle_squares(side: Side, castle: Castle) -> (Square, Square, Square, Square) {
    CASTLE_MOVES[castle.0 as usize][side.0 as usize]
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, EMPTY},
        castle::{KING_SIDE, NO_RIGHTS, QUEEN_SIDE, WHITE_KING_SIDE, WHITE_QUEEN_SIDE},
        piece::{
            BLACK_PAWN, NULL_PIECE, QUEEN, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
            WHITE_QUEEN, WHITE_ROOK,
        },
        r#move::Move,
        side::{BLACK, WHITE},
        square::Square,
        Position,
    };

    #[test]
    fn king_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(0), Square(8));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[8], WHITE_KING);
        assert_eq!(p2.piece(WHITE_KING), Board(0x0000_0000_0000_0100));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0000_0100));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn king_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/8/p7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(0), Square(8));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[8], WHITE_KING);
        assert_eq!(p2.piece(WHITE_KING), Board(0x0000_0000_0000_0100));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0000_0100));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_kingside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_castle(Square(4), Square(6), KING_SIDE);

        let capture = p2.make(m);
        assert_eq!(p2.pieces[4], NULL_PIECE);
        assert_eq!(p2.pieces[7], NULL_PIECE);
        assert_eq!(p2.pieces[6], WHITE_KING);
        assert_eq!(p2.pieces[5], WHITE_ROOK);
        assert_eq!(p2.piece(WHITE_KING), Board(0x0000_0000_0000_0040));
        assert_eq!(p2.piece(WHITE_ROOK), Board(0x0000_0000_0000_0021));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0000_0061));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_queenside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_castle(Square(4), Square(2), QUEEN_SIDE);

        let capture = p2.make(m);
        assert_eq!(p2.pieces[4], NULL_PIECE);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[2], WHITE_KING);
        assert_eq!(p2.pieces[3], WHITE_ROOK);
        assert_eq!(p2.piece(WHITE_KING), Board(0x0000_0000_0000_0004));
        assert_eq!(p2.piece(WHITE_ROOK), Board(0x0000_0000_0000_0088));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0000_008C));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/Q7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(8), Square(62));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_QUEEN);
        assert_eq!(p2.piece(WHITE_QUEEN), Board(0x4000_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x4000_0000_0000_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/Q7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(8), Square(62));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_QUEEN);
        assert_eq!(p2.piece(WHITE_QUEEN), Board(0x4000_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x4000_0000_0000_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/R7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(8), Square(56));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_ROOK);
        assert_eq!(p2.piece(WHITE_ROOK), Board(0x0100_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x0100_0000_0000_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_capture() {
        let p1 = Position::from_fen("p7/8/8/8/8/8/R7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(8), Square(56));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_ROOK);
        assert_eq!(p2.piece(WHITE_ROOK), Board(0x0100_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x0100_0000_0000_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/B7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(8), Square(62));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_BISHOP);
        assert_eq!(p2.piece(WHITE_BISHOP), Board(0x4000_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x4000_0000_0000_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/B7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(8), Square(62));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_BISHOP);
        assert_eq!(p2.piece(WHITE_BISHOP), Board(0x4000_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x4000_0000_0000_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/N7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(8), Square(25));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[25], WHITE_KNIGHT);
        assert_eq!(p2.piece(WHITE_KNIGHT), Board(0x0000_0000_0200_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0200_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_capture() {
        let p1 = Position::from_fen("8/8/8/8/1p6/8/N7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(8), Square(25));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[25], WHITE_KNIGHT);
        assert_eq!(p2.piece(WHITE_KNIGHT), Board(0x0000_0000_0200_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0200_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_single_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(8), Square(16));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[16], WHITE_PAWN);
        assert_eq!(p2.piece(WHITE_PAWN), Board(0x0000_0000_0001_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0001_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_double_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push_double_pawn(Square(8), Square(24));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[24], WHITE_PAWN);
        assert_eq!(p2.piece(WHITE_PAWN), Board(0x0000_0000_0100_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0100_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, Some(Square(16)));
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_push_promotion() {
        let p1 = Position::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push_promotion(Square(48), Square(56), QUEEN);

        let capture = p2.make(m);
        assert_eq!(p2.pieces[48], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_QUEEN);
        assert_eq!(p2.piece(WHITE_PAWN), EMPTY);
        assert_eq!(p2.piece(WHITE_QUEEN), Board(0x0100_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x0100_0000_0000_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/1p6/P7/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture(Square(8), Square(17));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[17], WHITE_PAWN);
        assert_eq!(p2.piece(WHITE_PAWN), Board(0x0000_0000_0002_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0000_0002_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_promotion() {
        let p1 = Position::from_fen("1p6/P7/8/8/8/8/8/K7 w - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture_promotion(Square(48), Square(57), QUEEN);

        let capture = p2.make(m);
        assert_eq!(p2.pieces[48], NULL_PIECE);
        assert_eq!(p2.pieces[57], WHITE_QUEEN);
        assert_eq!(p2.piece(WHITE_PAWN), EMPTY);
        assert_eq!(p2.piece(WHITE_QUEEN), Board(0x0200_0000_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x0200_0000_0000_0001));
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_en_passant() {
        let p1 = Position::from_fen("8/8/8/Pp6/8/8/8/K7 w - b6 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_capture_en_passant(Square(32), Square(41));

        let capture = p2.make(m);
        assert_eq!(p2.pieces[32], NULL_PIECE);
        assert_eq!(p2.pieces[40], NULL_PIECE);
        assert_eq!(p2.pieces[41], WHITE_PAWN);
        assert_eq!(p2.piece(WHITE_PAWN), Board(0x0000_0200_0000_0000));
        assert_eq!(p2.side(WHITE), Board(0x0000_0200_0000_0001));
        assert_eq!(p2.piece(BLACK_PAWN), EMPTY);
        assert_eq!(p2.side(BLACK), EMPTY);
        assert_eq!(p2.state.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn removing_castling_right() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(4), Square(5));

        let capture = p2.make(m);
        assert_eq!(p2.state.castling_rights, NO_RIGHTS);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);

        // When the kingside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(7), Square(6));

        let capture = p2.make(m);
        assert_eq!(p2.state.castling_rights, WHITE_QUEEN_SIDE);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);

        // When the queenside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(0), Square(1));

        let capture = p2.make(m);
        assert_eq!(p2.state.castling_rights, WHITE_KING_SIDE);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }

    #[test]
    fn black_to_move() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/k7 b - - 0 1");
        let mut p2 = p1.clone();
        let p2_state = p2.state.clone();
        let p2_hash = p2.hash;
        let m = Move::new_push(Square(0), Square(1));

        let capture = p2.make(m);
        assert_eq!(p2.state.side_to_move, WHITE);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 2);

        p2.unmake(m, capture, &p2_state, p2_hash);
        assert_eq!(p1, p2);
    }
}
