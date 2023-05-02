use crate::position::Position;

pub fn perft(position: Position, _depth: u8) -> usize {
    position.legal_moves().len()
}
