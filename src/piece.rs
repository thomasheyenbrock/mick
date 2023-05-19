use crate::side::Side;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PieceKind(pub u8);

pub const KING: PieceKind = PieceKind(0);
pub const QUEEN: PieceKind = PieceKind(1);
pub const ROOK: PieceKind = PieceKind(2);
pub const BISHOP: PieceKind = PieceKind(3);
pub const KNIGHT: PieceKind = PieceKind(4);
pub const PAWN: PieceKind = PieceKind(5);

impl PieceKind {
    pub fn to_piece(self, side: Side) -> Piece {
        Piece((self.0 << 1) | side.0)
    }

    pub fn try_from_char(c: char) -> Result<Self, String> {
        match c {
            'K' | 'k' => Ok(KING),
            'Q' | 'q' => Ok(QUEEN),
            'R' | 'r' => Ok(ROOK),
            'B' | 'b' => Ok(BISHOP),
            'N' | 'n' => Ok(KNIGHT),
            'P' | 'p' => Ok(PAWN),
            _ => Err(format!("Invalid piece kind: {}", c)),
        }
    }
}

impl Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CHARS[(self.0 as usize) << 1])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece(pub u8);

#[cfg(test)]
pub const WHITE_KING: Piece = Piece(0);
#[cfg(test)]
pub const BLACK_KING: Piece = Piece(1);
pub const WHITE_QUEEN: Piece = Piece(2);
pub const BLACK_QUEEN: Piece = Piece(3);
pub const WHITE_ROOK: Piece = Piece(4);
pub const BLACK_ROOK: Piece = Piece(5);
pub const WHITE_BISHOP: Piece = Piece(6);
pub const BLACK_BISHOP: Piece = Piece(7);
pub const WHITE_KNIGHT: Piece = Piece(8);
pub const BLACK_KNIGHT: Piece = Piece(9);
pub const WHITE_PAWN: Piece = Piece(10);
pub const BLACK_PAWN: Piece = Piece(11);
pub const NULL_PIECE: Piece = Piece(12);

impl Piece {
    pub fn is_slider(&self) -> bool {
        self.0 <= 7 && self.0 >= 2
    }

    pub fn is_some(&self) -> bool {
        *self != NULL_PIECE
    }

    pub fn kind(&self) -> PieceKind {
        PieceKind(self.0 >> 1)
    }

    pub fn side(&self) -> Side {
        Side(self.0 & 1)
    }

    pub fn to_char(&self) -> char {
        CHARS[self.0 as usize]
    }

    pub fn to_symbol(&self) -> Option<char> {
        if self.0 >= 12 {
            None
        } else {
            Some(SYMBOLS[self.0 as usize])
        }
    }

    pub fn try_from_char(c: char) -> Result<Piece, String> {
        match c {
            'K' => Ok(Piece(0)),
            'k' => Ok(Piece(1)),
            'Q' => Ok(Piece(2)),
            'q' => Ok(Piece(3)),
            'R' => Ok(Piece(4)),
            'r' => Ok(Piece(5)),
            'B' => Ok(Piece(6)),
            'b' => Ok(Piece(7)),
            'N' => Ok(Piece(8)),
            'n' => Ok(Piece(9)),
            'P' => Ok(Piece(10)),
            'p' => Ok(Piece(11)),
            _ => Err(format!("Invalid piece: {}", c)),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

const CHARS: [char; 13] = [
    'K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p', ' ',
];

const SYMBOLS: [char; 12] = ['♔', '♚', '♕', '♛', '♖', '♜', '♗', '♝', '♘', '♞', '♙', '♟'];

#[cfg(test)]
mod tests {
    use crate::piece::{
        Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
        WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    };

    #[test]
    fn try_from_valid() {
        assert_eq!(Piece::try_from_char('K'), Ok(WHITE_KING));
        assert_eq!(Piece::try_from_char('k'), Ok(BLACK_KING));
        assert_eq!(Piece::try_from_char('Q'), Ok(WHITE_QUEEN));
        assert_eq!(Piece::try_from_char('q'), Ok(BLACK_QUEEN));
        assert_eq!(Piece::try_from_char('R'), Ok(WHITE_ROOK));
        assert_eq!(Piece::try_from_char('r'), Ok(BLACK_ROOK));
        assert_eq!(Piece::try_from_char('B'), Ok(WHITE_BISHOP));
        assert_eq!(Piece::try_from_char('b'), Ok(BLACK_BISHOP));
        assert_eq!(Piece::try_from_char('N'), Ok(WHITE_KNIGHT));
        assert_eq!(Piece::try_from_char('n'), Ok(BLACK_KNIGHT));
        assert_eq!(Piece::try_from_char('P'), Ok(WHITE_PAWN));
        assert_eq!(Piece::try_from_char('p'), Ok(BLACK_PAWN));
    }

    #[test]
    fn try_from_invalid() {
        assert_eq!(
            Piece::try_from_char('-'),
            Err(String::from("Invalid piece: -"))
        );
        assert_eq!(
            Piece::try_from_char('1'),
            Err(String::from("Invalid piece: 1"))
        );
    }
}
