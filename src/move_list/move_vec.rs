use super::MoveAdder;
use crate::{
    board::{Board, END_RANKS},
    castle::Castle,
    piece::{PieceKind, BISHOP, KNIGHT, QUEEN, ROOK},
    r#move::Move,
    square::Square,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct MoveVec {
    moves: Vec<Move>,
}

impl Display for MoveVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|mv: &Move| mv.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl MoveAdder for MoveVec {
    fn add_pushes(&mut self, from: Square, targets: Board) {
        self.insert_moves(from, targets, Move::new_push);
    }

    fn add_captures(&mut self, from: Square, targets: Board) {
        self.insert_moves(from, targets, Move::new_capture);
    }

    fn add_castle(&mut self, from: Square, to: Square, castle: Castle) {
        self.moves.push(Move::new_castle(from, to, castle));
    }

    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        self.moves.push(Move::new_capture_en_passant(from, to));
    }

    fn add_pawn_pushes(&mut self, shift: u8, targets: Board) {
        self.insert_promos_by_shift(shift, targets & END_RANKS, Move::new_push_promotion);
        self.insert_moves_by_shift(shift, targets & !END_RANKS, Move::new_push);
    }

    fn add_double_pawn_pushes(&mut self, shift: u8, targets: Board) {
        self.insert_moves_by_shift(shift, targets, Move::new_push_double_pawn);
    }

    fn add_pawn_captures(&mut self, shift: u8, targets: Board) {
        self.insert_promos_by_shift(shift, targets & END_RANKS, Move::new_capture_promotion);
        self.insert_moves_by_shift(shift, targets & !END_RANKS, Move::new_capture);
    }
}

impl Default for MoveVec {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveVec {
    pub fn new() -> MoveVec {
        MoveVec {
            moves: Vec::with_capacity(60),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves.iter()
    }

    fn insert_moves<F: Fn(Square, Square) -> Move>(&mut self, from: Square, targets: Board, f: F) {
        for (to, _) in targets.iter() {
            self.moves.push(f(from, to));
        }
    }

    fn insert_moves_by_shift<F: Fn(Square, Square) -> Move>(
        &mut self,
        shift: u8,
        targets: Board,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift);
            self.moves.push(f(from, to));
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    fn insert_promos_by_shift<F: Fn(Square, Square, PieceKind) -> Move>(
        &mut self,
        shift: u8,
        targets: Board,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift);
            self.moves.push(f(from, to, QUEEN));
            self.moves.push(f(from, to, KNIGHT));
            self.moves.push(f(from, to, BISHOP));
            self.moves.push(f(from, to, ROOK));
        }
    }
}
