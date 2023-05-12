use std::{fmt::Display, ops::Not};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Side(pub u8);

pub const WHITE: Side = Side(0);
pub const BLACK: Side = Side(1);

impl Side {
    pub fn try_from_str(s: &str) -> Result<Side, String> {
        match s {
            "w" => Ok(WHITE),
            "b" => Ok(BLACK),
            _ => Err(format!("Invalid side: {}", s)),
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CHARS[self.0 as usize])
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        Side(1 - self.0)
    }
}

const CHARS: [char; 2] = ['w', 'b'];

#[cfg(test)]
mod tests {
    use crate::side::{Side, BLACK, WHITE};

    #[test]
    fn from_valid() {
        assert_eq!(Side::try_from_str("w"), Ok(WHITE));
        assert_eq!(Side::try_from_str("b"), Ok(BLACK));
    }

    #[test]
    fn from_invalid() {
        assert_eq!(
            Side::try_from_str("-"),
            Err(String::from("Invalid side: -"))
        );
    }

    #[test]
    fn not() {
        assert_eq!(!WHITE, BLACK);
        assert_eq!(!BLACK, WHITE);
        assert_eq!(!!WHITE, WHITE);
        assert_eq!(!!BLACK, BLACK);
    }
}
