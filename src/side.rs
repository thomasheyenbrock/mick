#[derive(Debug, PartialEq)]
pub struct Side(u8);

impl Side {
    pub const WHITE: Self = Self(0);
    pub const BLACK: Self = Self(1);

    pub fn from_str(s: &str) -> Self {
        match s {
            "w" => Side(0),
            "b" => Side(1),
            _ => panic!("Invalid side {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::side::Side;

    #[test]
    fn from_valid() {
        assert_eq!(Side::from_str("w"), Side::WHITE);
        assert_eq!(Side::from_str("b"), Side::BLACK);
    }

    #[test]
    #[should_panic(expected = "Invalid side -")]
    fn from_invalid() {
        Side::from_str("-");
    }
}
