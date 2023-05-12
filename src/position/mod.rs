mod fen;
mod r#move;

use crate::{
    board::{Board, RANKS},
    castle::{
        Castle, CastlingRights, BLACK_KING_SIDE, BLACK_QUEEN_SIDE, KING_SIDE, QUEEN_SIDE,
        WHITE_KING_SIDE, WHITE_QUEEN_SIDE,
    },
    hash::DEFAULT_ZOBRISH_HASH,
    piece::{Piece, BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
    r#move::Move,
    side::{Side, WHITE},
    square::{Square, C1, C8, G1, G8},
    utils::grid_to_string_with_props,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pieces: [Piece; 64],
    piece_boards: [Board; 12],
    side_boards: [Board; 2],
    state: State,
    hash: u64,
}

impl Position {
    pub fn at(&self, sq: Square) -> Piece {
        unsafe { return *self.pieces.get_unchecked(sq.0 as usize) }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        // TODO: find out the best value for capacity with benchmarks
        let mut legal_moves = Vec::<Move>::with_capacity(60);

        let (attacked, checkers, pinned, pinners) = self.metadata();
        let occupied = self.side_boards[0] | self.side_boards[1];
        let empty_squares = !occupied;

        // Opponent pieces
        let opponent_side = !self.state.side_to_move;
        let mut capture_mask = self.side_boards[opponent_side.0 as usize];
        // Empty squares
        let mut push_mask = empty_squares;
        let not_attacked = !attacked;

        // King moves
        let king = self.piece_boards[KING.to_piece(self.state.side_to_move).0 as usize];
        let king_square = king.to_square();
        let king_moves = king.king_attacks();
        for (_, to) in (king_moves & capture_mask & not_attacked).iter() {
            legal_moves.push(Move::new_capture(king_square, to));
        }
        for (_, to) in (king_moves & push_mask & not_attacked).iter() {
            legal_moves.push(Move::new_capture(king_square, to));
        }

        match checkers.occupied() {
            0 => {
                // No checks
            }
            1 => {
                // King is in check once, so other piece moves would need to capture the attacker or block the check
                capture_mask = checkers;

                let checker_square = checkers.to_square();

                let queen = self.piece_boards[QUEEN.to_piece(!self.state.side_to_move).0 as usize];
                let rook = self.piece_boards[ROOK.to_piece(!self.state.side_to_move).0 as usize];
                let bishop =
                    self.piece_boards[BISHOP.to_piece(!self.state.side_to_move).0 as usize];
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

        // Castles
        for (castle, castling_rights, to, blocking, safe) in
            CASTLE_BY_SIDE[self.state.side_to_move.0 as usize]
        {
            if self.state.castling_rights.0 & castling_rights.0 != 0
                && occupied & blocking == Board::EMPTY
                && safe & attacked == Board::EMPTY
            {
                legal_moves.push(Move::new_castle(king_square, to, castle))
            }
        }

        // Slider moves
        let queen = self.piece_boards[QUEEN.to_piece(self.state.side_to_move).0 as usize];
        let rook = self.piece_boards[ROOK.to_piece(self.state.side_to_move).0 as usize];
        let bishop = self.piece_boards[BISHOP.to_piece(self.state.side_to_move).0 as usize];

        // Non-pinned straight sliders
        for (_, from) in ((queen | rook) & !pinned).iter() {
            let attacks = from.straight_attacks(occupied);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(from, to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(from, to));
            }
        }

        // Pinned straight sliders
        for (_, from) in ((queen | rook) & pinned).iter() {
            let attacks = from.straight_attacks(occupied) & from.lines_along(&king_square);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(from, to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(from, to));
            }
        }

        // Non-pinned diagonal sliders
        for (_, from) in ((queen | bishop) & !pinned).iter() {
            let attacks = from.diagonal_attacks(occupied);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(from, to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(from, to));
            }
        }

        // Pinned diagonal sliders
        for (_, from) in ((queen | bishop) & pinned).iter() {
            let attacks = from.diagonal_attacks(occupied) & from.lines_along(&king_square);
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(from, to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(from, to));
            }
        }

        // Knight moves (pinned knights can't move)
        let knight = self.piece_boards[KNIGHT.to_piece(self.state.side_to_move).0 as usize];
        for (from_board, from) in (knight & !pinned).iter() {
            let attacks = from_board.knight_attacks();
            for (_, to) in (attacks & capture_mask).iter() {
                legal_moves.push(Move::new_capture(from, to));
            }
            for (_, to) in (attacks & push_mask).iter() {
                legal_moves.push(Move::new_push(from, to));
            }
        }

        // Pawn moves
        let pawn = self.piece_boards[PAWN.to_piece(self.state.side_to_move).0 as usize];
        // TODO: make this an array lookup (if it's faster)
        let (rotate, double_push_rank_index, promotion_rank_index) =
            if self.state.side_to_move == WHITE {
                (8, 3, 7)
            } else {
                (56, 4, 0)
            };

        // Non-pinned pawns
        for (from_board, from) in (pawn & !pinned).iter() {
            // Single pushes
            let single_push = from_board.rotate_left(rotate) & empty_squares;
            for (_, to) in (single_push & push_mask).iter() {
                if to.rank_index() == promotion_rank_index {
                    legal_moves.push(Move::new_push_promotion(from, to, QUEEN));
                    legal_moves.push(Move::new_push_promotion(from, to, ROOK));
                    legal_moves.push(Move::new_push_promotion(from, to, BISHOP));
                    legal_moves.push(Move::new_push_promotion(from, to, KNIGHT));
                } else {
                    legal_moves.push(Move::new_push(from, to));
                }
            }

            // Double pushes
            let double_push = single_push.rotate_left(rotate) & push_mask;
            for (_, to) in double_push.iter() {
                if to.rank_index() == double_push_rank_index {
                    legal_moves.push(Move::new_push_double_pawn(from, to));
                }
            }

            // Captures
            for (_, to) in (from_board.pawn_attacks(&self.state.side_to_move) & capture_mask).iter()
            {
                if to.rank_index() == promotion_rank_index {
                    legal_moves.push(Move::new_capture_promotion(from, to, QUEEN));
                    legal_moves.push(Move::new_capture_promotion(from, to, ROOK));
                    legal_moves.push(Move::new_capture_promotion(from, to, BISHOP));
                    legal_moves.push(Move::new_capture_promotion(from, to, KNIGHT));
                } else {
                    legal_moves.push(Move::new_capture(from, to));
                }
            }
        }

        let pinned_pawns = pawn & pinned;

        // Pinned pawns on the same file as the king can porentially be pushed (but never promoted)
        for (from_board, from) in (pinned_pawns & king_square.file()).iter() {
            let single_push = from_board.rotate_left(rotate) & push_mask;
            for (_, to) in single_push.iter() {
                legal_moves.push(Move::new_push(from, to));
            }

            let double_push = single_push.rotate_left(rotate) & push_mask;
            for (_, to) in double_push.iter() {
                if to.rank_index() == double_push_rank_index {
                    legal_moves.push(Move::new_push_double_pawn(from, to));
                }
            }
        }

        // Pinned pawns on the same diagonal as the king can only porentially capture the pinner
        let diagonals = king_square.diagonal_rays();
        for (from_board, from) in (pinned_pawns & diagonals).iter() {
            for (_, to) in
                (from_board.pawn_attacks(&self.state.side_to_move) & diagonals & capture_mask)
                    .iter()
            {
                legal_moves.push(Move::new_capture(from, to))
            }
        }

        // En-passant captures
        if let Some(en_passant_target) = self.state.en_passant_target {
            let capturers = pawn & en_passant_target.to_board().pawn_attacks(&opponent_side);

            for (_, from) in capturers.iter() {
                // Capturing is only possible when moving to a square in `push_mask`
                // or capturing a piece on `capture_mask`
                let capture_square = if self.state.side_to_move == WHITE {
                    Square(en_passant_target.0 - 8)
                } else {
                    Square(en_passant_target.0 + 8)
                };
                if push_mask.has(&en_passant_target) || capture_mask.has(&capture_square) {
                    let is_discovered_check = {
                        let rank = RANKS[capture_square.0 as usize];
                        if rank & king == Board::EMPTY {
                            false
                        } else {
                            let opponent = !self.state.side_to_move;
                            let queen = self.piece_boards[QUEEN.to_piece(opponent).0 as usize];
                            let rook = self.piece_boards[ROOK.to_piece(opponent).0 as usize];
                            let potential_attackers = queen | rook;

                            let mut occupied = occupied;
                            occupied.flip_square(&from);
                            occupied.flip_square(&capture_square);

                            let mut is_discovered_check = false;
                            for (_, attacker) in potential_attackers.iter() {
                                if king_square.between(&attacker) & occupied == Board::EMPTY {
                                    is_discovered_check = true;
                                    break;
                                }
                            }

                            is_discovered_check
                        }
                    };
                    if !is_discovered_check {
                        legal_moves.push(Move::new_capture_en_passant(from, en_passant_target));
                    }
                }
            }
        }

        legal_moves
    }

    fn metadata(&self) -> (Board, Board, Board, Board) {
        let friendly_king = self.piece_boards[KING.to_piece(self.state.side_to_move).0 as usize];

        let occupied = self.side_boards[0] | self.side_boards[1];
        let occupied_without_king = occupied ^ friendly_king;
        let opponent = !self.state.side_to_move;

        let king = self.piece_boards[KING.to_piece(opponent).0 as usize];
        let queen = self.piece_boards[QUEEN.to_piece(opponent).0 as usize];
        let rook = self.piece_boards[ROOK.to_piece(opponent).0 as usize];
        let bishop = self.piece_boards[BISHOP.to_piece(opponent).0 as usize];
        let knight = self.piece_boards[KNIGHT.to_piece(opponent).0 as usize];
        let pawn = self.piece_boards[PAWN.to_piece(opponent).0 as usize];

        let straight = queen | rook;
        let diagonal = queen | bishop;

        let attacked = king.king_attacks()
            | straight.straight_attacks(occupied_without_king)
            | diagonal.diagonal_attacks(occupied_without_king)
            | knight.knight_attacks()
            | pawn.pawn_attacks(&opponent);

        let king_square = friendly_king.to_square();
        let potential_attackers =
            (straight & king_square.straight_rays()) | (diagonal & king_square.diagonal_rays());

        let mut checkers = Board::EMPTY;
        let mut pinned = Board::EMPTY;
        let mut pinners = Board::EMPTY;

        for (_, square) in potential_attackers.iter() {
            let between = square.between(&king_square);

            if between & self.side_boards[opponent.0 as usize] != Board::EMPTY {
                // There is another opponents piece in between the potential attacker and the king, nothing to do
            } else if between & occupied == Board::EMPTY {
                // No pieces between the attacker and the king
                checkers.flip_square(&square);
            } else {
                let friendly_between =
                    between & self.side_boards[self.state.side_to_move.0 as usize];
                if friendly_between.occupied() == 1 {
                    // There is exactly one friendly piece between the attacker and the king, so it's pinned
                    pinned.flip_board(&friendly_between);
                    pinners.flip_square(&square);
                }
            }
        }

        // Pawns and knights can only be checkers, no pinners
        checkers |= (friendly_king.knight_attacks() & knight)
            | (friendly_king.pawn_attacks(&self.state.side_to_move) & pawn);

        (attacked, checkers, pinned, pinners)
    }

    pub fn new(
        pieces: [Piece; 64],
        piece_boards: [Board; 12],
        side_to_move: Side,
        castling_rights: CastlingRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        fullmove_number: u32,
    ) -> Self {
        let mut side_boards = [Board::EMPTY; 2];
        for (i, board) in piece_boards.iter().enumerate() {
            side_boards[i % 2].flip_board(board);
        }

        let state = State {
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        };

        let hash = DEFAULT_ZOBRISH_HASH.position(&pieces, &state);

        Self {
            pieces,
            piece_boards,
            side_boards,
            state,
            hash,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let props = vec![
            ("    side to move", format!("{}", self.state.side_to_move)),
            (
                " castling rights",
                format!("{}", self.state.castling_rights),
            ),
            (
                "      en-passant",
                self.state
                    .en_passant_target
                    .map_or("-".to_string(), |s| s.to_string()),
            ),
            (" half-move clock", self.state.halfmove_clock.to_string()),
            ("full-move number", self.state.fullmove_number.to_string()),
            ("             FEN", self.to_fen()),
            ("            hash", format!("{:016X}", self.hash)),
        ];
        let s = grid_to_string_with_props(
            |s: Square| -> char {
                let piece = self.pieces[s.0 as usize];
                if piece.is_some() {
                    piece.to_char()
                } else {
                    ' '
                }
            },
            props.as_slice(),
        );

        write!(f, "{}", &s)
    }
}

pub static CASTLE_BY_SIDE: [[(Castle, CastlingRights, Square, Board, Board); 2]; 2] = [
    [
        (
            KING_SIDE,
            WHITE_KING_SIDE,
            G1,
            Board::WHITE_KINGSIDE_BLOCKING,
            Board::WHITE_KINGSIDE_SAFE,
        ),
        (
            QUEEN_SIDE,
            WHITE_QUEEN_SIDE,
            C1,
            Board::WHITE_QUEENSIDE_BLOCKING,
            Board::WHITE_QUEENSIDE_SAFE,
        ),
    ],
    [
        (
            KING_SIDE,
            BLACK_KING_SIDE,
            G8,
            Board::BLACK_KINGSIDE_BLOCKING,
            Board::BLACK_KINGSIDE_SAFE,
        ),
        (
            QUEEN_SIDE,
            BLACK_QUEEN_SIDE,
            C8,
            Board::BLACK_QUEENSIDE_BLOCKING,
            Board::BLACK_QUEENSIDE_SAFE,
        ),
    ],
];

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn king_moves() {
        // In the corner
        let position = Position::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3);

        // At the edge of the board
        let position = Position::from_fen("8/8/8/8/8/8/8/1K6 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 5);

        // Else
        let position = Position::from_fen("8/8/8/8/8/8/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 8);

        // Castling
        let position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        assert_eq!(position.legal_moves().len(), 5 + 19 + 2);
    }

    #[test]
    fn not_moving_king_in_check() {
        // Attacks by queens
        let position = Position::from_fen("8/8/8/8/8/3q4/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3);

        // Attacks by rooks
        let position = Position::from_fen("8/8/8/8/8/3r4/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 5);

        // Attacks by bishops
        let position = Position::from_fen("8/8/8/8/2b5/8/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 6);

        // Attacks by knights
        let position = Position::from_fen("8/8/8/8/3n4/8/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 6);

        // Attacks by pawns
        let position = Position::from_fen("8/8/8/8/p7/8/1K6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 7);
    }

    #[test]
    fn queen_moves() {
        // In the corner
        let position = Position::from_fen("8/8/8/8/8/8/7K/Q7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 21 + 5);

        // At the edge of the board
        let position = Position::from_fen("8/8/8/8/8/8/7K/1Q6 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 21 + 5);

        // In the center of the board
        let position = Position::from_fen("8/8/8/7K/3Q4/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 27 + 5);
    }

    #[test]
    fn rook_moves() {
        // In the corner
        let position = Position::from_fen("8/8/8/8/8/8/7K/R7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 14 + 5);

        // At the edge of the board
        let position = Position::from_fen("8/8/8/8/8/8/7K/1R6 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 14 + 5);

        // Else
        let position = Position::from_fen("8/8/8/7K/3R4/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 14 + 5);
    }

    #[test]
    fn bishop_moves() {
        // In the corner
        let position = Position::from_fen("8/8/8/8/8/8/7K/B7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 7 + 5);

        // At the edge of the board
        let position = Position::from_fen("8/8/8/8/8/8/7K/1B6 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 7 + 5);

        // Else
        let position = Position::from_fen("8/8/8/7K/3B4/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 13 + 5);
    }

    #[test]
    fn knight_moves() {
        // In the corner
        let position = Position::from_fen("7K/8/8/8/8/8/8/N7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 3);

        // One square from a corner
        let position = Position::from_fen("7K/8/8/8/8/8/8/1N6 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 3);

        // Two squares from a corner
        let position = Position::from_fen("7K/8/8/8/8/8/1N6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 4 + 3);

        // Three squares from a corner
        let position = Position::from_fen("7K/8/8/8/8/1N6/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 6 + 3);

        // In the center
        let position = Position::from_fen("7K/8/8/8/3K4/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 8 + 3);
    }

    #[test]
    fn pawn_moves() {
        // Single push
        let position = Position::from_fen("7K/8/8/8/8/1P6/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 1 + 3);

        // Single push with captures
        let position = Position::from_fen("7K/8/8/8/p1p5/1P6/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 3);

        // Double push
        let position = Position::from_fen("7K/8/8/8/8/8/1P6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 3);

        // Double push with captures
        let position = Position::from_fen("7K/8/8/8/8/p1p5/1P6/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 4 + 3);

        // Promotion
        let position = Position::from_fen("7K/1P6/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 4 + 3);

        // Promotion with captures
        let position = Position::from_fen("p1p4K/1P6/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 12 + 3);

        // En-passant capture
        let position = Position::from_fen("7K/8/8/pP6/8/8/8/8 w - a6 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 3);
    }

    #[test]
    fn double_check() {
        let position = Position::from_fen("4q3/8/b7/8/8/7R/3PK3/5N2 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3);
    }

    #[test]
    fn single_check() {
        // Moving the king
        let position = Position::from_fen("q7/8/8/8/8/8/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2);

        // Capturing the checker
        let position = Position::from_fen("qR6/8/8/8/8/8/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 1);

        // Blocking the check
        let position = Position::from_fen("q7/1R6/8/8/8/8/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 1);
    }

    #[test]
    fn pinned_straight_sliders() {
        // Pinned by diagonal slider
        let position = Position::from_fen("7q/8/8/8/8/2R5/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 0);

        // Pinned by straight slider
        let position = Position::from_fen("q7/8/8/8/8/R7/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 6);
    }

    #[test]
    fn pinned_diagonal_sliders() {
        let position = Position::from_fen("7q/8/8/8/8/2B5/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 6);

        // Diagonal slider pinned by straight slider
        let position = Position::from_fen("q7/8/8/8/8/B7/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 0);
    }

    #[test]
    fn pinned_knights() {
        let position = Position::from_fen("q7/8/8/8/8/N7/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 0);
    }

    #[test]
    fn pinned_pawns() {
        // Pinned in the direction of movement
        let position = Position::from_fen("q7/8/8/8/8/P7/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 1);

        // With double-push pinned in the direction of movement
        let position = Position::from_fen("q7/8/8/8/8/8/P7/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 2 + 2);

        // Pinned not in the direction of movement
        let position = Position::from_fen("7q/8/8/8/1p6/2P5/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 0);

        // Pinned not in the direction of movement that can capture the attacker
        let position = Position::from_fen("8/8/8/8/1p1q4/2P5/8/K7 w - - 0 1");
        assert_eq!(position.legal_moves().len(), 3 + 1);
    }

    #[test]
    fn en_passant_discovered_check() {
        let position = Position::from_fen("8/8/8/K2Pp2q/8/8/8/8 w - e6 0 1");
        assert_eq!(position.legal_moves().len(), 5 + 1);
    }

    #[test]
    fn en_passant_when_in_check() {
        // Capturing the checker not possible
        let position = Position::from_fen("8/K6q/8/3Pp3/8/8/8/8 w - e6 0 1");
        assert_eq!(position.legal_moves().len(), 4 + 0);

        // Capturing the checker possible
        let position = Position::from_fen("8/8/8/3Pp3/5K2/8/8/8 w - e6 0 1");
        assert_eq!(position.legal_moves().len(), 8 + 1);
    }
}
