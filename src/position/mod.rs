mod zorbist;

use crate::{
    board::Board, castling_rights::CastlingRights, piece::Piece, side::Side, square::Square,
};

use self::zorbist::Zorbist;

#[derive(Debug, PartialEq)]
pub struct Position {
    piece_boards: [Board; 12],
    side_to_move: Side,
    castling_rights: CastlingRights,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
    hash: u64,
}

impl Position {
    pub const STARTING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split(" ").collect();
        assert!(parts.len() == 6, "Invalid fen {fen}");

        let mut piece_boards = [Board::EMPTY; 12];
        for (rank_index, rank) in parts[0].split("/").enumerate() {
            let mut file_index = 0;
            for char in rank.chars() {
                if let Ok(piece) = Piece::try_from_char(char) {
                    let board = &mut piece_boards[piece.to_usize()];
                    let square_index = (7 - rank_index) * 8 + file_index;
                    board.flip(&Square::new(square_index as u8));
                    file_index += 1;
                } else if let Some(digit) = char.to_digit(10) {
                    file_index += digit as usize;
                } else {
                    panic!("Invalid fen {fen}")
                }
            }
        }

        let side = Side::from_str(unsafe { parts.get_unchecked(1) });
        let castling_rights = CastlingRights::from_str(unsafe { parts.get_unchecked(2) });
        let en_passant_target = Square::try_from_str(unsafe { parts.get_unchecked(3) }).ok();

        let raw_halfmove_clock = unsafe { parts.get_unchecked(4) };
        let raw_fullmove_number = unsafe { parts.get_unchecked(5) };

        let halfmove_clock = (*raw_halfmove_clock)
            .parse::<u32>()
            .expect(&format!("Invalid halfmove clock {raw_halfmove_clock}"));
        let fullmove_number = (*raw_fullmove_number)
            .parse::<u32>()
            .expect(&format!("Invalid fullmove number {raw_fullmove_number}"));

        Self::new(
            piece_boards,
            side,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        )
    }

    pub fn new(
        piece_boards: [Board; 12],
        side_to_move: Side,
        castling_rights: CastlingRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        fullmove_number: u32,
    ) -> Self {
        let hash = Zorbist::DEFAULT.hash(
            &piece_boards,
            &side_to_move,
            &castling_rights,
            &en_passant_target,
        );
        Self {
            piece_boards,
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, castling_rights::CastlingRights, position::Position, side::Side};

    #[test]
    fn parses_fen_starting_position() {
        let p = Position::from_fen(Position::STARTING);

        assert_eq!(
            p,
            Position {
                piece_boards: [
                    Board::new(0x0000_0000_0000_0010),
                    Board::new(0x1000_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0008),
                    Board::new(0x0800_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0081),
                    Board::new(0x8100_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0024),
                    Board::new(0x2400_0000_0000_0000),
                    Board::new(0x0000_0000_0000_0042),
                    Board::new(0x4200_0000_0000_0000),
                    Board::new(0x0000_0000_0000_FF00),
                    Board::new(0x00FF_0000_0000_0000),
                ],
                side_to_move: Side::WHITE,
                castling_rights: CastlingRights::ALL_RIGHTS,
                en_passant_target: None,
                halfmove_clock: 0,
                fullmove_number: 1,
                hash: 1278498509228713946
            }
        );
    }
}
