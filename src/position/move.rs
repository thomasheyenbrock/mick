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
        board::Board, castle::CastlingRights, piece::Piece, position::Position, r#move::Move,
        side::Side, square::Square,
    };

    #[test]
    fn single_pawn_push() {
        let mut position = Position::from_fen(Position::STARTING);
        position.make(&Move::new_push(&Square::new(8), &Square::new(16)));
        assert_eq!(
            position,
            Position::new(
                [
                    Piece::WHITE_ROOK,
                    Piece::WHITE_KNIGHT,
                    Piece::WHITE_BISHOP,
                    Piece::WHITE_QUEEN,
                    Piece::WHITE_KING,
                    Piece::WHITE_BISHOP,
                    Piece::WHITE_KNIGHT,
                    Piece::WHITE_ROOK,
                    Piece::NONE,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_ROOK,
                    Piece::BLACK_KNIGHT,
                    Piece::BLACK_BISHOP,
                    Piece::BLACK_QUEEN,
                    Piece::BLACK_KING,
                    Piece::BLACK_BISHOP,
                    Piece::BLACK_KNIGHT,
                    Piece::BLACK_ROOK,
                ],
                [
                    Board::new(0x0000_0000_0000_0010),
                    Board::new(0x1000_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0008),
                    Board::new(0x0800_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0081),
                    Board::new(0x8100_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0024),
                    Board::new(0x2400_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0042),
                    Board::new(0x4200_0000_0000_0000),
                    Board::new(0x0000_0000_0001_FE00),
                    Board::new(0x00FF_0000_0000_0000),
                ],
                Side::BLACK,
                CastlingRights::ALL_RIGHTS,
                None,
                0,
                1
            )
        )
    }

    #[test]
    fn en_passant() {
        let mut position = Position::from_fen("8/8/8/8/1p6/8/P7/K7 w - - 0 1");
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
}
