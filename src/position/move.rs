use crate::{
    castle::{Castle, CastlingRights},
    piece::Piece,
    r#move::Move,
    side::Side,
    square::Square,
};

use super::{zorbist::Zorbist, Position};

impl Position {
    pub fn make(&mut self, m: &Move) -> &Self {
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
        if let Some(en_passant_target) = &self.en_passant_target {
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, en_passant_target);
            self.en_passant_target = None;
        }

        // Handle double pawn pushes
        if m.is_double_pawn_push() {
            let en_passant_target = Square::new(if self.side_to_move == Side::WHITE {
                from.to_u8() + 8
            } else {
                from.to_u8() - 8
            });
            self.hash = Zorbist::DEFAULT.toggle_en_passant_target(self.hash, &en_passant_target);
            self.en_passant_target = Some(en_passant_target);
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
            let castling_rights = self.castling_rights & CastlingRights::BLACK;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        } else if moved_piece == Piece::BLACK_KING {
            let castling_rights = self.castling_rights & CastlingRights::WHITE;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        } else if from_square_index == 7 && moved_piece == Piece::WHITE_ROOK {
            let castling_rights = self.castling_rights & CastlingRights::NOT_WHITE_KINGSIDE;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        } else if from_square_index == 63 && moved_piece == Piece::BLACK_ROOK {
            let castling_rights = self.castling_rights & CastlingRights::NOT_BLACK_KINGSIDE;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        } else if from_square_index == 0 && moved_piece == Piece::WHITE_ROOK {
            let castling_rights = self.castling_rights & CastlingRights::NOT_WHITE_QUEENSIDE;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        } else if from_square_index == 56 && moved_piece == Piece::BLACK_ROOK {
            let castling_rights = self.castling_rights & CastlingRights::NOT_BLACK_QUEENSIDE;
            self.hash = Zorbist::DEFAULT.toggle_castling_rights(self.hash, &castling_rights);
            self.castling_rights = castling_rights;
        }

        self.halfmove_clock = if moved_piece == Piece::WHITE_PAWN
            || moved_piece == Piece::BLACK_PAWN
            || captured_piece != Piece::NONE
        {
            0
        } else {
            self.halfmove_clock + 1
        };

        if self.side_to_move == Side::BLACK {
            self.fullmove_number += 1;
        }

        self.side_to_move = opponent;
        self.hash = Zorbist::DEFAULT.toggle_side(self.hash);

        self
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
        let mut position = Position::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(0), &Square::new(8)));
        assert_eq!(position.pieces[0], Piece::NONE);
        assert_eq!(position.pieces[8], Piece::WHITE_KING);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn king_capture() {
        let mut position = Position::from_fen("8/8/8/8/8/8/p7/K7 w - - 0 1");
        position.make(&Move::new_capture(&Square::new(0), &Square::new(8)));
        assert_eq!(position.pieces[0], Piece::NONE);
        assert_eq!(position.pieces[8], Piece::WHITE_KING);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0100)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn castle_kingside() {
        let mut position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        position.make(&Move::new_castle(
            &Square::new(4),
            &Square::new(6),
            &Castle::KINGSIDE,
        ));
        assert_eq!(position.pieces[4], Piece::NONE);
        assert_eq!(position.pieces[7], Piece::NONE);
        assert_eq!(position.pieces[6], Piece::WHITE_KING);
        assert_eq!(position.pieces[5], Piece::WHITE_ROOK);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0040)
        );
        assert_eq!(
            position.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0000_0000_0000_0021)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_0061)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn castle_queenside() {
        let mut position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        position.make(&Move::new_castle(
            &Square::new(4),
            &Square::new(2),
            &Castle::QUEENSIDE,
        ));
        assert_eq!(position.pieces[4], Piece::NONE);
        assert_eq!(position.pieces[0], Piece::NONE);
        assert_eq!(position.pieces[2], Piece::WHITE_KING);
        assert_eq!(position.pieces[3], Piece::WHITE_ROOK);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KING.to_usize()],
            Board::new(0x0000_0000_0000_0004)
        );
        assert_eq!(
            position.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0000_0000_0000_0088)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0000_008C)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn queen_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/Q7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(62)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[62], Piece::WHITE_QUEEN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn queen_capture() {
        let mut position = Position::from_fen("6p1/8/8/8/8/8/Q7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(62)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[62], Piece::WHITE_QUEEN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn rook_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/R7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(56)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[56], Piece::WHITE_ROOK);
        assert_eq!(
            position.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn rook_capture() {
        let mut position = Position::from_fen("p7/8/8/8/8/8/R7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(56)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[56], Piece::WHITE_ROOK);
        assert_eq!(
            position.piece_boards[Piece::WHITE_ROOK.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn bishop_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/B7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(62)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[62], Piece::WHITE_BISHOP);
        assert_eq!(
            position.piece_boards[Piece::WHITE_BISHOP.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn bishop_capture() {
        let mut position = Position::from_fen("6p1/8/8/8/8/8/B7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(62)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[62], Piece::WHITE_BISHOP);
        assert_eq!(
            position.piece_boards[Piece::WHITE_BISHOP.to_usize()],
            Board::new(0x4000_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x4000_0000_0000_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn knight_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/N7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(25)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[25], Piece::WHITE_KNIGHT);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KNIGHT.to_usize()],
            Board::new(0x0000_0000_0200_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0200_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn knight_capture() {
        let mut position = Position::from_fen("8/8/8/8/1p6/8/N7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(25)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[25], Piece::WHITE_KNIGHT);
        assert_eq!(
            position.piece_boards[Piece::WHITE_KNIGHT.to_usize()],
            Board::new(0x0000_0000_0200_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0200_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_single_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        position.make(&Move::new_push(&Square::new(8), &Square::new(16)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[16], Piece::WHITE_PAWN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0001_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0001_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_double_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/P7/K7 w - - 0 1");
        position.make(&Move::new_push_double_pawn(
            &Square::new(8),
            &Square::new(24),
        ));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[24], Piece::WHITE_PAWN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0100_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0100_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, Some(Square::new(16)));
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_push_promotion() {
        let mut position = Position::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1");
        position.make(&Move::new_push_promotion(
            &Square::new(48),
            &Square::new(56),
            &PieceKind::QUEEN,
        ));
        assert_eq!(position.pieces[48], Piece::NONE);
        assert_eq!(position.pieces[56], Piece::WHITE_QUEEN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(
            position.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x0100_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0100_0000_0000_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_capture() {
        let mut position = Position::from_fen("8/8/8/8/8/1p6/P7/K7 w - - 0 1");
        position.make(&Move::new_capture(&Square::new(8), &Square::new(17)));
        assert_eq!(position.pieces[8], Piece::NONE);
        assert_eq!(position.pieces[17], Piece::WHITE_PAWN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0000_0002_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0000_0002_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_capture_promotion() {
        let mut position = Position::from_fen("1p6/P7/8/8/8/8/8/K7 w - - 0 1");
        position.make(&Move::new_capture_promotion(
            &Square::new(48),
            &Square::new(57),
            &PieceKind::QUEEN,
        ));
        assert_eq!(position.pieces[48], Piece::NONE);
        assert_eq!(position.pieces[57], Piece::WHITE_QUEEN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(
            position.piece_boards[Piece::WHITE_QUEEN.to_usize()],
            Board::new(0x0200_0000_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0200_0000_0000_0001)
        );
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn pawn_capture_en_passant() {
        let mut position = Position::from_fen("8/8/8/Pp6/8/8/8/K7 w - b6 0 1");
        position.make(&&Move::new_capture_en_passant(
            &Square::new(32),
            &Square::new(41),
        ));
        assert_eq!(position.pieces[32], Piece::NONE);
        assert_eq!(position.pieces[40], Piece::NONE);
        assert_eq!(position.pieces[41], Piece::WHITE_PAWN);
        assert_eq!(
            position.piece_boards[Piece::WHITE_PAWN.to_usize()],
            Board::new(0x0000_0200_0000_0000)
        );
        assert_eq!(
            position.side_boards[Side::WHITE.to_usize()],
            Board::new(0x0000_0200_0000_0001)
        );
        assert_eq!(
            position.piece_boards[Piece::BLACK_PAWN.to_usize()],
            Board::EMPTY
        );
        assert_eq!(position.side_boards[Side::BLACK.to_usize()], Board::EMPTY);
        assert_eq!(position.side_to_move, Side::BLACK);
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);
        assert_eq!(position.en_passant_target, None);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn removing_castling_right() {
        // When the king moves
        let mut position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        position.make(&Move::new_push(&Square::new(4), &Square::new(5)));
        assert_eq!(position.castling_rights, CastlingRights::NO_RIGHTS);

        // When the kingside rook moves
        let mut position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        position.make(&Move::new_push(&Square::new(7), &Square::new(6)));
        assert_eq!(position.castling_rights, CastlingRights::WHITE_QUEENSIDE);

        // When the queenside rook moves
        let mut position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        position.make(&Move::new_push(&Square::new(0), &Square::new(1)));
        assert_eq!(position.castling_rights, CastlingRights::WHITE_KINGSIDE);
    }

    #[test]
    fn black_to_move() {
        // When the king moves
        let mut position = Position::from_fen("8/8/8/8/8/8/8/k7 b - - 0 1");
        position.make(&Move::new_push(&Square::new(0), &Square::new(1)));
        assert_eq!(position.side_to_move, Side::WHITE);
        assert_eq!(position.halfmove_clock, 1);
        assert_eq!(position.fullmove_number, 2);
    }
}
