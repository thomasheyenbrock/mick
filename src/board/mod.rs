mod attacks;

use crate::{
    side::{Side, WHITE},
    square::Square,
    utils::grid_to_string,
};
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board(pub u64);

pub const FILE_A: Board = Board(0x0101_0101_0101_0101);
pub const FILE_B: Board = Board(FILE_A.0 << 1);
pub const FILE_G: Board = Board(FILE_A.0 << 6);
pub const FILE_H: Board = Board(FILE_A.0 << 7);
pub const NOT_FILE_A: Board = Board(!FILE_A.0);
pub const NOT_FILE_H: Board = Board(!FILE_H.0);

impl Board {
    pub const EMPTY: Self = Self(0);
    pub const ALL: Self = Self(0xFFFF_FFFF_FFFF_FFFF);

    const FILE_A: Self = Self(0x0101_0101_0101_0101);
    const FILE_B: Self = Self(0x0202_0202_0202_0202);
    const FILE_G: Self = Self(0x4040_4040_4040_4040);
    const FILE_H: Self = Self(0x8080_8080_8080_8080);

    const NOT_FILE_A: Self = Self(0xFEFE_FEFE_FEFE_FEFE);
    const NOT_FILE_H: Self = Self(0x7F7F_7F7F_7F7F_7F7F);

    pub const WHITE_KINGSIDE_BLOCKING: Self = Self(0x0000_0000_0000_0060);
    pub const WHITE_QUEENSIDE_BLOCKING: Self = Self(0x0000_0000_0000_000E);
    pub const BLACK_KINGSIDE_BLOCKING: Self = Self(0x6000_0000_0000_0000);
    pub const BLACK_QUEENSIDE_BLOCKING: Self = Self(0x0E00_0000_0000_0000);

    pub const WHITE_KINGSIDE_SAFE: Self = Self(0x0000_0000_0000_0070);
    pub const WHITE_QUEENSIDE_SAFE: Self = Self(0x0000_0000_0000_001C);
    pub const BLACK_KINGSIDE_SAFE: Self = Self(0x7000_0000_0000_0000);
    pub const BLACK_QUEENSIDE_SAFE: Self = Self(0x1C00_0000_0000_0000);

    pub fn flip_board(&mut self, board: &Self) {
        self.0 ^= board.0
    }

    pub fn flip_square(&mut self, square: &Square) {
        self.0 ^= 1 << square.0
    }

    pub fn has(&self, square: &Square) -> bool {
        self.0 & (1 << square.0) != 0
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator(self.0)
    }

    pub fn king_attacks(&self) -> Self {
        let right = (self.0 << 1) & Self::NOT_FILE_A.0;
        let left = (self.0 >> 1) & Self::NOT_FILE_H.0;
        let side = right | left;

        Self(side | (side << 8) | (side >> 8) | (self.0 << 8) | (self.0 >> 8))
    }

    pub fn new(square: Square) -> Board {
        Board(1u64 << square.0)
    }

    pub fn occupied(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn rotate_left(&self, n: u32) -> Self {
        Self(self.0.rotate_left(n))
    }

    pub fn pawn_attacks(&self, side_to_move: &Side) -> Self {
        let attacks_right = (self.0 << 1) & Self::NOT_FILE_A.0;
        let attacks_left = (self.0 >> 1) & Self::NOT_FILE_H.0;
        let attacks = attacks_right | attacks_left;

        if *side_to_move == WHITE {
            Self(attacks << 8)
        } else {
            Self(attacks >> 8)
        }
    }

    pub fn to_square(&self) -> Square {
        Square(self.0.trailing_zeros() as u8)
    }
}

impl BitAnd for Board {
    type Output = Board;

    fn bitand(self, rhs: Self) -> Self::Output {
        Board(self.0 & rhs.0)
    }
}

impl BitOr for Board {
    type Output = Board;

    fn bitor(self, rhs: Self) -> Self::Output {
        Board(self.0 | rhs.0)
    }
}

impl BitOrAssign for Board {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Board {
    type Output = Board;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Board(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Board {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            grid_to_string(|s: Square| -> char {
                if (self.0 >> s.0) & 1 != 0 {
                    'X'
                } else {
                    ' '
                }
            })
        );
    }
}

impl Not for Board {
    type Output = Board;

    fn not(self) -> Self::Output {
        Board(!self.0)
    }
}

impl Shr<u8> for Board {
    type Output = Board;

    fn shr(self, amount: u8) -> Board {
        Board(self.0 >> amount)
    }
}

impl Shl<u8> for Board {
    type Output = Board;

    fn shl(self, amount: u8) -> Board {
        Board(self.0 << amount)
    }
}

pub struct BoardIterator(u64);

impl Iterator for BoardIterator {
    type Item = (Board, Square);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing = self.0.trailing_zeros() as u8;
        let board = 1 << trailing;
        self.0 ^= board;
        Some((Board(board), Square(trailing)))
    }
}

pub static FILES: [Board; 64] = [
    Board(0x0101_0101_0101_0101), // a1
    Board(0x0202_0202_0202_0202), // b1
    Board(0x0404_0404_0404_0404), // c1
    Board(0x0808_0808_0808_0808), // d1
    Board(0x1010_1010_1010_1010), // e1
    Board(0x2020_2020_2020_2020), // f1
    Board(0x4040_4040_4040_4040), // g1
    Board(0x8080_8080_8080_8080), // h1
    Board(0x0101_0101_0101_0101), // a2
    Board(0x0202_0202_0202_0202), // b2
    Board(0x0404_0404_0404_0404), // c2
    Board(0x0808_0808_0808_0808), // d2
    Board(0x1010_1010_1010_1010), // e2
    Board(0x2020_2020_2020_2020), // f2
    Board(0x4040_4040_4040_4040), // g2
    Board(0x8080_8080_8080_8080), // h2
    Board(0x0101_0101_0101_0101), // a3
    Board(0x0202_0202_0202_0202), // b3
    Board(0x0404_0404_0404_0404), // c3
    Board(0x0808_0808_0808_0808), // d3
    Board(0x1010_1010_1010_1010), // e3
    Board(0x2020_2020_2020_2020), // f3
    Board(0x4040_4040_4040_4040), // g3
    Board(0x8080_8080_8080_8080), // h3
    Board(0x0101_0101_0101_0101), // a4
    Board(0x0202_0202_0202_0202), // b4
    Board(0x0404_0404_0404_0404), // c4
    Board(0x0808_0808_0808_0808), // d4
    Board(0x1010_1010_1010_1010), // e4
    Board(0x2020_2020_2020_2020), // f4
    Board(0x4040_4040_4040_4040), // g4
    Board(0x8080_8080_8080_8080), // h4
    Board(0x0101_0101_0101_0101), // a5
    Board(0x0202_0202_0202_0202), // b5
    Board(0x0404_0404_0404_0404), // c5
    Board(0x0808_0808_0808_0808), // d5
    Board(0x1010_1010_1010_1010), // e5
    Board(0x2020_2020_2020_2020), // f5
    Board(0x4040_4040_4040_4040), // g5
    Board(0x8080_8080_8080_8080), // h5
    Board(0x0101_0101_0101_0101), // a6
    Board(0x0202_0202_0202_0202), // b6
    Board(0x0404_0404_0404_0404), // c6
    Board(0x0808_0808_0808_0808), // d6
    Board(0x1010_1010_1010_1010), // e6
    Board(0x2020_2020_2020_2020), // f6
    Board(0x4040_4040_4040_4040), // g6
    Board(0x8080_8080_8080_8080), // h6
    Board(0x0101_0101_0101_0101), // a7
    Board(0x0202_0202_0202_0202), // b7
    Board(0x0404_0404_0404_0404), // c7
    Board(0x0808_0808_0808_0808), // d7
    Board(0x1010_1010_1010_1010), // e7
    Board(0x2020_2020_2020_2020), // f7
    Board(0x4040_4040_4040_4040), // g7
    Board(0x8080_8080_8080_8080), // h7
    Board(0x0101_0101_0101_0101), // a8
    Board(0x0202_0202_0202_0202), // b8
    Board(0x0404_0404_0404_0404), // c8
    Board(0x0808_0808_0808_0808), // d8
    Board(0x1010_1010_1010_1010), // e8
    Board(0x2020_2020_2020_2020), // f8
    Board(0x4040_4040_4040_4040), // g8
    Board(0x8080_8080_8080_8080), // h8
];

pub static RANKS: [Board; 64] = [
    Board(0x0000_0000_0000_00FF), // a1
    Board(0x0000_0000_0000_00FF), // b1
    Board(0x0000_0000_0000_00FF), // c1
    Board(0x0000_0000_0000_00FF), // d1
    Board(0x0000_0000_0000_00FF), // e1
    Board(0x0000_0000_0000_00FF), // f1
    Board(0x0000_0000_0000_00FF), // g1
    Board(0x0000_0000_0000_00FF), // h1
    Board(0x0000_0000_0000_FF00), // a2
    Board(0x0000_0000_0000_FF00), // b2
    Board(0x0000_0000_0000_FF00), // c2
    Board(0x0000_0000_0000_FF00), // d2
    Board(0x0000_0000_0000_FF00), // e2
    Board(0x0000_0000_0000_FF00), // f2
    Board(0x0000_0000_0000_FF00), // g2
    Board(0x0000_0000_0000_FF00), // h2
    Board(0x0000_0000_00FF_0000), // a3
    Board(0x0000_0000_00FF_0000), // b3
    Board(0x0000_0000_00FF_0000), // c3
    Board(0x0000_0000_00FF_0000), // d3
    Board(0x0000_0000_00FF_0000), // e3
    Board(0x0000_0000_00FF_0000), // f3
    Board(0x0000_0000_00FF_0000), // g3
    Board(0x0000_0000_00FF_0000), // h3
    Board(0x0000_0000_FF00_0000), // a4
    Board(0x0000_0000_FF00_0000), // b4
    Board(0x0000_0000_FF00_0000), // c4
    Board(0x0000_0000_FF00_0000), // d4
    Board(0x0000_0000_FF00_0000), // e4
    Board(0x0000_0000_FF00_0000), // f4
    Board(0x0000_0000_FF00_0000), // g4
    Board(0x0000_0000_FF00_0000), // h4
    Board(0x0000_00FF_0000_0000), // a5
    Board(0x0000_00FF_0000_0000), // b5
    Board(0x0000_00FF_0000_0000), // c5
    Board(0x0000_00FF_0000_0000), // d5
    Board(0x0000_00FF_0000_0000), // e5
    Board(0x0000_00FF_0000_0000), // f5
    Board(0x0000_00FF_0000_0000), // g5
    Board(0x0000_00FF_0000_0000), // h5
    Board(0x0000_FF00_0000_0000), // a6
    Board(0x0000_FF00_0000_0000), // b6
    Board(0x0000_FF00_0000_0000), // c6
    Board(0x0000_FF00_0000_0000), // d6
    Board(0x0000_FF00_0000_0000), // e6
    Board(0x0000_FF00_0000_0000), // f6
    Board(0x0000_FF00_0000_0000), // g6
    Board(0x0000_FF00_0000_0000), // h6
    Board(0x00FF_0000_0000_0000), // a7
    Board(0x00FF_0000_0000_0000), // b7
    Board(0x00FF_0000_0000_0000), // c7
    Board(0x00FF_0000_0000_0000), // d7
    Board(0x00FF_0000_0000_0000), // e7
    Board(0x00FF_0000_0000_0000), // f7
    Board(0x00FF_0000_0000_0000), // g7
    Board(0x00FF_0000_0000_0000), // h7
    Board(0xFF00_0000_0000_0000), // a8
    Board(0xFF00_0000_0000_0000), // b8
    Board(0xFF00_0000_0000_0000), // c8
    Board(0xFF00_0000_0000_0000), // d8
    Board(0xFF00_0000_0000_0000), // e8
    Board(0xFF00_0000_0000_0000), // f8
    Board(0xFF00_0000_0000_0000), // g8
    Board(0xFF00_0000_0000_0000), // h8
];

#[cfg(test)]
mod tests {
    use crate::{board::Board, square::Square};

    #[test]
    fn flips_board() {
        let mut board1 = Board(0x0101_0202_0404_0808);
        let board2 = Board(0x8080_4040_2020_1010);
        let board3 = Board(0x0101_0202_0404_0808);

        board1.flip_board(&board2);
        assert_eq!(board1, Board(0x8181_4242_2424_1818));

        board1.flip_board(&board2);
        assert_eq!(board1, Board(0x0101_0202_0404_0808));

        board1.flip_board(&board3);
        assert_eq!(board1, Board::EMPTY);
    }

    #[test]
    fn flips_square() {
        let mut board = Board::EMPTY;
        let a1 = Square(0);

        board.flip_square(&a1);
        assert_eq!(board, Board(0x0000_0000_0000_0001));

        board.flip_square(&a1);
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
            assert_eq!(iter.next(), Some((Board(1 << i), Square(i))));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterates_over_some() {
        let mut iter = Board(0x0001_0002_0004_0008).iter();
        assert_eq!(iter.next(), Some((Board(1 << 3), Square(3))));
        assert_eq!(iter.next(), Some((Board(1 << 18), Square(18))));
        assert_eq!(iter.next(), Some((Board(1 << 33), Square(33))));
        assert_eq!(iter.next(), Some((Board(1 << 48), Square(48))));
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
