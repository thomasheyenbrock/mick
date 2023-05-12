use crate::side::Side;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Castle(pub u8);

pub const KING_SIDE: Castle = Castle(0);
pub const QUEEN_SIDE: Castle = Castle(1);

impl Display for Castle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == QUEEN_SIDE { "O-O-O" } else { "O-O" })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastlingRights(pub u8);

pub const NO_RIGHTS: CastlingRights = CastlingRights(0);
pub const ALL_RIGHTS: CastlingRights = CastlingRights(0b1111);
pub const WHITE_RIGHTS: CastlingRights = CastlingRights(0b0011);
pub const WHITE_KING_SIDE: CastlingRights = CastlingRights(0b0001);
pub const WHITE_QUEEN_SIDE: CastlingRights = CastlingRights(0b0010);
pub const BLACK_KING_SIDE: CastlingRights = CastlingRights(0b0100);
pub const BLACK_QUEEN_SIDE: CastlingRights = CastlingRights(0b1000);

impl CastlingRights {
    pub fn clear(&mut self, rights: CastlingRights) {
        self.0 &= !rights.0;
    }

    pub fn clear_side(&mut self, side: Side) {
        let rights = WHITE_RIGHTS.0 << side.0;
        self.0 &= !rights;
    }

    pub fn has(self, castle: Castle, side: Side) -> bool {
        self.0 & (1 << (2 * side.0 + castle.0)) != 0
    }

    pub fn try_from_str(s: &str) -> Result<CastlingRights, String> {
        if s == "-" {
            return Ok(NO_RIGHTS);
        }

        let mut rights = 0;
        for c in s.chars() {
            match c {
                'K' => rights |= 0b0001,
                'Q' => rights |= 0b0010,
                'k' => rights |= 0b0100,
                'q' => rights |= 0b1000,
                _ => return Err(format!("Invalid castling rights: {}", c)),
            }
        }
        Ok(CastlingRights(rights))
    }
}

const DISPLAY: [&str; 16] = [
    "-", "K", "Q", "KQ", "k", "Kk", "Qk", "KQk", "q", "Kq", "Qq", "KQq", "kq", "Kkq", "Qkq", "KQkq",
];

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DISPLAY[self.0 as usize])
    }
}

#[cfg(test)]
mod tests {
    use crate::castle::{
        CastlingRights, ALL_RIGHTS, BLACK_KING_SIDE, BLACK_QUEEN_SIDE, NO_RIGHTS, WHITE_KING_SIDE,
        WHITE_QUEEN_SIDE,
    };

    #[test]
    fn from_valid() {
        assert_eq!(CastlingRights::try_from_str("-"), Ok(NO_RIGHTS));
        assert_eq!(CastlingRights::try_from_str("K"), Ok(WHITE_KING_SIDE));
        assert_eq!(CastlingRights::try_from_str("Q"), Ok(WHITE_QUEEN_SIDE));
        assert_eq!(CastlingRights::try_from_str("k"), Ok(BLACK_KING_SIDE));
        assert_eq!(CastlingRights::try_from_str("q"), Ok(BLACK_QUEEN_SIDE));
        assert_eq!(CastlingRights::try_from_str("KkQq"), Ok(ALL_RIGHTS));
    }

    #[test]
    fn from_invalid() {
        assert_eq!(
            CastlingRights::try_from_str("a"),
            Err(String::from("Invalid castling rights: a"))
        );
    }
}
