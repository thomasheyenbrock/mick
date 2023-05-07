use crate::side::Side;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece(u8);

impl Piece {
    pub const WHITE_KING: Self = Self(0);
    pub const BLACK_KING: Self = Self(1);
    pub const WHITE_QUEEN: Self = Self(2);
    pub const BLACK_QUEEN: Self = Self(3);
    pub const WHITE_ROOK: Self = Self(4);
    pub const BLACK_ROOK: Self = Self(5);
    pub const WHITE_BISHOP: Self = Self(6);
    pub const BLACK_BISHOP: Self = Self(7);
    pub const WHITE_KNIGHT: Self = Self(8);
    pub const BLACK_KNIGHT: Self = Self(9);
    pub const WHITE_PAWN: Self = Self(10);
    pub const BLACK_PAWN: Self = Self(11);
    pub const NONE: Self = Self(12);

    pub fn is_some(&self) -> bool {
        self.0 <= 11
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn try_from_char(c: char) -> Result<Self, String> {
        match c {
            'K' => Ok(Self(0)),
            'k' => Ok(Self(1)),
            'Q' => Ok(Self(2)),
            'q' => Ok(Self(3)),
            'R' => Ok(Self(4)),
            'r' => Ok(Self(5)),
            'B' => Ok(Self(6)),
            'b' => Ok(Self(7)),
            'N' => Ok(Self(8)),
            'n' => Ok(Self(9)),
            'P' => Ok(Self(10)),
            'p' => Ok(Self(11)),
            _ => Err(format!("Invalid piece {c}")),
        }
    }
}

pub struct PieceKind(u8);

impl PieceKind {
    pub const KING: Self = Self(0);
    pub const QUEEN: Self = Self(1);
    pub const ROOK: Self = Self(2);
    pub const BISHOP: Self = Self(3);
    pub const KNIGHT: Self = Self(4);
    pub const PAWN: Self = Self(5);

    pub fn to_piece(&self, side: &Side) -> Piece {
        Piece(2 * self.0 + side.to_u8())
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::Piece;

    #[test]
    fn try_from_valid() {
        assert_eq!(Piece::try_from_char('K'), Ok(Piece::WHITE_KING));
        assert_eq!(Piece::try_from_char('k'), Ok(Piece::BLACK_KING));
        assert_eq!(Piece::try_from_char('Q'), Ok(Piece::WHITE_QUEEN));
        assert_eq!(Piece::try_from_char('q'), Ok(Piece::BLACK_QUEEN));
        assert_eq!(Piece::try_from_char('R'), Ok(Piece::WHITE_ROOK));
        assert_eq!(Piece::try_from_char('r'), Ok(Piece::BLACK_ROOK));
        assert_eq!(Piece::try_from_char('B'), Ok(Piece::WHITE_BISHOP));
        assert_eq!(Piece::try_from_char('b'), Ok(Piece::BLACK_BISHOP));
        assert_eq!(Piece::try_from_char('N'), Ok(Piece::WHITE_KNIGHT));
        assert_eq!(Piece::try_from_char('n'), Ok(Piece::BLACK_KNIGHT));
        assert_eq!(Piece::try_from_char('P'), Ok(Piece::WHITE_PAWN));
        assert_eq!(Piece::try_from_char('p'), Ok(Piece::BLACK_PAWN));
    }

    #[test]
    fn try_from_invalid() {
        assert_eq!(
            Piece::try_from_char('-'),
            Err(String::from("Invalid piece -"))
        );
        assert_eq!(
            Piece::try_from_char('1'),
            Err(String::from("Invalid piece 1"))
        );
    }
}
