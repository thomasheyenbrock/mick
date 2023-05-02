use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not},
};

use crate::{side::Side, square::Square};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board(u64);

impl Board {
    pub const EMPTY: Self = Self(0);
    pub const ALL: Self = Self(0xFFFF_FFFF_FFFF_FFFF);

    const FILE_A: Self = Self(0x0101_0101_0101_0101);
    const FILE_B: Self = Self(0x0202_0202_0202_0202);
    const FILE_G: Self = Self(0x4040_4040_4040_4040);
    const FILE_H: Self = Self(0x8080_8080_8080_8080);

    const NOT_FILE_A: Self = Self(0xFEFE_FEFE_FEFE_FEFE);
    const NOT_FILE_H: Self = Self(0x7F7F_7F7F_7F7F_7F7F);

    pub fn diagonal_attacks(&self, empty_squares: &Self) -> Self {
        self.north_east_attacks(empty_squares)
            | self.north_west_attacks(empty_squares)
            | self.south_east_attacks(empty_squares)
            | self.south_west_attacks(empty_squares)
    }

    pub fn east_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 1);
        prop &= prop << 1;
        gen |= prop & (gen << 2);
        prop &= prop << 2;
        gen |= prop & (gen << 4);

        Self((gen << 1) & Self::NOT_FILE_A.0)
    }

    pub fn flip_board(&mut self, board: &Self) {
        self.0 ^= board.0
    }

    pub fn flip_square(&mut self, square: &Square) {
        self.0 ^= 1 << square.to_usize()
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator(self.0)
    }

    pub fn king_attacks(&self) -> Self {
        let right = (self.0 << 1) & Self::NOT_FILE_A.0;
        let left = (self.0 >> 1) & Self::NOT_FILE_H.0;
        let side = right | left;

        Self((side << 8) | (side >> 8) | (self.0 << 8) | (self.0 >> 8))
    }

    pub fn knight_attacks(&self) -> Self {
        let attacks_right_one = (self.0 << 1) & Self::NOT_FILE_A.0;
        let attacks_right_two = (self.0 << 2) & !(Self::FILE_A.0 | Self::FILE_B.0);
        let attacks_left_one = (self.0 >> 1) & Self::NOT_FILE_H.0;
        let attacks_left_two = (self.0 >> 2) & !(Self::FILE_H.0 | Self::FILE_G.0);

        let attacks_one = attacks_right_one | attacks_left_one;
        let attacks_two = attacks_right_two | attacks_left_two;

        Self((attacks_one << 16) | (attacks_one >> 16) | (attacks_two << 8) | (attacks_two >> 8))
    }

    pub fn new(i: u64) -> Self {
        Self(i)
    }

    pub fn north_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0;
        let mut gen = self.0;

        gen |= prop & (gen << 8);
        prop &= prop << 8;
        gen |= prop & (gen << 16);
        prop &= prop << 16;
        gen |= prop & (gen << 32);

        Self(gen << 8)
    }

    pub fn north_east_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 9);
        prop &= prop << 9;
        gen |= prop & (gen << 18);
        prop &= prop << 18;
        gen |= prop & (gen << 36);

        Self((gen << 9) & Self::NOT_FILE_A.0)
    }

    pub fn north_west_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen << 7);
        prop &= prop << 7;
        gen |= prop & (gen << 14);
        prop &= prop << 14;
        gen |= prop & (gen << 28);

        Self((gen << 7) & Self::NOT_FILE_H.0)
    }

    pub fn occupied(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn pawn_attacks(&self, side_to_move: &Side) -> Self {
        let attacks_right = (self.0 << 1) & Self::NOT_FILE_A.0;
        let attacks_left = (self.0 >> 1) & Self::NOT_FILE_H.0;
        let attacks = attacks_right | attacks_left;

        if *side_to_move == Side::WHITE {
            Self(attacks << 8)
        } else {
            Self(attacks >> 8)
        }
    }

    pub fn south_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 8);
        prop &= prop >> 8;
        gen |= prop & (gen >> 16);
        prop &= prop >> 16;
        gen |= prop & (gen >> 32);

        Self(gen >> 8)
    }

    pub fn south_east_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 7);
        prop &= prop >> 7;
        gen |= prop & (gen >> 14);
        prop &= prop >> 14;
        gen |= prop & (gen >> 28);

        Self((gen >> 7) & Self::NOT_FILE_A.0)
    }

    pub fn south_west_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 9);
        prop &= prop >> 9;
        gen |= prop & (gen >> 18);
        prop &= prop >> 18;
        gen |= prop & (gen >> 36);

        Self((gen >> 9) & Self::NOT_FILE_H.0)
    }

    pub fn straight_attacks(&self, empty_squares: &Self) -> Self {
        self.north_attacks(empty_squares)
            | self.south_attacks(empty_squares)
            | self.east_attacks(empty_squares)
            | self.west_attacks(empty_squares)
    }

    pub fn to_square(&self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn west_attacks(&self, empty_squares: &Self) -> Self {
        let mut prop = empty_squares.0 & Self::NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 1);
        prop &= prop >> 1;
        gen |= prop & (gen >> 2);
        prop &= prop >> 2;
        gen |= prop & (gen >> 4);

        Self((gen >> 1) & Self::NOT_FILE_H.0)
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

const GRID_FILES: &str = "    A    B    C    D    E    F    G    H";
const GRID_TOP: &str = "  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n";
const GRID_MIDDLE: &str = "  ├───┼───┼───┼───┼───┼───┼───┼───┤\n";
const GRID_BOTTOM: &str = "  └───┴───┴───┴───┴───┴───┴───┴───┘\n";

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printed = String::from(GRID_FILES) + "\n" + GRID_TOP;
        for rank_index in 0..8 {
            let rank_index = 8 - rank_index;
            printed += &rank_index.to_string();
            printed += " ";
            for file_index in 0..8 {
                if self.0 & (1 << (8 * (rank_index - 1) + file_index)) == 0 {
                    printed += "|   ";
                } else {
                    printed += "| X ";
                }
            }
            printed += "|\n";
            printed += if rank_index == 1 {
                GRID_BOTTOM
            } else {
                GRID_MIDDLE
            }
        }
        write!(f, "{}", printed)
    }
}

impl Not for Board {
    type Output = Board;

    fn not(self) -> Self::Output {
        Board(!self.0)
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

pub static STRAIGHT_RAYS: [Board; 64] = [
    Board(0x0101_0101_0101_01FE), // a1
    Board(0x0202_0202_0202_02FD), // a2
    Board(0x0404_0404_0404_04FB), // a3
    Board(0x0808_0808_0808_08F7), // a4
    Board(0x1010_1010_1010_10EF), // a5
    Board(0x2020_2020_2020_20DF), // a6
    Board(0x4040_4040_4040_40BF), // a7
    Board(0x8080_8080_8080_807F), // a8
    Board(0x0101_0101_0101_FE01), // b1
    Board(0x0202_0202_0202_FD02), // b2
    Board(0x0404_0404_0404_FB04), // b3
    Board(0x0808_0808_0808_F708), // b4
    Board(0x1010_1010_1010_EF10), // b5
    Board(0x2020_2020_2020_DF20), // b6
    Board(0x4040_4040_4040_BF40), // b7
    Board(0x8080_8080_8080_7F80), // b8
    Board(0x0101_0101_01FE_0101), // c1
    Board(0x0202_0202_02FD_0202), // c2
    Board(0x0404_0404_04FB_0404), // c3
    Board(0x0808_0808_08F7_0808), // c4
    Board(0x1010_1010_10EF_1010), // c5
    Board(0x2020_2020_20DF_2020), // c6
    Board(0x4040_4040_40BF_4040), // c7
    Board(0x8080_8080_807F_8080), // c8
    Board(0x0101_0101_FE01_0101), // d1
    Board(0x0202_0202_FD02_0202), // d2
    Board(0x0404_0404_FB04_0404), // d3
    Board(0x0808_0808_F708_0808), // d4
    Board(0x1010_1010_EF10_1010), // d5
    Board(0x2020_2020_DF20_2020), // d6
    Board(0x4040_4040_BF40_4040), // d7
    Board(0x8080_8080_7F80_8080), // d8
    Board(0x0101_01FE_0101_0101), // e1
    Board(0x0202_02FD_0202_0202), // e2
    Board(0x0404_04FB_0404_0404), // e3
    Board(0x0808_08F7_0808_0808), // e4
    Board(0x1010_10EF_1010_1010), // e5
    Board(0x2020_20DF_2020_2020), // e6
    Board(0x4040_40BF_4040_4040), // e7
    Board(0x8080_807F_8080_8080), // e8
    Board(0x0101_FE01_0101_0101), // f1
    Board(0x0202_FD02_0202_0202), // f2
    Board(0x0404_FB04_0404_0404), // f3
    Board(0x0808_F708_0808_0808), // f4
    Board(0x1010_EF10_1010_1010), // f5
    Board(0x2020_DF20_2020_2020), // f6
    Board(0x4040_BF40_4040_4040), // f7
    Board(0x8080_7F80_8080_8080), // f8
    Board(0x01FE_0101_0101_0101), // g1
    Board(0x02FD_0202_0202_0202), // g2
    Board(0x04FB_0404_0404_0404), // g3
    Board(0x08F7_0808_0808_0808), // g4
    Board(0x10EF_1010_1010_1010), // g5
    Board(0x20DF_2020_2020_2020), // g6
    Board(0x40BF_4040_4040_4040), // g7
    Board(0x807F_8080_8080_8080), // g8
    Board(0xFE01_0101_0101_0101), // h1
    Board(0xFD02_0202_0202_0202), // h2
    Board(0xFB04_0404_0404_0404), // h3
    Board(0xF708_0808_0808_0808), // h4
    Board(0xEF10_1010_1010_1010), // h5
    Board(0xDF20_2020_2020_2020), // h6
    Board(0xBF40_4040_4040_4040), // h7
    Board(0x7F80_8080_8080_8080), // h8
];

pub static DIAGONAL_RAYS: [Board; 64] = [
    Board(0x8040_2010_0804_0200), // a1
    Board(0x0080_4020_1008_0500), // a2
    Board(0x0000_8040_2011_0a00), // a3
    Board(0x0000_0080_4122_1400), // a4
    Board(0x0000_0001_8244_2800), // a5
    Board(0x0000_0102_0488_5000), // a6
    Board(0x0001_0204_0810_a000), // a7
    Board(0x0102_0408_1020_4000), // a8
    Board(0x4020_1008_0402_0002), // b1
    Board(0x8040_2010_0805_0005), // b2
    Board(0x0080_4020_110a_000a), // b3
    Board(0x0000_8041_2214_0014), // b4
    Board(0x0000_0182_4428_0028), // b5
    Board(0x0001_0204_8850_0050), // b6
    Board(0x0102_0408_10a0_00a0), // b7
    Board(0x0204_0810_2040_0040), // b8
    Board(0x2010_0804_0200_0204), // c1
    Board(0x4020_1008_0500_0508), // c2
    Board(0x8040_2011_0a00_0a11), // c3
    Board(0x0080_4122_1400_1422), // c4
    Board(0x0001_8244_2800_2844), // c5
    Board(0x0102_0488_5000_5088), // c6
    Board(0x0204_0810_a000_a010), // c7
    Board(0x0408_1020_4000_4020), // c8
    Board(0x1008_0402_0002_0408), // d1
    Board(0x2010_0805_0005_0810), // d2
    Board(0x4020_110a_000a_1120), // d3
    Board(0x8041_2214_0014_2241), // d4
    Board(0x0182_4428_0028_4482), // d5
    Board(0x0204_8850_0050_8804), // d6
    Board(0x0408_10a0_00a0_1008), // d7
    Board(0x0810_2040_0040_2010), // d8
    Board(0x0804_0200_0204_0810), // e1
    Board(0x1008_0500_0508_1020), // e2
    Board(0x2011_0a00_0a11_2040), // e3
    Board(0x4122_1400_1422_4180), // e4
    Board(0x8244_2800_2844_8201), // e5
    Board(0x0488_5000_5088_0402), // e6
    Board(0x0810_a000_a010_0804), // e7
    Board(0x1020_4000_4020_1008), // e8
    Board(0x0402_0002_0408_1020), // f1
    Board(0x0805_0005_0810_2040), // f2
    Board(0x110a_000a_1120_4080), // f3
    Board(0x2214_0014_2241_8000), // f4
    Board(0x4428_0028_4482_0100), // f5
    Board(0x8850_0050_8804_0201), // f6
    Board(0x10a0_00a0_1008_0402), // f7
    Board(0x2040_0040_2010_0804), // f8
    Board(0x0200_0204_0810_2040), // g1
    Board(0x0500_0508_1020_4080), // g2
    Board(0x0a00_0a11_2040_8000), // g3
    Board(0x1400_1422_4180_0000), // g4
    Board(0x2800_2844_8201_0000), // g5
    Board(0x5000_5088_0402_0100), // g6
    Board(0xa000_a010_0804_0201), // g7
    Board(0x4000_4020_1008_0402), // g8
    Board(0x0002_0408_1020_4080), // h1
    Board(0x0005_0810_2040_8000), // h2
    Board(0x000a_1120_4080_0000), // h3
    Board(0x0014_2241_8000_0000), // h4
    Board(0x0028_4482_0100_0000), // h5
    Board(0x0050_8804_0201_0000), // h6
    Board(0x00a0_1008_0402_0100), // h7
    Board(0x0040_2010_0804_0201), // h8
];

pub static SQUARES_BETWEEN: [[Board; 64]; 64] = [
    // a1
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0002), // a3
        Board(0x0000_0000_0000_0006), // a4
        Board(0x0000_0000_0000_000e), // a5
        Board(0x0000_0000_0000_001e), // a6
        Board(0x0000_0000_0000_003e), // a7
        Board(0x0000_0000_0000_007e), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0100), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0200), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0001_0100), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0004_0200), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0101_0100), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0804_0200), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0001_0101_0100), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0010_0804_0200), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0101_0101_0100), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_2010_0804_0200), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0101_0101_0100), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_2010_0804_0200), // h8
    ],
    // a2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0004), // a4
        Board(0x0000_0000_0000_000c), // a5
        Board(0x0000_0000_0000_001c), // a6
        Board(0x0000_0000_0000_003c), // a7
        Board(0x0000_0000_0000_007c), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0200), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0400), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0002_0200), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0008_0400), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0202_0200), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_1008_0400), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0002_0202_0200), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0020_1008_0400), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0202_0202_0200), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_4020_1008_0400), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0202_0202_0200), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a3
    [
        Board(0x0000_0000_0000_0002), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0008), // a5
        Board(0x0000_0000_0000_0018), // a6
        Board(0x0000_0000_0000_0038), // a7
        Board(0x0000_0000_0000_0078), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0200), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0400), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0800), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0004_0400), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0010_0800), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0404_0400), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_2010_0800), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0004_0404_0400), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0040_2010_0800), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0404_0404_0400), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0404_0404_0400), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a4
    [
        Board(0x0000_0000_0000_0006), // a1
        Board(0x0000_0000_0000_0004), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0010), // a6
        Board(0x0000_0000_0000_0030), // a7
        Board(0x0000_0000_0000_0070), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0400), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0800), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_1000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0002_0400), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0008_0800), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0020_1000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0808_0800), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_4020_1000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0008_0808_0800), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0808_0808_0800), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0808_0808_0800), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a5
    [
        Board(0x0000_0000_0000_000e), // a1
        Board(0x0000_0000_0000_000c), // a2
        Board(0x0000_0000_0000_0008), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0020), // a7
        Board(0x0000_0000_0000_0060), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0800), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_1000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_2000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0004_0800), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0010_1000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0040_2000), // d8
        Board(0x0000_0000_0204_0800), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_1010_1000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0010_1010_1000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_1010_1010_1000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_1010_1010_1000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a6
    [
        Board(0x0000_0000_0000_001e), // a1
        Board(0x0000_0000_0000_001c), // a2
        Board(0x0000_0000_0000_0018), // a3
        Board(0x0000_0000_0000_0010), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0040), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_1000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_2000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_4000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0008_1000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0020_2000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0408_1000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_2020_2000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0002_0408_1000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0020_2020_2000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_2020_2020_2000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_2020_2020_2000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a7
    [
        Board(0x0000_0000_0000_003e), // a1
        Board(0x0000_0000_0000_003c), // a2
        Board(0x0000_0000_0000_0038), // a3
        Board(0x0000_0000_0000_0030), // a4
        Board(0x0000_0000_0000_0020), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_2000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_4000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0010_2000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0040_4000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0810_2000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_4040_4000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0004_0810_2000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0040_4040_4000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0204_0810_2000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_4040_4040_4000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_4040_4040_4000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // a8
    [
        Board(0x0000_0000_0000_007e), // a1
        Board(0x0000_0000_0000_007c), // a2
        Board(0x0000_0000_0000_0078), // a3
        Board(0x0000_0000_0000_0070), // a4
        Board(0x0000_0000_0000_0060), // a5
        Board(0x0000_0000_0000_0040), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_4000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_8000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0020_4000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0080_8000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_1020_4000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_8080_8000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0008_1020_4000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0080_8080_8000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0408_1020_4000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_8080_8080_8000), // g8
        Board(0x0002_0408_1020_4000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_8080_8080_8000), // h8
    ],
    // b1
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0200), // b3
        Board(0x0000_0000_0000_0600), // b4
        Board(0x0000_0000_0000_0e00), // b5
        Board(0x0000_0000_0000_1e00), // b6
        Board(0x0000_0000_0000_3e00), // b7
        Board(0x0000_0000_0000_7e00), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0001_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0002_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0101_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0402_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0001_0101_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0008_0402_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0101_0101_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_1008_0402_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0101_0101_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0020_1008_0402_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0400), // b4
        Board(0x0000_0000_0000_0c00), // b5
        Board(0x0000_0000_0000_1c00), // b6
        Board(0x0000_0000_0000_3c00), // b7
        Board(0x0000_0000_0000_7c00), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0002_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0004_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0202_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0804_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0002_0202_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0010_0804_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0202_0202_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_2010_0804_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0202_0202_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_2010_0804_0000), // h8
    ],
    // b3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0200), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0800), // b5
        Board(0x0000_0000_0000_1800), // b6
        Board(0x0000_0000_0000_3800), // b7
        Board(0x0000_0000_0000_7800), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0002_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0004_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0008_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0404_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_1008_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0004_0404_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0020_1008_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0404_0404_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_4020_1008_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0404_0404_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0600), // b1
        Board(0x0000_0000_0000_0400), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_1000), // b6
        Board(0x0000_0000_0000_3000), // b7
        Board(0x0000_0000_0000_7000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0004_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0008_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0010_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0204_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0808_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_2010_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0008_0808_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0040_2010_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0808_0808_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0808_0808_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0e00), // b1
        Board(0x0000_0000_0000_0c00), // b2
        Board(0x0000_0000_0000_0800), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_2000), // b7
        Board(0x0000_0000_0000_6000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0008_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0010_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0020_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0408_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_1010_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_4020_0000), // e8
        Board(0x0000_0002_0408_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0010_1010_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_1010_1010_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_1010_1010_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_1e00), // b1
        Board(0x0000_0000_0000_1c00), // b2
        Board(0x0000_0000_0000_1800), // b3
        Board(0x0000_0000_0000_1000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_4000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0010_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0020_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0040_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0810_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_2020_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0004_0810_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0020_2020_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0204_0810_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_2020_2020_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_2020_2020_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_3e00), // b1
        Board(0x0000_0000_0000_3c00), // b2
        Board(0x0000_0000_0000_3800), // b3
        Board(0x0000_0000_0000_3000), // b4
        Board(0x0000_0000_0000_2000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0020_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0040_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_1020_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_4040_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0008_1020_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0040_4040_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0408_1020_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_4040_4040_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0002_0408_1020_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_4040_4040_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // b8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_7e00), // b1
        Board(0x0000_0000_0000_7c00), // b2
        Board(0x0000_0000_0000_7800), // b3
        Board(0x0000_0000_0000_7000), // b4
        Board(0x0000_0000_0000_6000), // b5
        Board(0x0000_0000_0000_4000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0040_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0080_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_2040_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_8080_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0010_2040_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0080_8080_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0810_2040_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_8080_8080_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0004_0810_2040_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_8080_8080_0000), // h8
    ],
    // c1
    [
        Board(0x0000_0000_0000_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0200), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0002_0000), // c3
        Board(0x0000_0000_0006_0000), // c4
        Board(0x0000_0000_000e_0000), // c5
        Board(0x0000_0000_001e_0000), // c6
        Board(0x0000_0000_003e_0000), // c7
        Board(0x0000_0000_007e_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0100_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0200_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0001_0100_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0004_0200_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0101_0100_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0804_0200_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0101_0100_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0010_0804_0200_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0400), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0004_0000), // c4
        Board(0x0000_0000_000c_0000), // c5
        Board(0x0000_0000_001c_0000), // c6
        Board(0x0000_0000_003c_0000), // c7
        Board(0x0000_0000_007c_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0200_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0400_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0002_0200_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0008_0400_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0202_0200_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_1008_0400_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0202_0200_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0020_1008_0400_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c3
    [
        Board(0x0000_0000_0000_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0800), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0002_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0008_0000), // c5
        Board(0x0000_0000_0018_0000), // c6
        Board(0x0000_0000_0038_0000), // c7
        Board(0x0000_0000_0078_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0200_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0400_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0800_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0004_0400_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0010_0800_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0404_0400_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_2010_0800_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0404_0400_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_2010_0800_0000), // h8
    ],
    // c4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0400), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_1000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0006_0000), // c1
        Board(0x0000_0000_0004_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0010_0000), // c6
        Board(0x0000_0000_0030_0000), // c7
        Board(0x0000_0000_0070_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0400_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0800_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_1000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0002_0400_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0008_0800_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0020_1000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0808_0800_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_4020_1000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0808_0800_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0800), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_2000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_000e_0000), // c1
        Board(0x0000_0000_000c_0000), // c2
        Board(0x0000_0000_0008_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0020_0000), // c7
        Board(0x0000_0000_0060_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0800_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_1000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_2000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0004_0800_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0010_1000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0040_2000_0000), // f8
        Board(0x0000_0204_0800_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_1010_1000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_1010_1000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_1000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_4000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_001e_0000), // c1
        Board(0x0000_0000_001c_0000), // c2
        Board(0x0000_0000_0018_0000), // c3
        Board(0x0000_0000_0010_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0040_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_1000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_2000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_4000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0008_1000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0020_2000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0408_1000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_2020_2000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0002_0408_1000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_2020_2000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_2000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_003e_0000), // c1
        Board(0x0000_0000_003c_0000), // c2
        Board(0x0000_0000_0038_0000), // c3
        Board(0x0000_0000_0030_0000), // c4
        Board(0x0000_0000_0020_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_2000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_4000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0010_2000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0040_4000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0810_2000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_4040_4000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0004_0810_2000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_4040_4000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // c8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_4000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_007e_0000), // c1
        Board(0x0000_0000_007c_0000), // c2
        Board(0x0000_0000_0078_0000), // c3
        Board(0x0000_0000_0070_0000), // c4
        Board(0x0000_0000_0060_0000), // c5
        Board(0x0000_0000_0040_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_4000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_8000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0020_4000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0080_8000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_1020_4000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_8080_8000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0008_1020_4000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_8080_8000_0000), // h8
    ],
    // d1
    [
        Board(0x0000_0000_0001_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0002_0400), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0001_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0002_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0200_0000), // d3
        Board(0x0000_0000_0600_0000), // d4
        Board(0x0000_0000_0e00_0000), // d5
        Board(0x0000_0000_1e00_0000), // d6
        Board(0x0000_0000_3e00_0000), // d7
        Board(0x0000_0000_7e00_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0001_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0002_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0101_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0402_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0101_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0008_0402_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0002_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0004_0800), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0002_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0004_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0400_0000), // d4
        Board(0x0000_0000_0c00_0000), // d5
        Board(0x0000_0000_1c00_0000), // d6
        Board(0x0000_0000_3c00_0000), // d7
        Board(0x0000_0000_7c00_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0002_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0004_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0202_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0804_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0202_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0010_0804_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0004_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0008_1000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0002_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0004_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0008_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0200_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0800_0000), // d5
        Board(0x0000_0000_1800_0000), // d6
        Board(0x0000_0000_3800_0000), // d7
        Board(0x0000_0000_7800_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0002_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0004_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0008_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0404_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_1008_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0404_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0020_1008_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d4
    [
        Board(0x0000_0000_0004_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0008_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0010_2000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0004_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0008_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0010_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0600_0000), // d1
        Board(0x0000_0000_0400_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_1000_0000), // d6
        Board(0x0000_0000_3000_0000), // d7
        Board(0x0000_0000_7000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0004_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0008_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0010_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0204_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0808_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_2010_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0808_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_2010_0000_0000), // h8
    ],
    // d5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0008_0400), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0010_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0020_4000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0008_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0010_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0020_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0e00_0000), // d1
        Board(0x0000_0000_0c00_0000), // d2
        Board(0x0000_0000_0800_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_2000_0000), // d7
        Board(0x0000_0000_6000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0008_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0010_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0020_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0408_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_1010_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_4020_0000_0000), // g8
        Board(0x0002_0408_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_1010_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0010_0800), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0020_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0010_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0020_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0040_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_1e00_0000), // d1
        Board(0x0000_0000_1c00_0000), // d2
        Board(0x0000_0000_1800_0000), // d3
        Board(0x0000_0000_1000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_4000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0010_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0020_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0040_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0810_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_2020_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0004_0810_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_2020_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0020_1000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0040_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0020_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0040_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_3e00_0000), // d1
        Board(0x0000_0000_3c00_0000), // d2
        Board(0x0000_0000_3800_0000), // d3
        Board(0x0000_0000_3000_0000), // d4
        Board(0x0000_0000_2000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0020_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0040_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_1020_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_4040_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0008_1020_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_4040_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // d8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0040_2000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0080_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0040_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0080_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_7e00_0000), // d1
        Board(0x0000_0000_7c00_0000), // d2
        Board(0x0000_0000_7800_0000), // d3
        Board(0x0000_0000_7000_0000), // d4
        Board(0x0000_0000_6000_0000), // d5
        Board(0x0000_0000_4000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0040_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0080_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_2040_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_8080_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0010_2040_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_8080_0000_0000), // h8
    ],
    // e1
    [
        Board(0x0000_0000_0101_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0204_0800), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0101_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0204_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0100_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0200_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0002_0000_0000), // e3
        Board(0x0000_0006_0000_0000), // e4
        Board(0x0000_000e_0000_0000), // e5
        Board(0x0000_001e_0000_0000), // e6
        Board(0x0000_003e_0000_0000), // e7
        Board(0x0000_007e_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0100_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0200_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0100_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0004_0200_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0202_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0408_1000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0202_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0408_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0200_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0400_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0004_0000_0000), // e4
        Board(0x0000_000c_0000_0000), // e5
        Board(0x0000_001c_0000_0000), // e6
        Board(0x0000_003c_0000_0000), // e7
        Board(0x0000_007c_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0200_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0400_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0200_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0008_0400_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0404_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0810_2000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0404_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0810_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0200_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0400_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0800_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0002_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0008_0000_0000), // e5
        Board(0x0000_0018_0000_0000), // e6
        Board(0x0000_0038_0000_0000), // e7
        Board(0x0000_0078_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0200_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0400_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0800_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0400_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0010_0800_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0808_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_1020_4000), // a8
        Board(0x0000_0000_0402_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0808_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_1020_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0400_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0800_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_1000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0006_0000_0000), // e1
        Board(0x0000_0004_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0010_0000_0000), // e6
        Board(0x0000_0030_0000_0000), // e7
        Board(0x0000_0070_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0400_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0800_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_1000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0002_0400_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0800_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0020_1000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e5
    [
        Board(0x0000_0000_0804_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_1010_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0804_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_1010_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_2040_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0800_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_1000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_2000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_000e_0000_0000), // e1
        Board(0x0000_000c_0000_0000), // e2
        Board(0x0000_0008_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0020_0000_0000), // e7
        Board(0x0000_0060_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0800_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_1000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_2000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0004_0800_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_1000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_2000_0000_0000), // h8
    ],
    // e6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_1008_0400), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_2020_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_1008_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_2020_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_1000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_2000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_4000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_001e_0000_0000), // e1
        Board(0x0000_001c_0000_0000), // e2
        Board(0x0000_0018_0000_0000), // e3
        Board(0x0000_0010_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0040_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_1000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_2000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_4000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0008_1000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_2000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_2010_0800), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_4040_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_2010_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_4040_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_2000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_4000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_003e_0000_0000), // e1
        Board(0x0000_003c_0000_0000), // e2
        Board(0x0000_0038_0000_0000), // e3
        Board(0x0000_0030_0000_0000), // e4
        Board(0x0000_0020_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_2000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_4000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0010_2000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_4000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // e8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_4020_1000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_8080_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_4020_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_8080_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_4000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_8000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_007e_0000_0000), // e1
        Board(0x0000_007c_0000_0000), // e2
        Board(0x0000_0078_0000_0000), // e3
        Board(0x0000_0070_0000_0000), // e4
        Board(0x0000_0060_0000_0000), // e5
        Board(0x0000_0040_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_4000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_8000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0020_4000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_8000_0000_0000), // h8
    ],
    // f1
    [
        Board(0x0000_0001_0101_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0002_0408_1000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0001_0101_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0002_0408_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0001_0100_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0002_0400_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0001_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0002_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0200_0000_0000), // f3
        Board(0x0000_0600_0000_0000), // f4
        Board(0x0000_0e00_0000_0000), // f5
        Board(0x0000_1e00_0000_0000), // f6
        Board(0x0000_3e00_0000_0000), // f7
        Board(0x0000_7e00_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0001_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0002_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0002_0202_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0004_0810_2000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0002_0202_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0004_0810_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0002_0200_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0004_0800_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0002_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0004_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0400_0000_0000), // f4
        Board(0x0000_0c00_0000_0000), // f5
        Board(0x0000_1c00_0000_0000), // f6
        Board(0x0000_3c00_0000_0000), // f7
        Board(0x0000_7c00_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0002_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0004_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0004_0404_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0008_1020_4000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0004_0404_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0008_1020_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0004_0400_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0008_1000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0002_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0004_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0008_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0200_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0800_0000_0000), // f5
        Board(0x0000_1800_0000_0000), // f6
        Board(0x0000_3800_0000_0000), // f7
        Board(0x0000_7800_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0002_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0004_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0008_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0008_0808_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0008_0808_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0010_2040_0000), // b8
        Board(0x0000_0004_0200_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0008_0800_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0010_2000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0004_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0008_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0010_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0600_0000_0000), // f1
        Board(0x0000_0400_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_1000_0000_0000), // f6
        Board(0x0000_3000_0000_0000), // f7
        Board(0x0000_7000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0004_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0008_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0010_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0010_1010_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0008_0402_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0010_1010_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0008_0400_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0010_1000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0020_4000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0008_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0010_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0020_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0e00_0000_0000), // f1
        Board(0x0000_0c00_0000_0000), // f2
        Board(0x0000_0800_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_2000_0000_0000), // f7
        Board(0x0000_6000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0008_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0010_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0020_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f6
    [
        Board(0x0000_0010_0804_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0020_2020_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0010_0804_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0020_2020_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0010_0800_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0020_2000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0010_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0020_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0040_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_1e00_0000_0000), // f1
        Board(0x0000_1c00_0000_0000), // f2
        Board(0x0000_1800_0000_0000), // f3
        Board(0x0000_1000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_4000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0010_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0020_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0040_0000_0000_0000), // h8
    ],
    // f7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0020_1008_0400), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0040_4040_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0020_1008_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0040_4040_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0020_1000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0040_4000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0020_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0040_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_3e00_0000_0000), // f1
        Board(0x0000_3c00_0000_0000), // f2
        Board(0x0000_3800_0000_0000), // f3
        Board(0x0000_3000_0000_0000), // f4
        Board(0x0000_2000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0020_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0040_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // f8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0040_2010_0800), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0080_8080_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0040_2010_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0080_8080_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0040_2000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0080_8000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0040_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0080_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_7e00_0000_0000), // f1
        Board(0x0000_7c00_0000_0000), // f2
        Board(0x0000_7800_0000_0000), // f3
        Board(0x0000_7000_0000_0000), // f4
        Board(0x0000_6000_0000_0000), // f5
        Board(0x0000_4000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0040_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0080_0000_0000_0000), // h8
    ],
    // g1
    [
        Board(0x0000_0101_0101_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0204_0810_2000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0101_0101_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0204_0810_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0101_0100_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0204_0800_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0101_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0204_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0100_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0200_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0002_0000_0000_0000), // g3
        Board(0x0006_0000_0000_0000), // g4
        Board(0x000e_0000_0000_0000), // g5
        Board(0x001e_0000_0000_0000), // g6
        Board(0x003e_0000_0000_0000), // g7
        Board(0x007e_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0202_0202_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0408_1020_4000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0202_0202_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0408_1020_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0202_0200_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0408_1000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0202_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0408_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0200_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0400_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0004_0000_0000_0000), // g4
        Board(0x000c_0000_0000_0000), // g5
        Board(0x001c_0000_0000_0000), // g6
        Board(0x003c_0000_0000_0000), // g7
        Board(0x007c_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0404_0404_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0404_0404_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0810_2040_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0404_0400_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0810_2000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0404_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0810_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0200_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0400_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0800_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0002_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0008_0000_0000_0000), // g5
        Board(0x0018_0000_0000_0000), // g6
        Board(0x0038_0000_0000_0000), // g7
        Board(0x0078_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0808_0808_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0808_0808_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0808_0800_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_1020_4000_0000), // c8
        Board(0x0000_0402_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0808_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_1020_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0400_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0800_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_1000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0006_0000_0000_0000), // g1
        Board(0x0004_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0010_0000_0000_0000), // g6
        Board(0x0030_0000_0000_0000), // g7
        Board(0x0070_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_1010_1010_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_1010_1010_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0804_0200_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_1010_1000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0804_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_1010_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_2040_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0800_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_1000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_2000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x000e_0000_0000_0000), // g1
        Board(0x000c_0000_0000_0000), // g2
        Board(0x0008_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0020_0000_0000_0000), // g7
        Board(0x0060_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_2020_2020_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_1008_0402_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_2020_2020_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_1008_0400_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_2020_2000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_1008_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_2020_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_1000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_2000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_4000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x001e_0000_0000_0000), // g1
        Board(0x001c_0000_0000_0000), // g2
        Board(0x0018_0000_0000_0000), // g3
        Board(0x0010_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0040_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g7
    [
        Board(0x0000_2010_0804_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_4040_4040_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_2010_0804_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_4040_4040_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_2010_0800_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_4040_4000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_2010_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_4040_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_2000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_4000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x003e_0000_0000_0000), // g1
        Board(0x003c_0000_0000_0000), // g2
        Board(0x0038_0000_0000_0000), // g3
        Board(0x0030_0000_0000_0000), // g4
        Board(0x0020_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // g8
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_4020_1008_0400), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_8080_8080_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_4020_1008_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_8080_8080_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_4020_1000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_8080_8000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_4020_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_8080_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_4000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_8000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x007e_0000_0000_0000), // g1
        Board(0x007c_0000_0000_0000), // g2
        Board(0x0078_0000_0000_0000), // g3
        Board(0x0070_0000_0000_0000), // g4
        Board(0x0060_0000_0000_0000), // g5
        Board(0x0040_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // h1
    [
        Board(0x0001_0101_0101_0100), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0002_0408_1020_4000), // a8
        Board(0x0001_0101_0101_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0002_0408_1020_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0001_0101_0100_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0002_0408_1000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0001_0101_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0002_0408_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0001_0100_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0002_0400_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0001_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0002_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0200_0000_0000_0000), // h3
        Board(0x0600_0000_0000_0000), // h4
        Board(0x0e00_0000_0000_0000), // h5
        Board(0x1e00_0000_0000_0000), // h6
        Board(0x3e00_0000_0000_0000), // h7
        Board(0x7e00_0000_0000_0000), // h8
    ],
    // h2
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0002_0202_0202_0200), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0002_0202_0202_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0004_0810_2040_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0002_0202_0200_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0004_0810_2000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0002_0202_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0004_0810_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0002_0200_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0004_0800_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0002_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0004_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0000_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0400_0000_0000_0000), // h4
        Board(0x0c00_0000_0000_0000), // h5
        Board(0x1c00_0000_0000_0000), // h6
        Board(0x3c00_0000_0000_0000), // h7
        Board(0x7c00_0000_0000_0000), // h8
    ],
    // h3
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0004_0404_0404_0400), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0004_0404_0404_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0004_0404_0400_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0008_1020_4000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0004_0404_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0008_1020_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0004_0400_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0008_1000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0002_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0004_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0008_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0200_0000_0000_0000), // h1
        Board(0x0000_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0800_0000_0000_0000), // h5
        Board(0x1800_0000_0000_0000), // h6
        Board(0x3800_0000_0000_0000), // h7
        Board(0x7800_0000_0000_0000), // h8
    ],
    // h4
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0008_0808_0808_0800), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0008_0808_0808_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0008_0808_0800_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0008_0808_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0010_2040_0000_0000), // d8
        Board(0x0004_0200_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0008_0800_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0010_2000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0004_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0008_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0010_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0600_0000_0000_0000), // h1
        Board(0x0400_0000_0000_0000), // h2
        Board(0x0000_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x1000_0000_0000_0000), // h6
        Board(0x3000_0000_0000_0000), // h7
        Board(0x7000_0000_0000_0000), // h8
    ],
    // h5
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0010_1010_1010_1000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0010_1010_1010_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0010_1010_1000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0008_0402_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0010_1010_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0008_0400_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0010_1000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0020_4000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0008_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0010_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0020_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x0e00_0000_0000_0000), // h1
        Board(0x0c00_0000_0000_0000), // h2
        Board(0x0800_0000_0000_0000), // h3
        Board(0x0000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x2000_0000_0000_0000), // h7
        Board(0x6000_0000_0000_0000), // h8
    ],
    // h6
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0020_2020_2020_2000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0020_2020_2020_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0010_0804_0200_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0020_2020_2000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0010_0804_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0020_2020_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0010_0800_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0020_2000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0010_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0020_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0040_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x1e00_0000_0000_0000), // h1
        Board(0x1c00_0000_0000_0000), // h2
        Board(0x1800_0000_0000_0000), // h3
        Board(0x1000_0000_0000_0000), // h4
        Board(0x0000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x4000_0000_0000_0000), // h8
    ],
    // h7
    [
        Board(0x0000_0000_0000_0000), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0040_4040_4040_4000), // a7
        Board(0x0000_0000_0000_0000), // a8
        Board(0x0020_1008_0402_0000), // b1
        Board(0x0000_0000_0000_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0040_4040_4040_0000), // b7
        Board(0x0000_0000_0000_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0020_1008_0400_0000), // c2
        Board(0x0000_0000_0000_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0040_4040_4000_0000), // c7
        Board(0x0000_0000_0000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0020_1008_0000_0000), // d3
        Board(0x0000_0000_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0040_4040_0000_0000), // d7
        Board(0x0000_0000_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0020_1000_0000_0000), // e4
        Board(0x0000_0000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0040_4000_0000_0000), // e7
        Board(0x0000_0000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0020_0000_0000_0000), // f5
        Board(0x0000_0000_0000_0000), // f6
        Board(0x0040_0000_0000_0000), // f7
        Board(0x0000_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x3e00_0000_0000_0000), // h1
        Board(0x3c00_0000_0000_0000), // h2
        Board(0x3800_0000_0000_0000), // h3
        Board(0x3000_0000_0000_0000), // h4
        Board(0x2000_0000_0000_0000), // h5
        Board(0x0000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
    // h8
    [
        Board(0x0040_2010_0804_0200), // a1
        Board(0x0000_0000_0000_0000), // a2
        Board(0x0000_0000_0000_0000), // a3
        Board(0x0000_0000_0000_0000), // a4
        Board(0x0000_0000_0000_0000), // a5
        Board(0x0000_0000_0000_0000), // a6
        Board(0x0000_0000_0000_0000), // a7
        Board(0x0080_8080_8080_8000), // a8
        Board(0x0000_0000_0000_0000), // b1
        Board(0x0040_2010_0804_0000), // b2
        Board(0x0000_0000_0000_0000), // b3
        Board(0x0000_0000_0000_0000), // b4
        Board(0x0000_0000_0000_0000), // b5
        Board(0x0000_0000_0000_0000), // b6
        Board(0x0000_0000_0000_0000), // b7
        Board(0x0080_8080_8080_0000), // b8
        Board(0x0000_0000_0000_0000), // c1
        Board(0x0000_0000_0000_0000), // c2
        Board(0x0040_2010_0800_0000), // c3
        Board(0x0000_0000_0000_0000), // c4
        Board(0x0000_0000_0000_0000), // c5
        Board(0x0000_0000_0000_0000), // c6
        Board(0x0000_0000_0000_0000), // c7
        Board(0x0080_8080_8000_0000), // c8
        Board(0x0000_0000_0000_0000), // d1
        Board(0x0000_0000_0000_0000), // d2
        Board(0x0000_0000_0000_0000), // d3
        Board(0x0040_2010_0000_0000), // d4
        Board(0x0000_0000_0000_0000), // d5
        Board(0x0000_0000_0000_0000), // d6
        Board(0x0000_0000_0000_0000), // d7
        Board(0x0080_8080_0000_0000), // d8
        Board(0x0000_0000_0000_0000), // e1
        Board(0x0000_0000_0000_0000), // e2
        Board(0x0000_0000_0000_0000), // e3
        Board(0x0000_0000_0000_0000), // e4
        Board(0x0040_2000_0000_0000), // e5
        Board(0x0000_0000_0000_0000), // e6
        Board(0x0000_0000_0000_0000), // e7
        Board(0x0080_8000_0000_0000), // e8
        Board(0x0000_0000_0000_0000), // f1
        Board(0x0000_0000_0000_0000), // f2
        Board(0x0000_0000_0000_0000), // f3
        Board(0x0000_0000_0000_0000), // f4
        Board(0x0000_0000_0000_0000), // f5
        Board(0x0040_0000_0000_0000), // f6
        Board(0x0000_0000_0000_0000), // f7
        Board(0x0080_0000_0000_0000), // f8
        Board(0x0000_0000_0000_0000), // g1
        Board(0x0000_0000_0000_0000), // g2
        Board(0x0000_0000_0000_0000), // g3
        Board(0x0000_0000_0000_0000), // g4
        Board(0x0000_0000_0000_0000), // g5
        Board(0x0000_0000_0000_0000), // g6
        Board(0x0000_0000_0000_0000), // g7
        Board(0x0000_0000_0000_0000), // g8
        Board(0x7e00_0000_0000_0000), // h1
        Board(0x7c00_0000_0000_0000), // h2
        Board(0x7800_0000_0000_0000), // h3
        Board(0x7000_0000_0000_0000), // h4
        Board(0x6000_0000_0000_0000), // h5
        Board(0x4000_0000_0000_0000), // h6
        Board(0x0000_0000_0000_0000), // h7
        Board(0x0000_0000_0000_0000), // h8
    ],
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
        let a1 = Square::new(0);

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
            assert_eq!(iter.next(), Some(Square::new(i)));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterates_over_some() {
        let mut iter = Board(0x0001_0002_0004_0008).iter();
        assert_eq!(iter.next(), Some(Square::new(3)));
        assert_eq!(iter.next(), Some(Square::new(18)));
        assert_eq!(iter.next(), Some(Square::new(33)));
        assert_eq!(iter.next(), Some(Square::new(48)));
        assert_eq!(iter.next(), None);
    }
}
