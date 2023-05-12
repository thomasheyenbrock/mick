pub mod move_counter;
pub mod move_vec;

use crate::{board::Board, castle::Castle, square::Square};

pub trait MoveAdder {
    fn add_pushes(&mut self, from: Square, targets: Board);

    fn add_captures(&mut self, from: Square, targets: Board);

    fn add_castle(&mut self, from: Square, to: Square, castle: Castle);

    fn add_pawn_pushes(&mut self, shift: u8, targets: Board);

    fn add_pawn_captures(&mut self, shift: u8, targets: Board);

    fn add_pawn_ep_capture(&mut self, from: Square, to: Square);
}
