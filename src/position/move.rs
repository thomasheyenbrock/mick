use crate::{
    castle::{Castle, CastlingRights},
    piece::{
        Piece, BLACK_KING, BLACK_PAWN, BLACK_ROOK, NULL_PIECE, PAWN, WHITE_KING, WHITE_PAWN,
        WHITE_ROOK,
    },
    r#move::Move,
    side::{BLACK, WHITE},
    square::Square,
};

use super::{zorbist::Zorbist, Position, State};

impl Position {
    pub fn make(&self, m: &Move) -> (Self, Piece, State) {
        let mut cloned = self.clone();
        let (captured_piece, prev_state) = cloned.make_mut(m);
        (cloned, captured_piece, prev_state)
    }

    pub fn make_mut(&mut self, m: &Move) -> (Piece, State) {
        let prev_state = self.state.clone();

        let opponent = !self.side_to_move;

        let from = m.from();
        let from_square_index = from.0 as usize;

        let to = m.to();
        let to_square_index = to.0 as usize;

        let moved_piece = self.pieces[from_square_index];
        let captured_piece = self.pieces[to_square_index];

        let side_index = self.side_to_move.0 as usize;
        let moved_piece_index = moved_piece.0 as usize;

        // Move the piece
        self.pieces[from_square_index] = NULL_PIECE;
        self.piece_boards[moved_piece_index].flip_square(&from);
        self.side_boards[side_index].flip_square(&from);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &from);

        self.pieces[to_square_index] = moved_piece;
        self.piece_boards[moved_piece_index].flip_square(&to);
        self.side_boards[side_index].flip_square(&to);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &to);

        // Handle captures
        if captured_piece.is_some() {
            self.piece_boards[captured_piece.0 as usize].flip_square(&to);
            self.side_boards[opponent.0 as usize].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &captured_piece, &to);
        }

        // Handle promotions
        if let Some(promotion_piece_kind) = m.promote_to() {
            let promotion_piece = promotion_piece_kind.to_piece(self.side_to_move);

            self.pieces[to_square_index] = promotion_piece;

            self.piece_boards[moved_piece_index].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &to);

            self.piece_boards[promotion_piece.0 as usize].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &promotion_piece, &to);
        }

        // Handle en passant
        if m.is_en_passant_capture() {
            if self.side_to_move == WHITE {
                let captured_pawn = Square(to.0 - 8);
                self.piece_boards[BLACK_PAWN.0 as usize].flip_square(&captured_pawn);
                self.side_boards[BLACK.0 as usize].flip_square(&captured_pawn);
            } else {
                let captured_pawn = Square(to.0 + 8);
                self.piece_boards[WHITE_PAWN.0 as usize].flip_square(&captured_pawn);
                self.side_boards[WHITE.0 as usize].flip_square(&captured_pawn);
            };
        }
        if let Some(en_passant_target) = &self.state.en_passant_target {
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, en_passant_target);
            self.state.en_passant_target = None;
        }

        // Handle double pawn pushes
        if m.is_double_pawn_push() {
            let en_passant_target = Square(if self.side_to_move == WHITE {
                from.0 + 8
            } else {
                from.0 - 8
            });
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, &en_passant_target);
            self.state.en_passant_target = Some(en_passant_target);
        }

        // Handle castles
        if let Some(castle) = m.castle() {
            let (rook, rook_from_index, rook_to_index) = match (castle, self.side_to_move) {
                (Castle::KINGSIDE, WHITE) => (WHITE_ROOK, 7, 5),
                (Castle::QUEENSIDE, WHITE) => (WHITE_ROOK, 0, 3),
                (Castle::KINGSIDE, BLACK) => (BLACK_ROOK, 63, 61),
                (Castle::QUEENSIDE, BLACK) => (BLACK_ROOK, 56, 59),
                _ => unreachable!(),
            };

            let rook_index = rook.0 as usize;
            let rook_from_square = Square(rook_from_index as u8);
            let rook_to_square = Square(rook_to_index as u8);

            self.pieces[rook_from_index] = NULL_PIECE;
            self.piece_boards[rook_index].flip_square(&rook_from_square);
            self.side_boards[side_index].flip_square(&rook_from_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_from_square);

            self.pieces[rook_to_index] = rook;
            self.piece_boards[rook_index].flip_square(&rook_to_square);
            self.side_boards[side_index].flip_square(&rook_to_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_to_square);
        }
        if moved_piece == WHITE_KING {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE),
            );
            self.state.castling_rights = self.state.castling_rights & CastlingRights::BLACK;
        } else if moved_piece == BLACK_KING {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK),
            );
            self.state.castling_rights = self.state.castling_rights & CastlingRights::WHITE;
        } else if from_square_index == 7 && moved_piece == WHITE_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE_KINGSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_WHITE_KINGSIDE;
        } else if from_square_index == 63 && moved_piece == BLACK_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK_KINGSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_BLACK_KINGSIDE;
        } else if from_square_index == 0 && moved_piece == WHITE_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE_QUEENSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_WHITE_QUEENSIDE;
        } else if from_square_index == 56 && moved_piece == BLACK_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK_QUEENSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_BLACK_QUEENSIDE;
        }

        self.state.halfmove_clock = if moved_piece == WHITE_PAWN
            || moved_piece == BLACK_PAWN
            || captured_piece != NULL_PIECE
        {
            0
        } else {
            prev_state.halfmove_clock + 1
        };

        if self.side_to_move == BLACK {
            self.state.fullmove_number += 1;
        }

        self.side_to_move = opponent;
        self.hash = Zorbist::DEFAULT.toggle_side(self.hash);

        (captured_piece, prev_state)
    }

    fn unmake(&self, m: &Move, captured_piece: Piece, prev_state: State) {
        let mut cloned = self.clone();
        cloned.unmake_mut(m, captured_piece, prev_state);
    }

    fn unmake_mut(&mut self, m: &Move, captured_piece: Piece, prev_state: State) {
        let opponent = self.side_to_move;

        self.side_to_move = !opponent;
        self.hash = Zorbist::DEFAULT.toggle_side(self.hash);

        let from = m.from();
        let from_square_index = from.0 as usize;

        let to = m.to();
        let to_square_index = to.0 as usize;

        let moved_piece = self.pieces[to_square_index];

        let side_index = self.side_to_move.0 as usize;
        let moved_piece_index = moved_piece.0 as usize;

        // Move the piece
        self.pieces[from_square_index] = moved_piece;
        self.piece_boards[moved_piece_index].flip_square(&from);
        self.side_boards[side_index].flip_square(&from);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &from);

        self.pieces[to_square_index] = captured_piece;
        self.piece_boards[moved_piece_index].flip_square(&to);
        self.side_boards[side_index].flip_square(&to);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &to);

        // Handle captures
        if captured_piece.is_some() {
            self.piece_boards[captured_piece.0 as usize].flip_square(&to);
            self.side_boards[opponent.0 as usize].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &captured_piece, &to);
        }

        // Handle promotions
        if m.promote_to().is_some() {
            let pawn = PAWN.to_piece(self.side_to_move);

            self.pieces[from_square_index] = pawn;

            self.piece_boards[moved_piece_index].flip_square(&from);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &from);

            self.piece_boards[pawn.0 as usize].flip_square(&from);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &pawn, &from);
        }

        // Handle en passant
        if m.is_en_passant_capture() {
            if self.side_to_move == WHITE {
                let captured_pawn = Square(to.0 - 8);
                self.piece_boards[BLACK_PAWN.0 as usize].flip_square(&captured_pawn);
                self.side_boards[BLACK.0 as usize].flip_square(&captured_pawn);
            } else {
                let captured_pawn = Square(to.0 + 8);
                self.piece_boards[WHITE_PAWN.0 as usize].flip_square(&captured_pawn);
                self.side_boards[WHITE.0 as usize].flip_square(&captured_pawn);
            };
        }
        if let Some(en_passant_target) = self.state.en_passant_target {
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, &en_passant_target);
        }
        if let Some(en_passant_target) = prev_state.en_passant_target {
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, &en_passant_target);
        }

        // Handle castles
        if let Some(castle) = m.castle() {
            let (rook, rook_from_index, rook_to_index) = match (castle, self.side_to_move) {
                (Castle::KINGSIDE, WHITE) => (WHITE_ROOK, 7, 5),
                (Castle::QUEENSIDE, WHITE) => (WHITE_ROOK, 0, 3),
                (Castle::KINGSIDE, BLACK) => (BLACK_ROOK, 63, 61),
                (Castle::QUEENSIDE, BLACK) => (BLACK_ROOK, 56, 59),
                _ => unreachable!(),
            };

            let rook_index = rook.0 as usize;
            let rook_from_square = Square(rook_from_index as u8);
            let rook_to_square = Square(rook_to_index as u8);

            self.pieces[rook_from_index] = rook;
            self.piece_boards[rook_index].flip_square(&rook_from_square);
            self.side_boards[side_index].flip_square(&rook_from_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_from_square);

            self.pieces[rook_to_index] = NULL_PIECE;
            self.piece_boards[rook_index].flip_square(&rook_to_square);
            self.side_boards[side_index].flip_square(&rook_to_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_to_square);
        }
        self.hash = Zorbist::DEFAULT.toggle_castling_rights(
            self.hash,
            &(prev_state.castling_rights ^ self.state.castling_rights),
        );

        self.state.castling_rights = prev_state.castling_rights;
        self.state.en_passant_target = prev_state.en_passant_target;
        self.state.halfmove_clock = prev_state.halfmove_clock;
        self.state.fullmove_number = prev_state.fullmove_number;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        castle::{Castle, CastlingRights},
        piece::{
            BLACK_PAWN, NULL_PIECE, QUEEN, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
            WHITE_QUEEN, WHITE_ROOK,
        },
        position::Position,
        r#move::Move,
        side::{BLACK, WHITE},
        square::Square,
    };

    #[test]
    fn king_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_push(Square(0), Square(8));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[8], WHITE_KING);
        assert_eq!(
            p2.piece_boards[WHITE_KING.0 as usize],
            Board(0x0000_0000_0000_0100)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0000_0100)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn king_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/8/p7/K7 w - - 0 1");
        let m = Move::new_capture(Square(0), Square(8));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[8], WHITE_KING);
        assert_eq!(
            p2.piece_boards[WHITE_KING.0 as usize],
            Board(0x0000_0000_0000_0100)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0000_0100)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_kingside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_castle(Square(4), Square(6), Castle::KINGSIDE);

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[4], NULL_PIECE);
        assert_eq!(p2.pieces[7], NULL_PIECE);
        assert_eq!(p2.pieces[6], WHITE_KING);
        assert_eq!(p2.pieces[5], WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[WHITE_KING.0 as usize],
            Board(0x0000_0000_0000_0040)
        );
        assert_eq!(
            p2.piece_boards[WHITE_ROOK.0 as usize],
            Board(0x0000_0000_0000_0021)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0000_0061)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_queenside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_castle(Square(4), Square(2), Castle::QUEENSIDE);

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[4], NULL_PIECE);
        assert_eq!(p2.pieces[0], NULL_PIECE);
        assert_eq!(p2.pieces[2], WHITE_KING);
        assert_eq!(p2.pieces[3], WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[WHITE_KING.0 as usize],
            Board(0x0000_0000_0000_0004)
        );
        assert_eq!(
            p2.piece_boards[WHITE_ROOK.0 as usize],
            Board(0x0000_0000_0000_0088)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0000_008C)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/Q7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(62));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_QUEEN);
        assert_eq!(
            p2.piece_boards[WHITE_QUEEN.0 as usize],
            Board(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/Q7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(62));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_QUEEN);
        assert_eq!(
            p2.piece_boards[WHITE_QUEEN.0 as usize],
            Board(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/R7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(56));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[WHITE_ROOK.0 as usize],
            Board(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_capture() {
        let p1 = Position::from_fen("p7/8/8/8/8/8/R7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(56));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[WHITE_ROOK.0 as usize],
            Board(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/B7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(62));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_BISHOP);
        assert_eq!(
            p2.piece_boards[WHITE_BISHOP.0 as usize],
            Board(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/B7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(62));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[62], WHITE_BISHOP);
        assert_eq!(
            p2.piece_boards[WHITE_BISHOP.0 as usize],
            Board(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/N7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(25));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[25], WHITE_KNIGHT);
        assert_eq!(
            p2.piece_boards[WHITE_KNIGHT.0 as usize],
            Board(0x0000_0000_0200_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0200_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_capture() {
        let p1 = Position::from_fen("8/8/8/8/1p6/8/N7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(25));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[25], WHITE_KNIGHT);
        assert_eq!(
            p2.piece_boards[WHITE_KNIGHT.0 as usize],
            Board(0x0000_0000_0200_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0200_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_single_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let m = Move::new_push(Square(8), Square(16));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[16], WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[WHITE_PAWN.0 as usize],
            Board(0x0000_0000_0001_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0001_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_double_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let m = Move::new_push_double_pawn(Square(8), Square(24));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[24], WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[WHITE_PAWN.0 as usize],
            Board(0x0000_0000_0100_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0100_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, Some(Square(16)));
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_push_promotion() {
        let p1 = Position::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_push_promotion(Square(48), Square(56), QUEEN);

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[48], NULL_PIECE);
        assert_eq!(p2.pieces[56], WHITE_QUEEN);
        assert_eq!(p2.piece_boards[WHITE_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(
            p2.piece_boards[WHITE_QUEEN.0 as usize],
            Board(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/1p6/P7/K7 w - - 0 1");
        let m = Move::new_capture(Square(8), Square(17));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[8], NULL_PIECE);
        assert_eq!(p2.pieces[17], WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[WHITE_PAWN.0 as usize],
            Board(0x0000_0000_0002_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0000_0002_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_promotion() {
        let p1 = Position::from_fen("1p6/P7/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_capture_promotion(Square(48), Square(57), QUEEN);

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[48], NULL_PIECE);
        assert_eq!(p2.pieces[57], WHITE_QUEEN);
        assert_eq!(p2.piece_boards[WHITE_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(
            p2.piece_boards[WHITE_QUEEN.0 as usize],
            Board(0x0200_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0200_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_en_passant() {
        let p1 = Position::from_fen("8/8/8/Pp6/8/8/8/K7 w - b6 0 1");
        let m = Move::new_capture_en_passant(Square(32), Square(41));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.pieces[32], NULL_PIECE);
        assert_eq!(p2.pieces[40], NULL_PIECE);
        assert_eq!(p2.pieces[41], WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[WHITE_PAWN.0 as usize],
            Board(0x0000_0200_0000_0000)
        );
        assert_eq!(
            p2.side_boards[WHITE.0 as usize],
            Board(0x0000_0200_0000_0001)
        );
        assert_eq!(p2.piece_boards[BLACK_PAWN.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_boards[BLACK.0 as usize], Board::EMPTY);
        assert_eq!(p2.side_to_move, BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn removing_castling_right() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(Square(4), Square(5));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);

        // When the kingside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(Square(7), Square(6));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::WHITE_QUEENSIDE);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);

        // When the queenside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(Square(0), Square(1));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::WHITE_KINGSIDE);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn black_to_move() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/k7 b - - 0 1");
        let m = Move::new_push(Square(0), Square(1));

        let (mut p2, captured_piece, prev_state) = p1.make(&m);
        assert_eq!(p2.side_to_move, WHITE);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 2);

        p2.unmake_mut(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }
}
