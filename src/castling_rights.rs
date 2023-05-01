#[derive(Debug, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const NO_RIGHTS: Self = Self(0);
    pub const ALL_RIGHTS: Self = Self(15);

    pub const WHITE_KINGSIDE: Self = Self(1);
    pub const WHITE_QUEENSIDE: Self = Self(2);
    pub const BLACK_KINGSIDE: Self = Self(4);
    pub const BLACK_QUEENSIDE: Self = Self(8);

    pub fn from_str(s: &str) -> Self {
        if s == "-" {
            return Self::NO_RIGHTS;
        }

        let mut i = 0;
        for c in s.chars() {
            match c {
                'K' => i ^= 1,
                'Q' => i ^= 2,
                'k' => i ^= 4,
                'q' => i ^= 8,
                _ => panic!("Invalid castling rights {s}"),
            }
        }
        Self(i)
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::castling_rights::CastlingRights;

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
