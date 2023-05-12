use super::MoveAdder;
use crate::{
    board::{Board, END_RANKS},
    castle::Castle,
    square::Square,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveCounter {
    pub moves: u64,
    pub captures: u64,
    pub castles: u64,
    pub promotions: u32,
    pub ep_captures: u32,
}

impl MoveCounter {
    pub fn new() -> MoveCounter {
        MoveCounter {
            moves: 0,
            captures: 0,
            castles: 0,
            promotions: 0,
            ep_captures: 0,
        }
    }
}

impl MoveAdder for MoveCounter {
    fn add_pushes(&mut self, _: Square, targets: Board) {
        self.moves += targets.occupied() as u64;
    }

    fn add_captures(&mut self, _: Square, targets: Board) {
        let count = targets.occupied() as u64;
        self.moves += count;
        self.captures += count;
    }

    fn add_castle(&mut self, _: Square, _: Square, _: Castle) {
        self.moves += 1;
        self.castles += 1;
    }

    fn add_pawn_ep_capture(&mut self, _: Square, _: Square) {
        self.moves += 1;
        self.captures += 1;
        self.ep_captures += 1;
    }

    fn add_pawn_pushes(&mut self, _: u8, targets: Board) {
        // non-promotions
        self.moves += (targets & !END_RANKS).occupied() as u64;

        let promo_count = (targets & END_RANKS).occupied() * 4;
        self.moves += promo_count as u64;
        self.promotions += promo_count;
    }

    fn add_double_pawn_pushes(&mut self, _: u8, targets: Board) {
        self.moves += targets.occupied() as u64;
    }

    fn add_pawn_captures(&mut self, _: u8, targets: Board) {
        // non-promotions
        let non_promo_count = (targets & !END_RANKS).occupied();

        let promo_count = (targets & END_RANKS).occupied() * 4;
        self.promotions += promo_count;

        let total = (promo_count + non_promo_count) as u64;
        self.moves += total;
        self.captures += total;
    }
}
