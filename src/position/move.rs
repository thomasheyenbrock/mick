use crate::{
    castle::{Castle, CastlingRights},
    piece::{Piece, PieceKind},
    r#move::Move,
    side::Side,
    square::Square,
};

use super::{zorbist::Zorbist, Position, State};

impl Position {
    pub fn make(&mut self, m: &Move) -> (Piece, State) {
        let prev_state = self.state.clone();

        let opponent = !self.side_to_move;

        let from = m.from();
        let from_square_index = from.to_usize();

        let to = m.to();
        let to_square_index = to.to_usize();

        let moved_piece = self.pieces[from_square_index];
        let captured_piece = self.pieces[to_square_index];

        let side_index = self.side_to_move.to_usize();
        let moved_piece_index = moved_piece.to_usize();

        // Move the piece
        self.pieces[from_square_index] = Piece::NONE;
        self.piece_boards[moved_piece_index].flip_square(&from);
        self.side_boards[side_index].flip_square(&from);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &from);

        self.pieces[to_square_index] = moved_piece;
        self.piece_boards[moved_piece_index].flip_square(&to);
        self.side_boards[side_index].flip_square(&to);
        self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &to);

        // Handle captures
        if captured_piece.is_some() {
            self.piece_boards[captured_piece.to_usize()].flip_square(&to);
            self.side_boards[opponent.to_usize()].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &captured_piece, &to);
        }

        // Handle promotions
        if let Some(promotion_piece_kind) = m.promotion_piece_kind() {
            let promotion_piece = promotion_piece_kind.to_piece(&self.side_to_move);

            self.pieces[to_square_index] = promotion_piece;

            self.piece_boards[moved_piece_index].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &to);

            self.piece_boards[promotion_piece.to_usize()].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &promotion_piece, &to);
        }

        // Handle en passant
        if m.is_en_passant_capture() {
            if self.side_to_move == Side::WHITE {
                let captured_pawn = Square::new(to.to_u8() - 8);
                self.piece_boards[Piece::BLACK_PAWN.to_usize()].flip_square(&captured_pawn);
                self.side_boards[Side::BLACK.to_usize()].flip_square(&captured_pawn);
            } else {
                let captured_pawn = Square::new(to.to_u8() + 8);
                self.piece_boards[Piece::WHITE_PAWN.to_usize()].flip_square(&captured_pawn);
                self.side_boards[Side::WHITE.to_usize()].flip_square(&captured_pawn);
            };
        }
        if let Some(en_passant_target) = &self.state.en_passant_target {
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, en_passant_target);
            self.state.en_passant_target = None;
        }

        // Handle double pawn pushes
        if m.is_double_pawn_push() {
            let en_passant_target = Square::new(if self.side_to_move == Side::WHITE {
                from.to_u8() + 8
            } else {
                from.to_u8() - 8
            });
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, &en_passant_target);
            self.state.en_passant_target = Some(en_passant_target);
        }

        // Handle castles
        if let Some(castle) = m.castle() {
            let (rook, rook_from_index, rook_to_index) = match (castle, self.side_to_move) {
                (Castle::KINGSIDE, Side::WHITE) => (Piece::WHITE_ROOK, 7, 5),
                (Castle::QUEENSIDE, Side::WHITE) => (Piece::WHITE_ROOK, 0, 3),
                (Castle::KINGSIDE, Side::BLACK) => (Piece::BLACK_ROOK, 63, 61),
                (Castle::QUEENSIDE, Side::BLACK) => (Piece::BLACK_ROOK, 56, 59),
                _ => unreachable!(),
            };

            let rook_index = rook.to_usize();
            let rook_from_square = Square::new(rook_from_index as u8);
            let rook_to_square = Square::new(rook_to_index as u8);

            self.pieces[rook_from_index] = Piece::NONE;
            self.piece_boards[rook_index].flip_square(&rook_from_square);
            self.side_boards[side_index].flip_square(&rook_from_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_from_square);

            self.pieces[rook_to_index] = rook;
            self.piece_boards[rook_index].flip_square(&rook_to_square);
            self.side_boards[side_index].flip_square(&rook_to_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_to_square);
        }
        if moved_piece == Piece::WHITE_KING {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE),
            );
            self.state.castling_rights = self.state.castling_rights & CastlingRights::BLACK;
        } else if moved_piece == Piece::BLACK_KING {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK),
            );
            self.state.castling_rights = self.state.castling_rights & CastlingRights::WHITE;
        } else if from_square_index == 7 && moved_piece == Piece::WHITE_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE_KINGSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_WHITE_KINGSIDE;
        } else if from_square_index == 63 && moved_piece == Piece::BLACK_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK_KINGSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_BLACK_KINGSIDE;
        } else if from_square_index == 0 && moved_piece == Piece::WHITE_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::WHITE_QUEENSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_WHITE_QUEENSIDE;
        } else if from_square_index == 56 && moved_piece == Piece::BLACK_ROOK {
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(
                self.hash,
                &(self.state.castling_rights & CastlingRights::BLACK_QUEENSIDE),
            );
            self.state.castling_rights =
                self.state.castling_rights & CastlingRights::NOT_BLACK_QUEENSIDE;
        }

        self.state.halfmove_clock = if moved_piece == Piece::WHITE_PAWN
            || moved_piece == Piece::BLACK_PAWN
            || captured_piece != Piece::NONE
        {
            0
        } else {
            prev_state.halfmove_clock + 1
        };

        if self.side_to_move == Side::BLACK {
            self.state.fullmove_number += 1;
        }

        self.side_to_move = opponent;
        self.hash = Zorbist::DEFAULT.toggle_side(self.hash);

        (captured_piece, prev_state)
    }

    fn unmake(&mut self, m: &Move, captured_piece: Piece, prev_state: State) {
        let opponent = self.side_to_move;

        self.side_to_move = !opponent;
        self.hash = Zorbist::DEFAULT.toggle_side(self.hash);

        let from = m.from();
        let from_square_index = from.to_usize();

        let to = m.to();
        let to_square_index = to.to_usize();

        let moved_piece = self.pieces[to_square_index];

        let side_index = self.side_to_move.to_usize();
        let moved_piece_index = moved_piece.to_usize();

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
            self.piece_boards[captured_piece.to_usize()].flip_square(&to);
            self.side_boards[opponent.to_usize()].flip_square(&to);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &captured_piece, &to);
        }

        // Handle promotions
        if m.promotion_piece_kind().is_some() {
            let pawn = PieceKind::PAWN.to_piece(&self.side_to_move);

            self.pieces[from_square_index] = pawn;

            self.piece_boards[moved_piece_index].flip_square(&from);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &moved_piece, &from);

            self.piece_boards[pawn.to_usize()].flip_square(&from);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &pawn, &from);
        }

        // Handle en passant
        if m.is_en_passant_capture() {
            if self.side_to_move == Side::WHITE {
                let captured_pawn = Square::new(to.to_u8() - 8);
                self.piece_boards[Piece::BLACK_PAWN.to_usize()].flip_square(&captured_pawn);
                self.side_boards[Side::BLACK.to_usize()].flip_square(&captured_pawn);
            } else {
                let captured_pawn = Square::new(to.to_u8() + 8);
                self.piece_boards[Piece::WHITE_PAWN.to_usize()].flip_square(&captured_pawn);
                self.side_boards[Side::WHITE.to_usize()].flip_square(&captured_pawn);
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
                (Castle::KINGSIDE, Side::WHITE) => (Piece::WHITE_ROOK, 7, 5),
                (Castle::QUEENSIDE, Side::WHITE) => (Piece::WHITE_ROOK, 0, 3),
                (Castle::KINGSIDE, Side::BLACK) => (Piece::BLACK_ROOK, 63, 61),
                (Castle::QUEENSIDE, Side::BLACK) => (Piece::BLACK_ROOK, 56, 59),
                _ => unreachable!(),
            };

            let rook_index = rook.to_usize();
            let rook_from_square = Square::new(rook_from_index as u8);
            let rook_to_square = Square::new(rook_to_index as u8);

            self.pieces[rook_from_index] = rook;
            self.piece_boards[rook_index].flip_square(&rook_from_square);
            self.side_boards[side_index].flip_square(&rook_from_square);
            self.hash = Zorbist::DEFAULT.toggle_piece(self.hash, &rook, &rook_from_square);

            self.pieces[rook_to_index] = Piece::NONE;
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
        piece::{Piece, PieceKind},
        position::Position,
        r#move::Move,
        side::Side,
        square::Square,
    };

    #[test]
    fn king_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(0), &Square::new(8));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[0], Piece::NONE);
        assert_eq!(p2.pieces[8], Piece::WHITE_KING);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn king_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/8/p7/K7 w - - 0 1");
        let m = Move::new_capture(&Square::new(0), &Square::new(8));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[0], Piece::NONE);
        assert_eq!(p2.pieces[8], Piece::WHITE_KING);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_kingside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_castle(&Square::new(4), &Square::new(6), &Castle::KINGSIDE);

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[4], Piece::NONE);
        assert_eq!(p2.pieces[7], Piece::NONE);
        assert_eq!(p2.pieces[6], Piece::WHITE_KING);
        assert_eq!(p2.pieces[5], Piece::WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0040)
        );
        assert_eq!(
            p2.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0000_0000_0000_0021)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0061)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn castle_queenside() {
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_castle(&Square::new(4), &Square::new(2), &Castle::QUEENSIDE);

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[4], Piece::NONE);
        assert_eq!(p2.pieces[0], Piece::NONE);
        assert_eq!(p2.pieces[2], Piece::WHITE_KING);
        assert_eq!(p2.pieces[3], Piece::WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0004)
        );
        assert_eq!(
            p2.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0000_0000_0000_0088)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_008C)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/Q7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(62));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[62], Piece::WHITE_QUEEN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn queen_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/Q7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(62));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[62], Piece::WHITE_QUEEN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/R7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(56));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[56], Piece::WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn rook_capture() {
        let p1 = Position::from_fen("p7/8/8/8/8/8/R7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(56));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[56], Piece::WHITE_ROOK);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/B7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(62));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[62], Piece::WHITE_BISHOP);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_BISHOP.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bishop_capture() {
        let p1 = Position::from_fen("6p1/8/8/8/8/8/B7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(62));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[62], Piece::WHITE_BISHOP);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_BISHOP.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/N7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(25));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[25], Piece::WHITE_KNIGHT);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KNIGHT.to_usize()],
            Board::new(0x0000_0000_0200_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0200_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn knight_capture() {
        let p1 = Position::from_fen("8/8/8/8/1p6/8/N7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(25));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[25], Piece::WHITE_KNIGHT);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_KNIGHT.to_usize()],
            Board::new(0x0000_0000_0200_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0200_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_single_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let m = Move::new_push(&Square::new(8), &Square::new(16));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[16], Piece::WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0001_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0001_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_double_push() {
        let p1 = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        let m = Move::new_push_double_pawn(&Square::new(8), &Square::new(24));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[24], Piece::WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0100_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0100_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, Some(Square::new(16)));
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_push_promotion() {
        let p1 = Position::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_push_promotion(&Square::new(48), &Square::new(56), &PieceKind::QUEEN);

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[48], Piece::NONE);
        assert_eq!(p2.pieces[56], Piece::WHITE_QUEEN);
        assert_eq!(p2.piece_boards[Piece::WHITE_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture() {
        let p1 = Position::from_fen("8/8/8/8/8/1p6/P7/K7 w - - 0 1");
        let m = Move::new_capture(&Square::new(8), &Square::new(17));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[8], Piece::NONE);
        assert_eq!(p2.pieces[17], Piece::WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0002_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0002_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_promotion() {
        let p1 = Position::from_fen("1p6/P7/8/8/8/8/8/K7 w - - 0 1");
        let m = Move::new_capture_promotion(&Square::new(48), &Square::new(57), &PieceKind::QUEEN);

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[48], Piece::NONE);
        assert_eq!(p2.pieces[57], Piece::WHITE_QUEEN);
        assert_eq!(p2.piece_boards[Piece::WHITE_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x0200_0000_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0200_0000_0000_0001)
        );
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pawn_capture_en_passant() {
        let p1 = Position::from_fen("8/8/8/Pp6/8/8/8/K7 w - b6 0 1");
        let m = Move::new_capture_en_passant(&Square::new(32), &Square::new(41));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.pieces[32], Piece::NONE);
        assert_eq!(p2.pieces[40], Piece::NONE);
        assert_eq!(p2.pieces[41], Piece::WHITE_PAWN);
        assert_eq!(
            p2.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0200_0000_0000)
        );
        assert_eq!(
            p2.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0200_0000_0001)
        );
        assert_eq!(p2.piece_boards[Piece::BLACK_PAWN.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(p2.side_to_move, Side::BLACK);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(p2.state.en_passant_target, None);
        assert_eq!(p2.state.halfmove_clock, 0);
        assert_eq!(p2.state.fullmove_number, 1);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn removing_castling_right() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(&Square::new(4), &Square::new(5));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::NO_RIGHTS);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);

        // When the kingside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(&Square::new(7), &Square::new(6));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::WHITE_QUEENSIDE);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);

        // When the queenside rook moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let m = Move::new_push(&Square::new(0), &Square::new(1));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.state.castling_rights, CastlingRights::WHITE_KINGSIDE);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }

    #[test]
    fn black_to_move() {
        // When the king moves
        let p1 = Position::from_fen("8/8/8/8/8/8/8/k7 b - - 0 1");
        let m = Move::new_push(&Square::new(0), &Square::new(1));

        let mut p2 = p1.clone();
        let (captured_piece, prev_state) = p2.make(&m);
        assert_eq!(p2.side_to_move, Side::WHITE);
        assert_eq!(p2.state.halfmove_clock, 1);
        assert_eq!(p2.state.fullmove_number, 2);

        p2.unmake(&m, captured_piece, prev_state);
        assert_eq!(p1, p2);
    }
}
