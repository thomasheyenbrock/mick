use crate::{board::Board, castle::CastlingRights, piece::Piece, side::Side, square::Square};

use super::Position;

impl Position {
    pub const STARTING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split(" ").collect();
        assert!(parts.len() == 6, "Invalid FEN {fen}");

        let mut pieces = [Piece::NONE; 64];
        let mut piece_boards = [Board::EMPTY; 12];
        for (rank_index, rank) in parts[0].split("/").enumerate() {
            let mut file_index = 0;
            for char in rank.chars() {
                if let Ok(piece) = Piece::try_from_char(char) {
                    let square_index = (7 - rank_index) * 8 + file_index;
                    pieces[square_index] = piece;

                    let board = &mut piece_boards[piece.0 as usize];
                    board.flip_square(&Square(square_index as u8));
                    file_index += 1;
                } else if let Some(digit) = char.to_digit(10) {
                    file_index += digit as usize;
                } else {
                    panic!("Invalid FEN {fen}")
                }
            }
        }

        let side = Side::try_from_str(unsafe { parts.get_unchecked(1) }).expect("Invalid FEN");
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
            pieces,
            piece_boards,
            side,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        )
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank_index in 0..8 {
            let mut empty = 0;
            for file_index in 0..8 {
                let piece = self.pieces[Square::from(7 - rank_index, file_index).0 as usize];
                if piece.is_some() {
                    if empty > 0 {
                        fen += &empty.to_string();
                        empty = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen += &empty.to_string();
            }
            if rank_index != 7 {
                fen.push('/')
            }
        }

        fen.push(' ');
        fen.push_str(&format!("{}", self.side_to_move));
        fen.push(' ');
        fen.push_str(&self.state.castling_rights.to_string());
        fen.push(' ');
        fen.push_str(
            &(if let Some(sq) = self.state.en_passant_target {
                sq.to_string()
            } else {
                "-".to_string()
            }),
        );
        fen.push(' ');
        fen.push_str(&self.state.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.state.fullmove_number.to_string());

        fen
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        castle::CastlingRights,
        piece::Piece,
        position::{Position, State},
        side::WHITE,
    };

    #[test]
    fn parses_fen_starting_position() {
        let p = Position::from_fen(Position::STARTING);

        assert_eq!(
            p,
            Position {
                pieces: [
                    Piece::WHITE_ROOK,
                    Piece::WHITE_KNIGHT,
                    Piece::WHITE_BISHOP,
                    Piece::WHITE_QUEEN,
                    Piece::WHITE_KING,
                    Piece::WHITE_BISHOP,
                    Piece::WHITE_KNIGHT,
                    Piece::WHITE_ROOK,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::WHITE_PAWN,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::NONE,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_PAWN,
                    Piece::BLACK_ROOK,
                    Piece::BLACK_KNIGHT,
                    Piece::BLACK_BISHOP,
                    Piece::BLACK_QUEEN,
                    Piece::BLACK_KING,
                    Piece::BLACK_BISHOP,
                    Piece::BLACK_KNIGHT,
                    Piece::BLACK_ROOK,
                ],
                piece_boards: [
                    Board(0x0000_0000_0000_0010),
                    Board(0x1000_0000_0000_0000),
                    Board(0x0000_0000_0000_0008),
                    Board(0x0800_0000_0000_0000),
                    Board(0x0000_0000_0000_0081),
                    Board(0x8100_0000_0000_0000),
                    Board(0x0000_0000_0000_0024),
                    Board(0x2400_0000_0000_0000),
                    Board(0x0000_0000_0000_0042),
                    Board(0x4200_0000_0000_0000),
                    Board(0x0000_0000_0000_FF00),
                    Board(0x00FF_0000_0000_0000),
                ],
                side_boards: [Board(0x0000_0000_0000_FFFF), Board(0xFFFF_0000_0000_0000)],
                side_to_move: WHITE,
                state: State {
                    castling_rights: CastlingRights::ALL_RIGHTS,
                    en_passant_target: None,
                    halfmove_clock: 0,
                    fullmove_number: 1,
                },
                hash: 15169217504194791061
            }
        );
    }
}
