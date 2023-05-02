use std::fmt::Display;

use crate::board::{Board, DIAGONAL_RAYS, SQUARES_BETWEEN, STRAIGHT_RAYS};

static SQUARES: [&str; 64] = [
    "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8",
    "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8",
    "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8",
    "g1", "g2", "g3", "g4", "g5", "g6", "g7", "g8", "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8",
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

    pub fn file(&self) -> usize {
        (self.0 % 8) as usize
    }

    pub fn new(index: u8) -> Self {
        Self(index)
    }

    pub fn straight_rays(&self) -> Board {
        STRAIGHT_RAYS[self.0 as usize]
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
        assert_eq!(Square::try_from_str("a1"), Ok(Square::new(0)));
        assert_eq!(Square::try_from_str("a2"), Ok(Square::new(1)));
        assert_eq!(Square::try_from_str("a8"), Ok(Square::new(7)));
        assert_eq!(Square::try_from_str("d4"), Ok(Square::new(27)));
        assert_eq!(Square::try_from_str("h1"), Ok(Square::new(56)));
        assert_eq!(Square::try_from_str("h8"), Ok(Square::new(63)));
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
