use std::fmt::Display;

use crate::{
    castle::Castle,
    piece::{PieceKind, BISHOP, KNIGHT, QUEEN, ROOK},
    square::Square,
};

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
/// - 00 10: Double pawn push
/// - 01 00: Kingside castle
/// - 01 10: Queenside castle
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
/// - 01 01
/// - 01 11
#[derive(Debug)]
pub struct Move(
    /// The last 6 bits indicate the square from which a piece was moved
    u8,
    /// The last 6 bits indicate the square to which a piece was moved
    u8,
);

impl Move {
    pub fn all_capture_promotions(from: &Square, to: &Square) -> Vec<Self> {
        vec![
            Self(from.0 | 0b10_000000, to.0 | 0b01_000000),
            Self(from.0 | 0b10_000000, to.0 | 0b11_000000),
            Self(from.0 | 0b11_000000, to.0 | 0b01_000000),
            Self(from.0 | 0b11_000000, to.0 | 0b11_000000),
        ]
    }

    pub fn all_push_promotions(from: &Square, to: &Square) -> Vec<Self> {
        vec![
            Self(from.0 | 0b10_000000, to.0),
            Self(from.0 | 0b10_000000, to.0 | 0b10_000000),
            Self(from.0 | 0b11_000000, to.0),
            Self(from.0 | 0b11_000000, to.0 | 0b10_000000),
        ]
    }

    pub fn castle(&self) -> Option<Castle> {
        if self.0 & 0b11_000000 == 0b01_000000 {
            Some(Castle((self.1 & 0b10_000000) >> 7))
        } else {
            None
        }
    }

    pub fn from(&self) -> Square {
        Square(self.0 & 0b00_111111)
    }

    pub fn is_double_pawn_push(&self) -> bool {
        (self.0 & 0b11_000000 == 0b00_000000) && (self.1 & 0b11_000000 == 0b10_000000)
    }

    pub fn is_en_passant_capture(&self) -> bool {
        (self.0 & 0b10_000000 == 0) && (self.1 & 0b11_000000 == 0b11_000000)
    }

    pub fn new_castle(from: &Square, to: &Square, castle: &Castle) -> Self {
        Self(from.0 | 0b01_000000, to.0 | (castle.0 << 7))
    }

    pub fn new_capture(from: &Square, to: &Square) -> Self {
        Self(from.0, to.0 | 0b01_000000)
    }
    pub fn new_capture_en_passant(from: &Square, to: &Square) -> Self {
        Self(from.0, to.0 | 0b11_000000)
    }

    pub fn new_capture_promotion(from: &Square, to: &Square, piece: &PieceKind) -> Self {
        let (flag_1, flag_2) = FLAG_FOR_PROMOTION_PIECE[piece.0 as usize];
        Self(from.0 | flag_1, to.0 | flag_2 | 0b01_000000)
    }

    pub fn new_push(from: &Square, to: &Square) -> Self {
        Self(from.0, to.0)
    }

    pub fn new_push_double_pawn(from: &Square, to: &Square) -> Self {
        Self(from.0, to.0 | 0b10_000000)
    }

    pub fn new_push_promotion(from: &Square, to: &Square, piece: &PieceKind) -> Self {
        let (flag_1, flag_2) = FLAG_FOR_PROMOTION_PIECE[piece.0 as usize];
        Self(from.0 | flag_1, to.0 | flag_2)
    }

    pub fn promotion_piece_kind(&self) -> Option<PieceKind> {
        if self.0 & 0b10_000000 == 0 {
            None
        } else {
            match (self.0 & 0b01_000000, self.1 & 0b10_000000) {
                (0b00_000000, 0b00_000000) => Some(QUEEN),
                (0b00_000000, 0b10_000000) => Some(ROOK),
                (0b01_000000, 0b00_000000) => Some(BISHOP),
                (0b01_000000, 0b10_000000) => Some(KNIGHT),
                _ => unreachable!(),
            }
        }
    }

    pub fn to(&self) -> Square {
        Square(self.1 & 0b00_111111)
    }
}

static FLAG_FOR_PROMOTION_PIECE: [(u8, u8); 6] = [
    (0, 0),
    (0b10_000000, 0b00_000000),
    (0b10_000000, 0b10_000000),
    (0b11_000000, 0b00_000000),
    (0b11_000000, 0b10_000000),
    (0, 0),
];

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            Square(self.0 & 63),
            Square(self.1 & 63),
            if self.0 & 0b10_000000 == 0 {
                ""
            } else {
                "TODO: promotion piece"
            }
        )
    }
}
