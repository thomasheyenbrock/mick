use std::fmt::Display;

use crate::{castle::Castle, piece::PieceKind, square::Square};

/// The first two bytes of the first number and the first two bytes of the second number encode metadata about the move
/// that was performed:
/// - 01 00: A piece was captured
/// - 01 01: A pawn was captured en-passant
/// - 00 01: Double pawn push
/// - 00 10: Kingside castle
/// - 00 11: Queenside castle
/// - 10 00: Promotion to a queen
/// - 10 01: Promotion to a rook
/// - 10 10: Promotion to a bishop
/// - 10 11: Promotion to a knight
/// - 11 00: Promotion to a queen with a capture
/// - 11 01: Promotion to a rook with a capture
/// - 11 10: Promotion to a bishop with a capture
/// - 11 11: Promotion to a knight with a capture
/// - 00 00: None of the above (i.e. just a "regular" move)
///
/// These numbers were chosen so that:
/// - The last bit always indicates if a piece was captured
/// - The first bit always indicates if the move was a promotion
/// - Ideally there would be a single byte that always indicates if the move was a castle, but this is impossible
///   with the previous two statements being true. The closest solution is choosing the first bit being zero and
///   the second bit being one.
/// - The only remaining number with a trailing zero was chosen to indicate a double pawn push
///
/// The following numbers are missing from the list and thus are invalid:
/// - 01 10
/// - 01 11
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(packed(2))] // packed since often stored in transposition tables
pub struct Move(u8, u8);

impl Move {
    pub fn from(self) -> Square {
        Square(self.0 & 0b00_111111)
    }

    pub fn to(self) -> Square {
        Square(self.1 & 0b00_111111)
    }

    pub fn castle(self) -> Option<Castle> {
        if self.0 & 0b11_000000 == 0 && self.1 & 0b10_000000 == 0b10_000000 {
            Some(Castle((self.1 & 0b01_000000) >> 6))
        } else {
            None
        }
    }

    pub fn promote_to(self) -> Option<PieceKind> {
        if self.0 & 0b10_000000 == 0 {
            None
        } else {
            Some(PieceKind(((self.1 & 0b11_000000) >> 6) + 1))
        }
    }

    pub fn is_capture(self) -> bool {
        self.0 & 0b01_000000 == 0b01_000000
    }

    pub fn is_double_pawn_push(self) -> bool {
        self.0 & 0b11_000000 == 0 && self.1 & 0b11_000000 == 0b01_000000
    }

    pub fn is_en_passant_capture(self) -> bool {
        self.0 & 0b01_000000 == 0b01_000000 && self.1 & 0b01_000000 == 0b01_000000
    }

    pub fn new_capture(from: Square, to: Square) -> Move {
        Move(from.0 | 0b01_000000, to.0)
    }

    pub fn new_capture_en_passant(from: Square, to: Square) -> Move {
        Move(from.0 | 0b01_000000, to.0 | 0b01_000000)
    }

    pub fn new_capture_promotion(from: Square, to: Square, promote_to: PieceKind) -> Move {
        Move(from.0 | 0b11_000000, to.0 | ((promote_to.0 - 1) << 6))
    }

    pub fn new_castle(from: Square, to: Square, castle: Castle) -> Move {
        Move(from.0, to.0 | 0b10_000000 | (castle.0 << 6))
    }

    pub fn new_push(from: Square, to: Square) -> Move {
        Move(from.0, to.0)
    }

    pub fn new_push_double_pawn(from: Square, to: Square) -> Move {
        Move(from.0, to.0 | 0b01_000000)
    }

    pub fn new_push_promotion(from: Square, to: Square, promote_to: PieceKind) -> Move {
        Move(from.0 | 0b10_000000, to.0 | ((promote_to.0 - 1) << 6))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(castle) = self.castle() {
            return write!(f, "{}", castle);
        }

        let mut s = String::new();

        s += &self.from().to_string();

        if self.is_capture() {
            s.push('x');
        }

        s += &self.to().to_string();

        if let Some(kind) = self.promote_to() {
            s.push('=');
            s.push_str(&format!("{}", kind));
        }

        if self.is_en_passant_capture() {
            s += "e.p."
        }

        write!(f, "{}", &s)
    }
}
