use std::ops::Not;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        Side(1 - self.0)
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

    #[test]
    fn not() {
        assert_eq!(!Side::WHITE, Side::BLACK);
        assert_eq!(!Side::BLACK, Side::WHITE);
        assert_eq!(!!Side::WHITE, Side::WHITE);
        assert_eq!(!!Side::BLACK, Side::BLACK);
    }
}
