use std::fmt::Display;

use crate::square::Square;

#[derive(Debug, PartialEq)]
pub struct Board(u64);

impl Board {
    pub const EMPTY: Board = Board(0);
    pub const ALL: Board = Board(0xFFFF_FFFF_FFFF_FFFF);

    pub fn flip(&mut self, square: &Square) {
        self.0 ^= 1 << square.to_usize()
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator(self.0)
    }

    pub fn new(i: u64) -> Self {
        Self(i)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "TODO")
    }
}

pub struct BoardIterator(u64);

impl Iterator for BoardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing = self.0.trailing_zeros();
        self.0 ^= 1 << trailing;
        Some(Square::new(trailing as u8))
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, square::Square};

    #[test]
    fn flips_square() {
        let mut board = Board::EMPTY;
        let a1 = Square::new(0);

        board.flip(&a1);
        assert_eq!(board, Board::new(0x0000_0000_0000_0001));

        board.flip(&a1);
        assert_eq!(board, Board::EMPTY);
    }

    #[test]
    fn iterates_over_empty() {
        assert_eq!(Board::EMPTY.iter().next(), None);
    }

    #[test]
    fn iterates_over_all() {
        let mut iter = Board::ALL.iter();
        for i in 0..64 {
            assert_eq!(iter.next(), Some(Square::new(i)));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterates_over_some() {
        let mut iter = Board::new(0x0001_0002_0004_0008).iter();
        assert_eq!(iter.next(), Some(Square::new(3)));
        assert_eq!(iter.next(), Some(Square::new(18)));
        assert_eq!(iter.next(), Some(Square::new(33)));
        assert_eq!(iter.next(), Some(Square::new(48)));
        assert_eq!(iter.next(), None);
    }
}
