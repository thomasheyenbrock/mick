mod attacks;

use crate::{square::Square, utils::grid_to_string};
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board(pub u64);

pub const EMPTY: Board = Board(0x0000_0000_0000_0000);
pub const END_RANKS: Board = Board(RANK_1.0 | RANK_8.0);
pub const FILE_A: Board = Board(0x0101_0101_0101_0101);
pub const FILE_B: Board = Board(FILE_A.0 << 1);
pub const FILE_G: Board = Board(FILE_A.0 << 6);
pub const FILE_H: Board = Board(FILE_A.0 << 7);
pub const NOT_FILE_A: Board = Board(!FILE_A.0);
pub const NOT_FILE_H: Board = Board(!FILE_H.0);
pub const RANK_1: Board = Board(0x0000_0000_0000_00FF);
pub const RANK_4: Board = Board(RANK_1.0 << (3 * 8));
pub const RANK_5: Board = Board(RANK_1.0 << (4 * 8));
pub const RANK_8: Board = Board(RANK_1.0 << (7 * 8));

impl Board {
    pub fn any(self) -> bool {
        self.0 != 0
    }

    pub fn iter(self) -> BoardIterator {
        BoardIterator(self)
    }

    pub fn new(square: Square) -> Board {
        Board(1u64 << square.0)
    }

    pub fn occupied(self) -> u32 {
        self.0.count_ones()
    }

    pub fn rotate_left(self, amount: u32) -> Board {
        Board(self.0.rotate_left(amount))
    }

    pub fn rotate_right(self, amount: u32) -> Board {
        Board(self.0.rotate_right(amount))
    }

    pub fn to_square(self) -> Square {
        Square(self.0.trailing_zeros() as u8)
    }
}

impl BitAnd for Board {
    type Output = Board;

    fn bitand(self, other: Board) -> Board {
        Board(self.0 & other.0)
    }
}

impl BitOr for Board {
    type Output = Board;

    fn bitor(self, other: Board) -> Board {
        Board(self.0 | other.0)
    }
}

impl BitOrAssign for Board {
    fn bitor_assign(&mut self, other: Board) {
        self.0 |= other.0
    }
}

impl BitXor for Board {
    type Output = Board;

    fn bitxor(self, other: Board) -> Board {
        Board(self.0 ^ other.0)
    }
}

impl BitXorAssign for Board {
    fn bitxor_assign(&mut self, other: Board) {
        self.0 ^= other.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            grid_to_string(|sq: Square| -> char {
                if (self.0 >> sq.0) & 1 != 0 {
                    '#'
                } else {
                    '.'
                }
            })
        )
    }
}

impl Shl<u8> for Board {
    type Output = Board;

    fn shl(self, amount: u8) -> Board {
        Board(self.0 << amount)
    }
}

impl Shr<u8> for Board {
    type Output = Board;

    fn shr(self, amount: u8) -> Board {
        Board(self.0 >> amount)
    }
}

impl Not for Board {
    type Output = Board;

    fn not(self) -> Board {
        Board(!self.0)
    }
}

pub struct BoardIterator(Board);

impl Iterator for BoardIterator {
    type Item = (Square, Board);

    fn next(&mut self) -> Option<(Square, Board)> {
        let board = self.0;
        if board.0 == EMPTY.0 {
            return None;
        }

        let sq = board.to_square();
        let lsb = Board(board.0 & 0u64.wrapping_sub(board.0));
        self.0 ^= lsb;
        Some((sq, lsb))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, EMPTY},
        square::Square,
    };

    #[test]
    fn iterates_over_empty() {
        assert_eq!(EMPTY.iter().next(), None);
    }

    #[test]
    fn iterates_over_all() {
        let mut iter = (!EMPTY).iter();
        for i in 0..64 {
            assert_eq!(iter.next(), Some((Square(i), Board(1 << i))));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterates_over_some() {
        let mut iter = Board(0x0001_0002_0004_0008).iter();
        assert_eq!(iter.next(), Some((Square(3), Board(1 << 3))));
        assert_eq!(iter.next(), Some((Square(18), Board(1 << 18))));
        assert_eq!(iter.next(), Some((Square(33), Board(1 << 33))));
        assert_eq!(iter.next(), Some((Square(48), Board(1 << 48))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rotate_left() {
        let b = Board(0x0000_0000_0000_0001);
        assert_eq!(b.rotate_left(1), Board(0x0000_0000_0000_0002));
        assert_eq!(b.rotate_left(8), Board(0x0000_0000_0000_0100));
        assert_eq!(b.rotate_left(56), Board(0x0100_0000_0000_0000));
    }
}
