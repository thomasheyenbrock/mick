mod zorbist;

use crate::{
    board::Board,
    castling_rights::CastlingRights,
    piece::{Piece, PieceKind},
    r#move::Move,
    side::Side,
    square::Square,
};

use self::zorbist::Zorbist;

#[derive(Debug, PartialEq)]
pub struct Position {
    piece_boards: [Board; 12],
    side_boards: [Board; 2],
    side_to_move: Side,
    castling_rights: CastlingRights,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
    hash: u64,
}

impl Position {
    pub const STARTING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split(" ").collect();
        assert!(parts.len() == 6, "Invalid fen {fen}");

        let mut piece_boards = [Board::EMPTY; 12];
        for (rank_index, rank) in parts[0].split("/").enumerate() {
            let mut file_index = 0;
            for char in rank.chars() {
                if let Ok(piece) = Piece::try_from_char(char) {
                    let board = &mut piece_boards[piece.to_usize()];
                    let square_index = (7 - rank_index) * 8 + file_index;
                    board.flip_square(&Square::new(square_index as u8));
                    file_index += 1;
                } else if let Some(digit) = char.to_digit(10) {
                    file_index += digit as usize;
                } else {
                    panic!("Invalid fen {fen}")
                }
            }
        }

        let side = Side::from_str(unsafe { parts.get_unchecked(1) });
        let castling_rights = CastlingRights::from_str(unsafe { parts.get_unchecked(2) });
        let en_passant_target = Square::try_from_str(unsafe { parts.get_unchecked(3) }).ok();

        let raw_halfmove_clock = unsafe { parts.get_unchecked(4) };
        let raw_fullmove_number = unsafe { parts.get_unchecked(5) };

        let halfmove_clock = (*raw_halfmove_clock)
            .parse::<u32>()
            .expect(&format!("Invalid halfmove clock {raw_halfmove_clock}"));
        let fullmove_number = (*raw_fullmove_number)
            .parse::<u32>()
            .expect(&format!("Invalid fullmove number {raw_fullmove_number}"));

        Self::new(
            piece_boards,
            side,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        )
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        // TODO: find out the best value for capacity with benchmarks
        let mut legal_moves = Vec::<Move>::with_capacity(60);

        let (attacked, checkers, pinned, pinners) = self.metadata();
        let empty_squares = !(self.side_boards[0] | self.side_boards[1]);

        // Opponent pieces
        let mut capture_mask = self.side_boards[(!self.side_to_move).to_usize()];
        // Empty squares
        let mut push_mask = empty_squares;
        let not_attacked = !attacked;

        // King moves
        let king = self.piece_boards[PieceKind::KING.to_piece(&self.side_to_move).to_usize()];
        let king_square = king.to_square();
        let king_moves = king.king_attacks();
        for (_, to) in (king_moves & capture_mask & not_attacked).iter() {
            legal_moves.push(Move::new_capture(&king_square, &to));
        }
        for (_, to) in (king_moves & push_mask & not_attacked).iter() {
            legal_moves.push(Move::new_capture(&king_square, &to));
        }

        match checkers.occupied() {
            0 => {
                // No checks
            }
            1 => {
                // King is in check once, so other piece moves would need to capture the attacker or block the check
                capture_mask = checkers;

                let checker_square = checkers.to_square();

                let queen =
                    self.piece_boards[PieceKind::QUEEN.to_piece(&!self.side_to_move).to_usize()];
                let rook =
                    self.piece_boards[PieceKind::ROOK.to_piece(&!self.side_to_move).to_usize()];
                let bishop =
                    self.piece_boards[PieceKind::BISHOP.to_piece(&!self.side_to_move).to_usize()];
                let sliders = queen | rook | bishop;

                if sliders & checkers == Board::EMPTY {
                    // King is checked by a knight or pawn which can't be blocked
                    push_mask = Board::EMPTY;
                } else {
                    push_mask = king_square.between(&checker_square);
                }
            }
            _ => {
                // King is in double-check, so the only legal moves are king moves
                return legal_moves;
            }
        }

        // Slider moves
        let queen = self.piece_boards[PieceKind::QUEEN.to_piece(&self.side_to_move).to_usize()];
        let rook = self.piece_boards[PieceKind::ROOK.to_piece(&self.side_to_move).to_usize()];
        let bishop = self.piece_boards[PieceKind::BISHOP.to_piece(&self.side_to_move).to_usize()];

        // Non-pinned straight sliders
        for (from_board, from) in ((queen | rook) & !pinned).iter() {
            let attacks = from_board.straight_attacks(&empty_squares);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(&from, &to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(&from, &to));
            }
        }

        // Pinned straight sliders
        for (from_board, from) in ((queen | rook) & pinned).iter() {
            let attacks =
                from_board.straight_attacks(&empty_squares) & from.lines_along(&king_square);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(&from, &to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(&from, &to));
            }
        }

        // Non-pinned diagonal sliders
        for (from_board, from) in ((queen | bishop) & !pinned).iter() {
            let attacks = from_board.diagonal_attacks(&empty_squares);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(&from, &to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(&from, &to));
            }
        }

        // Pinned diagonal sliders
        for (from_board, from) in ((queen | rook) & pinned).iter() {
            let attacks =
                from_board.diagonal_attacks(&empty_squares) & from.lines_along(&king_square);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(&from, &to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(&from, &to));
            }
        }

        // Knight moves (pinned knights can't move)
        let knight = self.piece_boards[PieceKind::KNIGHT.to_piece(&self.side_to_move).to_usize()];
        for (from_board, from) in (knight & !pinned).iter() {
            let attacks = from_board.knight_attacks();
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(&from, &to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(&from, &to));
            }
        }

        // Pawn moves
        let pawn = self.piece_boards[PieceKind::PAWN.to_piece(&self.side_to_move).to_usize()];
        let (rotate, double_push_rank_index, promotion_rank_index) =
            if self.side_to_move == Side::WHITE {
                (8, 3, 7)
            } else {
                (56, 4, 0)
            };

        // Non-pinned pawns
        for (from_board, from) in (pawn & !pinned).iter() {
            // Single pushes
            let single_push = from_board.rotate_left(rotate) & push_mask;
            let single_push_square = single_push.to_square();
            if single_push_square.rank_index() == promotion_rank_index {
                // TODO: benchmark if this is the fastest way to add multiple items (maybe returning an array is faster?)
                legal_moves.append(&mut Move::new_push_promotion(&from, &single_push_square));
            } else {
                legal_moves.push(Move::new_push(&from, &single_push_square));
            }

            // Double pushes
            let double_push = (single_push.rotate_left(rotate) & push_mask).to_square();
            if double_push.rank_index() == double_push_rank_index {
                legal_moves.push(Move::new_push(&from, &double_push));
            }

            // Captures
            for (_, to) in (from_board.pawn_attacks(&self.side_to_move) & capture_mask).iter() {
                if to.rank_index() == promotion_rank_index {
                    // TODO: benchmark if this is the fastest way to add multiple items (maybe returning an array is faster?)
                    legal_moves.append(&mut Move::new_capture_promotion(&from, &to));
                } else {
                    legal_moves.push(Move::new_capture(&from, &to));
                }
            }
        }

        let pinned_pawns = pawn & pinned;

        // Pinned pawns on the same file as the king can porentially be pushed (but never promoted)
        for (from_board, from) in (pinned_pawns & king_square.file()).iter() {
            let single_push = from_board.rotate_left(rotate) & push_mask;
            legal_moves.push(Move::new_push(&from, &single_push.to_square()));

            let double_push = (single_push.rotate_left(rotate) & push_mask).to_square();
            if double_push.rank_index() == double_push_rank_index {
                legal_moves.push(Move::new_push(&from, &double_push));
            }
        }

        // Pinned pawns on the same diagonal as the king can only porentially capture the pinner
        let diagonals = king_square.diagonal_rays();
        for (from_board, from) in (pinned_pawns & diagonals).iter() {
            for (_, to) in
                (from_board.pawn_attacks(&self.side_to_move) & diagonals & capture_mask).iter()
            {
                legal_moves.push(Move::new_capture(&from, &to))
            }
        }

        legal_moves
    }

    fn metadata(&self) -> (Board, Board, Board, Board) {
        let friendly_king =
            self.piece_boards[PieceKind::KING.to_piece(&self.side_to_move).to_usize()];

        let occupied = self.side_boards[0] | self.side_boards[1];
        let empty_squares = !(self.side_boards[0] | self.side_boards[1]) ^ friendly_king;
        let opponent = !self.side_to_move;

        let king = self.piece_boards[PieceKind::KING.to_piece(&opponent).to_usize()];
        let queen = self.piece_boards[PieceKind::QUEEN.to_piece(&opponent).to_usize()];
        let rook = self.piece_boards[PieceKind::ROOK.to_piece(&opponent).to_usize()];
        let bishop = self.piece_boards[PieceKind::BISHOP.to_piece(&opponent).to_usize()];
        let knight = self.piece_boards[PieceKind::KNIGHT.to_piece(&opponent).to_usize()];
        let pawn = self.piece_boards[PieceKind::PAWN.to_piece(&opponent).to_usize()];

        let straight = queen | rook;
        let diagonal = queen | bishop;

        let attacked = king.king_attacks()
            | straight.straight_attacks(&empty_squares)
            | diagonal.diagonal_attacks(&empty_squares)
            | knight.knight_attacks()
            | pawn.pawn_attacks(&self.side_to_move);

        let king_square = friendly_king.to_square();
        let potential_attackers =
            (straight & king_square.straight_rays()) | (diagonal & king_square.diagonal_rays());

        let mut checkers = Board::EMPTY;
        let mut pinned = Board::EMPTY;
        let mut pinners = Board::EMPTY;

        for (_, square) in potential_attackers.iter() {
            let between = square.between(&king_square);

            if between & self.side_boards[opponent.to_usize()] != Board::EMPTY {
                // There is another opponents piece in between the potential attacker and the king, nothing to do
            } else if between & occupied == Board::EMPTY {
                // No pieces between the attacker and the king
                checkers.flip_square(&square);
            } else {
                let friendly_between = between & self.side_boards[self.side_to_move.to_usize()];
                if friendly_between.occupied() == 1 {
                    // There is exactly one friendly piece between the attacker and the king, so it's pinned
                    pinned.flip_board(&friendly_between);
                    pinners.flip_square(&square);
                }
            }
        }

        // Pawns and knights can only be checkers, no pinners
        checkers |= (friendly_king.knight_attacks() & knight)
            | (friendly_king.pawn_attacks(&opponent) & pawn);

        (attacked, checkers, pinned, pinners)
    }

    pub fn new(
        piece_boards: [Board; 12],
        side_to_move: Side,
        castling_rights: CastlingRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        fullmove_number: u32,
    ) -> Self {
        let hash = Zorbist::DEFAULT.hash(
            &piece_boards,
            &side_to_move,
            &castling_rights,
            &en_passant_target,
        );

        let mut side_boards = [Board::EMPTY; 2];
        for (i, board) in piece_boards.iter().enumerate() {
            side_boards[i % 2].flip_board(board);
        }

        Self {
            piece_boards,
            side_boards,
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, castling_rights::CastlingRights, position::Position, side::Side};

    #[test]
    fn parses_fen_starting_position() {
        let p = Position::from_fen(Position::STARTING);

        assert_eq!(
            p,
            Position {
                piece_boards: [
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
                    Board::new(0x0000_0000_0000_FF00),
                    Board::new(0x00FF_0000_0000_0000),
                ],
                side_boards: [
                    Board::new(0x0000_0000_0000_FFFF),
                    Board::new(0xFFFF_0000_0000_0000)
                ],
                side_to_move: Side::WHITE,
                castling_rights: CastlingRights::ALL_RIGHTS,
                en_passant_target: None,
                halfmove_clock: 0,
                fullmove_number: 1,
                hash: 1278498509228713946
            }
        );
    }
}
