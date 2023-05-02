use std::ops::BitAnd;

#[derive(Debug, Eq, PartialEq)]
pub struct Castle(u8);

impl Castle {
    pub const KINGSIDE: Self = Self(0);
    pub const QUEENSIDE: Self = Self(1);

    pub fn new(c: u8) -> Self {
        Self(c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const NO_RIGHTS: Self = Self(0b0000);
    pub const ALL_RIGHTS: Self = Self(0b1111);

    pub const WHITE: Self = Self(0b0011);
    pub const BLACK: Self = Self(0b1100);

    pub const WHITE_KINGSIDE: Self = Self(0b0001);
    pub const WHITE_QUEENSIDE: Self = Self(0b0010);
    pub const BLACK_KINGSIDE: Self = Self(0b0100);
    pub const BLACK_QUEENSIDE: Self = Self(0b1000);

    pub const NOT_WHITE_KINGSIDE: Self = Self(0b1110);
    pub const NOT_WHITE_QUEENSIDE: Self = Self(0b1101);
    pub const NOT_BLACK_KINGSIDE: Self = Self(0b1011);
    pub const NOT_BLACK_QUEENSIDE: Self = Self(0b0111);

    pub fn from_str(s: &str) -> Self {
        if s == "-" {
            return Self::NO_RIGHTS;
        }

        let mut i = 0;
        for c in s.chars() {
            match c {
                'K' => i ^= 0b0001,
                'Q' => i ^= 0b0010,
                'k' => i ^= 0b0100,
                'q' => i ^= 0b1000,
                _ => panic!("Invalid castling rights {s}"),
            }
        }
        Self(i)
    }

    pub fn iter(&self) -> CastlingRightsIterator {
        CastlingRightsIterator(self.0)
    }
}

impl BitAnd for CastlingRights {
    type Output = CastlingRights;

    fn bitand(self, rhs: Self) -> Self::Output {
        CastlingRights(self.0 & rhs.0)
    }
}

pub struct CastlingRightsIterator(u8);

impl Iterator for CastlingRightsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let trailing = self.0.trailing_zeros() as usize;
            self.0 ^= 1 << trailing;
            Some(trailing)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::castle::CastlingRights;

    #[test]
    fn from_valid() {
        assert_eq!(CastlingRights::from_str("-"), CastlingRights::NO_RIGHTS);
        assert_eq!(
            CastlingRights::from_str("K"),
            CastlingRights::WHITE_KINGSIDE
        );
        assert_eq!(
            CastlingRights::from_str("Q"),
            CastlingRights::WHITE_QUEENSIDE
        );
        assert_eq!(
            CastlingRights::from_str("k"),
            CastlingRights::BLACK_KINGSIDE
        );
        assert_eq!(
            CastlingRights::from_str("q"),
            CastlingRights::BLACK_QUEENSIDE
        );
        assert_eq!(CastlingRights::from_str("KkQq"), CastlingRights::ALL_RIGHTS);
    }

    #[test]
    #[should_panic(expected = "Invalid castling rights a")]
    fn from_invalid() {
        CastlingRights::from_str("a");
    }
}
