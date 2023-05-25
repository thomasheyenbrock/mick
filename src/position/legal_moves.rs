use crate::{
    board::{Board, EMPTY, NOT_FILE_A, NOT_FILE_H, RANK_4, RANK_5},
    castle::{KING_SIDE, QUEEN_SIDE},
    move_list::{move_vec::MoveVec, MoveAdder},
    piece::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
    side::{Side, WHITE},
    square::{Square, C1, C8, E1, E8, G1, G8},
    Position,
};

impl Position {
    pub fn legal_moves<L: MoveAdder>(&self, list: &mut L) -> bool {
        let side_to_move = self.state.side_to_move;
        let kings = self.piece(KING.to_piece(side_to_move));

        let attacker = !side_to_move;
        let occupied = self.occupied();
        let king_sq = kings.to_square();

        let opponent_kings = self.piece(KING.to_piece(attacker));
        let opponent_queens = self.piece(QUEEN.to_piece(attacker));
        let opponent_rooks = self.piece(ROOK.to_piece(attacker));
        let opponent_bishops = self.piece(BISHOP.to_piece(attacker));
        let opponent_knights = self.piece(KNIGHT.to_piece(attacker));
        let opponent_pawns = self.piece(PAWN.to_piece(attacker));

        let straight_attackers = opponent_queens | opponent_rooks;
        let diagonal_attackers = opponent_queens | opponent_bishops;

        let mut checkers = EMPTY;
        let mut pinned = EMPTY;
        let mut pinners = EMPTY;

        // Pawns and Knights can only be checkers, not pinners
        checkers |= king_sq.knight_moves() & opponent_knights;

        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[side_to_move.0 as usize].iter() {
            checkers |= kings.rotate_left(shift as u32) & file_mask & opponent_pawns;
        }

        // Sliding pieces can be checkers or pinners depending on occupancy of intermediate squares
        let potential_king_attackers = occupied
            & ((king_sq.diagonal_rays() & diagonal_attackers)
                | (king_sq.straight_rays() & straight_attackers));

        for (sq, bb) in potential_king_attackers.iter() {
            let potentially_pinned = king_sq.between(sq) & occupied;

            // If there are no friendly pieces between the attacker and the king
            // then the attacker is giving check
            if potentially_pinned == EMPTY {
                checkers |= bb;
            // If there is a friendly piece between the attacker and the king
            // then it is pinned
            } else if potentially_pinned.occupied() == 1 {
                pinned |= potentially_pinned;
                pinners |= bb;
            }
        }

        // We always need legal king moves
        let occupied_without_king = occupied & !kings;

        let mut attacked = opponent_kings.to_square().king_moves()
            | straight_attackers.straight_attacks(occupied_without_king)
            | diagonal_attackers.diagonal_attacks(occupied_without_king)
            | opponent_knights.knight_attacks();

        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[attacker.0 as usize].iter() {
            let targets = opponent_pawns.rotate_left(shift as u32) & file_mask;
            attacked |= targets;
        }

        let king_attacks_count = checkers.occupied();

        // capture_mask and push_mask represent squares our pieces are allowed to move to or capture,
        // respectively. The difference between the two is only important for pawn EP captures
        // Since push_mask is used to block a pin, we ignore push_mask when calculating king moves
        let enemy = self.side(attacker);
        let empty_squares = self.empty();

        let mut capture_mask = enemy;
        let king_capture_mask = enemy & !attacked;
        let mut push_mask = empty_squares;
        let king_push_mask = empty_squares & !attacked;

        match king_attacks_count {
            0 => (),
            1 => {
                // if ony one attacker, we can try attacking the attacker with
                // our other pieces.
                capture_mask = checkers;
                let checker_sq = checkers.to_square();
                let checker = self.at(checker_sq);
                if checker.is_slider() {
                    // If the piece giving check is a slider, we can additionally attempt
                    // to block the sliding piece;
                    push_mask = king_sq.between(checker_sq);
                } else {
                    // If we are in check by a jumping piece (aka a knight) then
                    // there are no valid non-captures to avoid check
                    push_mask = EMPTY;
                }
            }
            _ => {
                // multiple attackers... only solutions are king moves
                self.king_moves(king_capture_mask, king_push_mask, list);
                return true;
            }
        }

        // generate moves for pinned and unpinned sliders
        self.slider_moves(capture_mask, push_mask, pinned, king_sq, list);

        // generate moves for non-pinned knights (pinned knights can't move)
        self.knight_moves(capture_mask, push_mask, !pinned, list);

        // generate moves for unpinned pawns
        self.pawn_pushes(push_mask, !pinned, list);
        self.pawn_captures(capture_mask, push_mask, !pinned, list);

        // generate moves for pinned pawns
        // pinned pawn captures can only include pinners
        self.pawn_pin_ray_moves(
            capture_mask & pinners,
            push_mask,
            king_sq,
            pinned,
            side_to_move,
            list,
        );

        if king_attacks_count == 0 {
            // Not in check so can generate castles
            // impossible for castles to be affected by pins
            // so we don't need to consider pins here
            self.castles(attacked, list);
        }

        self.king_moves(king_capture_mask, king_push_mask, list);

        king_attacks_count > 0
    }

    pub fn legal_moves_vec(&self) -> (MoveVec, bool) {
        let mut list = MoveVec::new();
        let is_in_check = self.legal_moves(&mut list);
        (list, is_in_check)
    }

    fn castles<L: MoveAdder>(&self, attacked: Board, list: &mut L) {
        let side_to_move = self.state.side_to_move;
        let rights = self.state.castling_rights;
        let occupied_squares = self.occupied();

        for castle in [KING_SIDE, QUEEN_SIDE]
            .iter()
            .filter(|c| rights.has(**c, side_to_move))
        {
            // NOTE: should not need to check king and rook pos since
            // should not be able to castle once these are moved

            let blockers = CASTLE_BLOCKING_SQUARES[side_to_move.0 as usize][castle.0 as usize];
            let king_safe = KING_SAFE_SQUARES[side_to_move.0 as usize][castle.0 as usize];
            let (from, to) = FROM_TO_SQUARES[side_to_move.0 as usize][castle.0 as usize];

            if (occupied_squares & blockers).any() | (attacked & king_safe).any() {
                continue;
            }

            list.add_castle(from, to, *castle);
        }
    }

    fn en_passant_move_discovers_check(&self, from: Board, to: Board, side: Side) -> bool {
        let occupied = self.occupied() ^ from ^ to;
        let attacker = !side;
        let queens = self.piece(QUEEN.to_piece(attacker));
        let rooks = self.piece(ROOK.to_piece(attacker));
        let straight_attackers = queens | rooks;

        let king_sq = self.piece(KING.to_piece(side)).to_square();

        king_sq.straight_attacks(occupied) & straight_attackers != EMPTY
    }

    fn king_moves<L: MoveAdder>(&self, capture_mask: Board, push_mask: Board, list: &mut L) {
        let side_to_move = self.state.side_to_move;
        let piece = KING.to_piece(side_to_move);
        let movers = self.piece(piece);

        let from = movers.to_square();

        let capture_targets = from.king_moves() & capture_mask;
        let push_targets = from.king_moves() & push_mask;

        list.add_captures(from, capture_targets);
        list.add_pushes(from, push_targets);
    }

    fn knight_moves<L: MoveAdder>(
        &self,
        capture_mask: Board,
        push_mask: Board,
        from_mask: Board,
        list: &mut L,
    ) {
        let side_to_move = self.state.side_to_move;
        let piece = KNIGHT.to_piece(side_to_move);
        let movers = self.piece(piece) & from_mask;

        for (from, _) in movers.iter() {
            let capture_targets = from.knight_moves() & capture_mask;
            let push_targets = from.knight_moves() & push_mask;

            list.add_captures(from, capture_targets);
            list.add_pushes(from, push_targets);
        }
    }

    fn pawn_captures<L: MoveAdder>(
        &self,
        capture_mask: Board,
        push_mask: Board,
        from_mask: Board,
        list: &mut L,
    ) {
        let side_to_move = self.state.side_to_move;
        let piece = PAWN.to_piece(side_to_move);
        let movers = self.piece(piece) & from_mask;

        if movers == EMPTY {
            return;
        }

        if capture_mask != EMPTY {
            // CAPTURES
            for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[side_to_move.0 as usize].iter() {
                let targets = movers.rotate_left(shift as u32) & file_mask & capture_mask;
                list.add_pawn_captures(shift, targets);
            }
        }

        if self.state.en_passant_target.is_some() {
            let ep = self.state.en_passant_target.unwrap();
            // This is rare so worth duplicating work here to avoid doing it above
            for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[side_to_move.0 as usize].iter() {
                // EN-PASSANT CAPTURES
                let targets = movers.rotate_left(shift as u32) & file_mask;
                let ep_captures = targets & Board::new(ep);
                for (to, to_bb) in ep_captures.iter() {
                    let from = to.rotate_right(shift);

                    let capture_sq = from.along_row_with_col(to);
                    let capture_sq_bb = Board::new(capture_sq);

                    // can only make ep capture if moving to push_mask, or capturing on capture mask
                    if ((to_bb & push_mask) | (capture_sq_bb & capture_mask)).any() {
                        // here we need to ensure that there is no discovered check
                        let from_bb = to_bb.rotate_right(shift as u32);
                        // This is expensive but very infrequent
                        if !self.en_passant_move_discovers_check(
                            from_bb,
                            capture_sq_bb,
                            side_to_move,
                        ) {
                            list.add_pawn_ep_capture(from_bb.to_square(), ep);
                        }
                    }
                }
            }
        }
    }

    fn pawn_pin_ray_moves<L: MoveAdder>(
        &self,
        capture_mask: Board,
        push_mask: Board,
        king_sq: Square,
        pinned: Board,
        side_to_move: Side,
        list: &mut L,
    ) {
        let piece = PAWN.to_piece(side_to_move);
        let movers = self.piece(piece) & pinned;

        // exit early if no pinned pawns
        if movers == EMPTY {
            return;
        }

        let push_shift = if side_to_move == WHITE { 8 } else { 64 - 8 };
        let double_push_mask = push_mask
            & if side_to_move == WHITE {
                RANK_4
            } else {
                RANK_5
            };

        let can_push = movers & king_sq.file_mask();
        let king_diags = king_sq.diagonal_rays();
        let can_capture = movers & king_diags;

        // For pinned pawns, only possible moves are those along the king file
        for (_, pawn) in can_push.iter() {
            let single_pushes = pawn.rotate_left(push_shift as u32) & push_mask;
            list.add_pawn_pushes(push_shift, single_pushes);
            let double_pushes = single_pushes.rotate_left(push_shift as u32) & double_push_mask;
            let double_push_shift = (push_shift * 2) % 64;
            list.add_double_pawn_pushes(double_push_shift, double_pushes);
        }

        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[side_to_move.0 as usize].iter() {
            let targets =
                can_capture.rotate_left(shift as u32) & file_mask & capture_mask & king_diags;

            list.add_pawn_captures(shift, targets);
        }

        if self.state.en_passant_target.is_some() {
            for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[side_to_move.0 as usize].iter() {
                let targets = can_capture.rotate_left(shift as u32) & file_mask;

                let ep = self.state.en_passant_target.unwrap();
                let ep_captures = targets & Board::new(ep) & king_diags;

                for (to, to_bb) in ep_captures.iter() {
                    let from = to.rotate_right(shift);

                    let capture_sq = from.along_row_with_col(to);
                    let capture_sq_bb = Board::new(capture_sq);

                    // can only make ep capture if moving along king_diags, or capturing on capture mask
                    if ((to_bb & king_diags) | (capture_sq_bb & capture_mask)).any() {
                        let from_bb = to_bb.rotate_right(shift as u32);
                        list.add_pawn_ep_capture(from_bb.to_square(), ep);
                    }
                }
            }
        }
    }

    fn pawn_pushes<L: MoveAdder>(&self, to_mask: Board, from_mask: Board, list: &mut L) {
        let side_to_move = self.state.side_to_move;
        let piece = PAWN.to_piece(side_to_move);
        let movers = self.piece(piece) & from_mask;

        if movers == EMPTY {
            return;
        }

        let shift = if side_to_move == WHITE { 8 } else { 64 - 8 };
        let empty_squares = self.empty();

        // Dont apply to_mask here to avoid masking double pushes
        let single_pushes = movers.rotate_left(shift as u32) & empty_squares;

        list.add_pawn_pushes(shift, single_pushes & to_mask);

        let double_push_mask = if side_to_move == WHITE {
            RANK_4
        } else {
            RANK_5
        } & empty_squares
            & to_mask;
        let double_pushes = single_pushes.rotate_left(shift as u32) & double_push_mask;

        // DOUBLE PUSHES
        let double_push_shift = (shift * 2) % 64;
        list.add_double_pawn_pushes(double_push_shift, double_pushes & to_mask);
    }

    fn slider_moves<L: MoveAdder>(
        &self,
        capture_mask: Board,
        push_mask: Board,
        pinned_mask: Board,
        king_sq: Square,
        list: &mut L,
    ) {
        let side_to_move = self.state.side_to_move;
        let occupied = self.occupied();
        let queens = self.piece(QUEEN.to_piece(side_to_move));
        let rooks = self.piece(ROOK.to_piece(side_to_move));
        let bishops = self.piece(BISHOP.to_piece(side_to_move));
        let diagonal_attackers = queens | bishops;
        let straight_attackers = queens | rooks;

        for (from, _) in (straight_attackers & !pinned_mask).iter() {
            let targets = from.straight_attacks(occupied);
            list.add_captures(from, targets & capture_mask);
            list.add_pushes(from, targets & push_mask);
        }

        for (from, _) in (straight_attackers & pinned_mask).iter() {
            let ray_mask = from.lines_along(king_sq);
            let targets = from.straight_attacks(occupied) & ray_mask;
            list.add_captures(from, targets & capture_mask);
            list.add_pushes(from, targets & push_mask);
        }

        for (from, _) in (diagonal_attackers & !pinned_mask).iter() {
            let targets = from.diagonal_attacks(occupied);
            list.add_captures(from, targets & capture_mask);
            list.add_pushes(from, targets & push_mask);
        }

        for (from, _) in (diagonal_attackers & pinned_mask).iter() {
            let ray_mask = from.lines_along(king_sq);
            let targets = from.diagonal_attacks(occupied) & ray_mask;
            list.add_captures(from, targets & capture_mask);
            list.add_pushes(from, targets & push_mask);
        }
    }
}

const CASTLE_BLOCKING_SQUARES: [[Board; 2]; 2] = [
    [
        Board((1u64 << 5) + (1u64 << 6)),               // WHITE KS = F1 + G1
        Board((1u64 << 1) + (1u64 << 2) + (1u64 << 3)), // WHITE QS = B1 + C1 + D1
    ],
    [
        Board((1u64 << 61) + (1u64 << 62)), // BLACK KS = F8 + G8
        Board((1u64 << 57) + (1u64 << 58) + (1u64 << 59)), // BLACK QS = B8 + C8 + D1
    ],
];

// squares that must be not attacked for a castle to take place
const KING_SAFE_SQUARES: [[Board; 2]; 2] = [
    [
        Board((1u64 << 4) + (1u64 << 5) + (1u64 << 6)), // WHITE KS = E1 + F1 + G1
        Board((1u64 << 2) + (1u64 << 3) + (1u64 << 4)), // WHITE QS = C1 + D1 + E1
    ],
    [
        Board((1u64 << 60) + (1u64 << 61) + (1u64 << 62)), // BLACK KS = E8 + F8 + G8
        Board((1u64 << 58) + (1u64 << 59) + (1u64 << 60)), // BLACK QS = C8 + D8  + E8
    ],
];

const FROM_TO_SQUARES: [[(Square, Square); 2]; 2] = [
    [
        (E1, G1), // WHITE KS
        (E1, C1), // WHITE QS
    ],
    [
        (E8, G8), // BLACK KS
        (E8, C8), // BLACK QS
    ],
];

// white, left  = +7 remove FILE_H
// white, right = +9 remove FILE_A
// black, left  = -9 remove FILE_H
// black, right = -7 remove FILE_A
// maps: side -> capture-direction -> shift amount + overflow mask
const PAWN_CAPTURE_FILE_MASKS: [[(u8, Board); 2]; 2] = [
    [(7, NOT_FILE_H), (9, NOT_FILE_A)],
    [(64 - 9, NOT_FILE_H), (64 - 7, NOT_FILE_A)],
];

#[cfg(test)]
mod tests {
    use crate::{move_list::move_counter::MoveCounter, position::Position};

    #[test]
    fn king_moves() {
        // In the corner
        let position = Position::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3);

        // At the edge of the board
        let position = Position::from_fen("7k/8/8/8/8/8/8/1K6 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 5);

        // Else
        let position = Position::from_fen("7k/8/8/8/8/8/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 8);

        // Castling
        let position = Position::from_fen("6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 5 + 19 + 2);
    }

    #[test]
    fn not_moving_king_in_check() {
        // Attacks by queens
        let position = Position::from_fen("7k/8/8/8/8/3q4/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3);

        // Attacks by rooks
        let position = Position::from_fen("7k/8/8/8/8/3r4/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 5);

        // Attacks by bishops
        let position = Position::from_fen("7k/8/8/8/2b5/8/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 6);

        // Attacks by knights
        let position = Position::from_fen("7k/8/8/8/3n4/8/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 6);

        // Attacks by pawns
        let position = Position::from_fen("7k/8/8/8/p7/8/1K6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 7);
    }

    #[test]
    fn queen_moves() {
        // In the corner
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/Q7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 21 + 5);

        // At the edge of the board
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/1Q6 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 21 + 5);

        // In the center of the board
        let position = Position::from_fen("6k1/8/8/7K/3Q4/8/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 27 + 5);
    }

    #[test]
    fn rook_moves() {
        // In the corner
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/R7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 14 + 5);

        // At the edge of the board
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/1R6 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 14 + 5);

        // Else
        let position = Position::from_fen("6k1/8/8/7K/3R4/8/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 14 + 5);
    }

    #[test]
    fn bishop_moves() {
        // In the corner
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/B7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 7 + 5);

        // At the edge of the board
        let position = Position::from_fen("6k1/8/8/8/8/8/7K/1B6 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 7 + 5);

        // Else
        let position = Position::from_fen("6k1/8/8/7K/3B4/8/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 13 + 5);
    }

    #[test]
    fn knight_moves() {
        // In the corner
        let position = Position::from_fen("k6K/8/8/8/8/8/8/N7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 3);

        // One square from a corner
        let position = Position::from_fen("k6K/8/8/8/8/8/8/1N6 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 3);

        // Two squares from a corner
        let position = Position::from_fen("k6K/8/8/8/8/8/1N6/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 4 + 3);

        // Three squares from a corner
        let position = Position::from_fen("k6K/8/8/8/8/1N6/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 6 + 3);

        // In the center
        let position = Position::from_fen("k6K/8/8/8/3N4/8/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 8 + 3);
    }

    #[test]
    fn pawn_moves() {
        // Single push
        let position = Position::from_fen("k6K/8/8/8/8/1P6/8/8 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 1 + 3);

        // Single push with captures
        let position = Position::from_fen("8/8/8/8/p1p5/1P6/8/k6K w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 3);

        // Double push
        let position = Position::from_fen("8/8/8/8/8/8/1P6/k6K w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 3);

        // Double push with captures
        let position = Position::from_fen("8/8/8/8/8/p1p5/1P6/k6K w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 4 + 3);

        // Promotion
        let position = Position::from_fen("8/1P6/8/8/8/8/8/k6K w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 4 + 3);

        // Promotion with captures
        let position = Position::from_fen("p1p5/1P6/8/8/8/8/8/k6K w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 12 + 3);

        // En-passant capture
        let position = Position::from_fen("8/8/8/pP6/8/8/8/k6K w - a6 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 3);
    }

    #[test]
    fn double_check() {
        let position = Position::from_fen("k3q3/8/b7/8/8/7R/3PK3/5N2 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3);
    }

    #[test]
    fn single_check() {
        // Moving the king
        let position = Position::from_fen("q6k/8/8/8/8/8/8/K7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2);

        // Capturing the checker
        let position = Position::from_fen("qR5k/8/8/8/8/8/8/K7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 1);

        // Blocking the check
        let position = Position::from_fen("q6k/1R6/8/8/8/8/8/K7 w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 1);
    }

    #[test]
    fn pinned_straight_sliders() {
        // Pinned by diagonal slider
        let position = Position::from_fen("7q/8/8/8/8/2R5/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 0);

        // Pinned by straight slider
        let position = Position::from_fen("q7/8/8/8/8/R7/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 6);
    }

    #[test]
    fn pinned_diagonal_sliders() {
        let position = Position::from_fen("7q/8/8/8/8/2B5/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 6);

        // Diagonal slider pinned by straight slider
        let position = Position::from_fen("q7/8/8/8/8/B7/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 0);
    }

    #[test]
    fn pinned_knights() {
        let position = Position::from_fen("q7/8/8/8/8/N7/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 0);
    }

    #[test]
    fn pinned_pawns() {
        // Pinned in the direction of movement
        let position = Position::from_fen("q7/8/8/8/8/P7/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 1);

        // With double-push pinned in the direction of movement
        let position = Position::from_fen("q7/8/8/8/8/8/P7/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 2 + 2);

        // Pinned not in the direction of movement
        let position = Position::from_fen("7q/8/8/8/1p6/2P5/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 0);

        // Pinned not in the direction of movement that can capture the attacker
        let position = Position::from_fen("8/8/8/8/1p1q4/2P5/8/K6k w - - 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 3 + 1);
    }

    #[test]
    fn en_passant_discovered_check() {
        let position = Position::from_fen("7k/8/8/K2Pp2q/8/8/8/8 w - e6 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 5 + 1);
    }

    #[test]
    fn en_passant_when_in_check() {
        // Capturing the checker not possible
        let position = Position::from_fen("8/K6q/8/3Pp3/8/8/8/7k w - e6 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 4 + 0);

        // Capturing the checker possible
        let position = Position::from_fen("8/8/8/3Pp3/5K2/8/8/7k w - e6 0 1");
        let mut moves = MoveCounter::new();
        position.legal_moves(&mut moves);
        assert_eq!(moves.moves, 8 + 1);
    }
}
