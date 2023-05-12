mod consts;
mod rays;

use self::consts::{
    DIAGONAL_RAYS, KING_MOVES, KNIGHT_MOVES, LINES_ALONG, SQUARES_BETWEEN, STRAIGHT_RAYS,
};
use crate::board::{Board, FILES};
use std::fmt::Display;

static NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub u8);

pub const A1: Square = Square(0);
pub const C1: Square = Square(2);
pub const D1: Square = Square(3);
pub const E1: Square = Square(4);
pub const F1: Square = Square(5);
pub const G1: Square = Square(6);
pub const H1: Square = Square(7);
pub const A8: Square = Square(56);
pub const C8: Square = Square(58);
pub const D8: Square = Square(59);
pub const E8: Square = Square(60);
pub const F8: Square = Square(61);
pub const G8: Square = Square(62);
pub const H8: Square = Square(63);

impl Square {
    pub fn along_row_with_col(self, other: Square) -> Square {
        Square((self.0 & 56) | (other.0 & 7))
    }

    pub fn between(self, rhs: Self) -> Board {
        SQUARES_BETWEEN[self.0 as usize][rhs.0 as usize]
    }

    pub fn diagonal_rays(self) -> Board {
        DIAGONAL_RAYS[self.0 as usize]
    }

    pub fn file_index(self) -> u8 {
        self.0 & 7
    }

    pub fn file_mask(self) -> Board {
        FILES[self.0 as usize]
    }

    pub fn from(rank: u8, file: u8) -> Square {
        Square(rank * 8 + file)
    }

    pub fn king_moves(self) -> Board {
        unsafe { *KING_MOVES.get_unchecked(self.0 as usize) }
    }

    pub fn knight_moves(self) -> Board {
        unsafe { *KNIGHT_MOVES.get_unchecked(self.0 as usize) }
    }

    pub fn lines_along(self, rhs: Self) -> Board {
        LINES_ALONG[self.0 as usize][rhs.0 as usize]
    }

    pub fn rank_index(self) -> usize {
        self.0.div_floor(8) as usize
    }

    pub fn rotate_right(self, amount: u8) -> Square {
        Square((self.0 + (64 - amount)) & 63)
    }

    pub fn straight_rays(self) -> Board {
        STRAIGHT_RAYS[self.0 as usize]
    }

    pub fn to_board(self) -> Board {
        Board(1 << self.0)
    }

    pub fn try_from_str(s: &str) -> Result<Self, String> {
        for (i, square) in NAMES.iter().enumerate() {
            if s == *square {
                return Ok(Self(i as u8));
            }
        }

        Err(format!("Invalid square: {s}"))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAMES[self.0 as usize])
    }
}

#[cfg(test)]
mod tests {
    use crate::square::Square;

    #[test]
    fn try_from_valid() {
        assert_eq!(Square::try_from_str("a1"), Ok(Square(0)));
        assert_eq!(Square::try_from_str("b1"), Ok(Square(1)));
        assert_eq!(Square::try_from_str("h1"), Ok(Square(7)));
        assert_eq!(Square::try_from_str("d4"), Ok(Square(27)));
        assert_eq!(Square::try_from_str("a8"), Ok(Square(56)));
        assert_eq!(Square::try_from_str("h8"), Ok(Square(63)));
    }

    #[test]
    fn try_from_invalid() {
        assert_eq!(
            Square::try_from_str(""),
            Err(String::from("Invalid square: "))
        );
        assert_eq!(
            Square::try_from_str("-"),
            Err(String::from("Invalid square: -"))
        );
        assert_eq!(
            Square::try_from_str("a9"),
            Err(String::from("Invalid square: a9"))
        );
        assert_eq!(
            Square::try_from_str("i1"),
            Err(String::from("Invalid square: i1"))
        );
    }
}
