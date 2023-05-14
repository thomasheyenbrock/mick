mod consts;
mod rays;

use self::consts::{
    DIAGONAL_RAYS, KING_MOVES, KNIGHT_MOVES, LINES_ALONG, SQUARES_BETWEEN, STRAIGHT_RAYS,
};
use crate::board::{Board, FILE_A};
use std::fmt::Display;

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

    pub fn between(self, other: Square) -> Board {
        unsafe {
            *SQUARES_BETWEEN
                .get_unchecked(self.0 as usize)
                .get_unchecked(other.0 as usize)
        }
    }

    pub fn diagonal_rays(self) -> Board {
        unsafe { *DIAGONAL_RAYS.get_unchecked(self.0 as usize) }
    }

    pub fn file_index(self) -> u8 {
        self.0 & 7
    }

    pub fn file_mask(self) -> Board {
        FILE_A << (self.0 & 7)
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

    pub fn lines_along(self, other: Square) -> Board {
        unsafe {
            *LINES_ALONG
                .get_unchecked(self.0 as usize)
                .get_unchecked(other.0 as usize)
        }
    }

    pub fn rank_index(self) -> u8 {
        self.0 >> 3
    }

    pub fn rotate_right(self, amount: u8) -> Square {
        Square((self.0 + (64 - amount)) & 63)
    }

    pub fn straight_rays(self) -> Board {
        unsafe { *STRAIGHT_RAYS.get_unchecked(self.0 as usize) }
    }

    pub fn try_from_str(s: &str) -> Result<Option<Square>, String> {
        if s == "-" {
            return Ok(None);
        }

        if s.len() < 2 {
            return Err(format!("Invalid square: {}", s));
        }

        let file_char = s.chars().next().unwrap() as usize;
        let rank_char = s.chars().nth(1).unwrap() as usize;

        let file = file_char - 'a' as usize;
        let rank = rank_char - '1' as usize;

        if file > 7 {
            return Err(format!("Invalid file: {}", s));
        }

        if rank > 7 {
            return Err(format!("Invalid rank: {}", s));
        }

        Ok(Some(Square::from(rank as u8, file as u8)))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAMES[self.0 as usize])
    }
}

const NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

#[cfg(test)]
mod tests {
    use crate::square::Square;

    #[test]
    fn try_from_valid() {
        assert_eq!(Square::try_from_str("a1"), Ok(Some(Square(0))));
        assert_eq!(Square::try_from_str("b1"), Ok(Some(Square(1))));
        assert_eq!(Square::try_from_str("h1"), Ok(Some(Square(7))));
        assert_eq!(Square::try_from_str("d4"), Ok(Some(Square(27))));
        assert_eq!(Square::try_from_str("a8"), Ok(Some(Square(56))));
        assert_eq!(Square::try_from_str("h8"), Ok(Some(Square(63))));
        assert_eq!(Square::try_from_str("-"), Ok(None));
    }

    #[test]
    fn try_from_invalid() {
        assert_eq!(
            Square::try_from_str(""),
            Err(String::from("Invalid square: "))
        );
        assert_eq!(
            Square::try_from_str("a9"),
            Err(String::from("Invalid rank: a9"))
        );
        assert_eq!(
            Square::try_from_str("i1"),
            Err(String::from("Invalid file: i1"))
        );
    }
}
