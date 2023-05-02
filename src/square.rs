use std::fmt::Display;

use crate::board::{Board, DIAGONAL_RAYS, FILES, LINES_ALONG, SQUARES_BETWEEN, STRAIGHT_RAYS};

static SQUARES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

#[derive(Debug, PartialEq)]
pub struct Square(u8);

impl Square {
    pub fn between(&self, rhs: &Self) -> Board {
        SQUARES_BETWEEN[self.0 as usize][rhs.0 as usize]
    }

    pub fn diagonal_rays(&self) -> Board {
        DIAGONAL_RAYS[self.0 as usize]
    }

    pub fn file(&self) -> Board {
        FILES[self.0 as usize]
    }

    pub fn file_index(&self) -> usize {
        (self.0 % 8) as usize
    }

    pub fn lines_along(&self, rhs: &Self) -> Board {
        LINES_ALONG[self.0 as usize][rhs.0 as usize]
    }

    pub fn new(index: u8) -> Self {
        Self(index)
    }

    pub fn rank_index(&self) -> usize {
        self.0.div_floor(8) as usize
    }

    pub fn straight_rays(&self) -> Board {
        STRAIGHT_RAYS[self.0 as usize]
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn try_from_str(s: &str) -> Result<Self, String> {
        for (i, square) in SQUARES.iter().enumerate() {
            if s == *square {
                return Ok(Self(i as u8));
            }
        }

        Err(format!("Invalid square {s}"))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SQUARES[self.0 as usize])
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
            Err(String::from("Invalid square "))
        );
        assert_eq!(
            Square::try_from_str("-"),
            Err(String::from("Invalid square -"))
        );
        assert_eq!(
            Square::try_from_str("a9"),
            Err(String::from("Invalid square a9"))
        );
        assert_eq!(
            Square::try_from_str("i1"),
            Err(String::from("Invalid square i1"))
        );
    }
}
