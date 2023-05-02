use std::fmt::Display;

use crate::square::Square;

/// The first two bytes of the first number and the first two bytes of the second number encode metadata about the move
/// that was performed:
/// - 00 01: A piece was captured
/// - 00 11: A pawn was captured en-passant
/// - 10 00: Promotion to a queen
/// - 10 01: Promotion to a queen with a capture
/// - 10 10: Promotion to a rook
/// - 10 11: Promotion to a rook with a capture
/// - 11 00: Promotion to a bishop
/// - 11 01: Promotion to a bishop with a capture
/// - 11 10: Promotion to a knight
/// - 11 11: Promotion to a knight with a capture
/// - 00 10: Kingside castle
/// - 01 00: Queenside castle
/// - 00 00: None of the above (i.e. just a "regular" move)
///
/// These numbers were chosen so that:
/// - The last bit always indicates if a piece was captured
/// - The first bit always indicates if the move was a promotion
/// - Ideally there would be a single byte that always indicates if the move was a castle, but this is impossible
///   with the previous two statements being true, so we chose the remaining two single-byte numbers instead
///
/// The following numbers are missing from the list and thus are invalid:
/// - 01 01
/// - 01 10
/// - 01 11
#[derive(Debug)]
pub struct Move(
    /// The last 6 bits indicate the square from which a piece was moved
    u8,
    /// The last 6 bits indicate the square to which a piece was moved
    u8,
);

impl Move {
    pub fn new_capture(from: &Square, to: &Square) -> Self {
        Self(from.to_u8(), to.to_u8() | 0b01_000000)
    }

    pub fn new_capture_promotion(from: &Square, to: &Square) -> Vec<Self> {
        vec![
            Self(from.to_u8() | 0b10_000000, to.to_u8() | 0b01_000000),
            Self(from.to_u8() | 0b10_000000, to.to_u8() | 0b11_000000),
            Self(from.to_u8() | 0b11_000000, to.to_u8() | 0b01_000000),
            Self(from.to_u8() | 0b11_000000, to.to_u8() | 0b11_000000),
        ]
    }

    pub fn new_push(from: &Square, to: &Square) -> Self {
        Self(from.to_u8(), to.to_u8())
    }

    pub fn new_push_promotion(from: &Square, to: &Square) -> Vec<Self> {
        vec![
            Self(from.to_u8() | 0b10_000000, to.to_u8()),
            Self(from.to_u8() | 0b10_000000, to.to_u8() | 0b10_000000),
            Self(from.to_u8() | 0b11_000000, to.to_u8()),
            Self(from.to_u8() | 0b11_000000, to.to_u8() | 0b10_000000),
        ]
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            Square::new(self.0 & 63),
            Square::new(self.1 & 63),
            if self.0 & 0b10_000000 == 0 {
                ""
            } else {
                "TODO: promotion piece"
            }
        )
    }
}
