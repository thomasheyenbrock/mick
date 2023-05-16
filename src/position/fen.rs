use super::State;
use crate::{
    castle::{CastlingRights, ALL_RIGHTS},
    piece::{Piece, NULL_PIECE},
    side::{Side, WHITE},
    square::Square,
    Position,
};

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        Position::try_from_fen(fen).expect("Invalid fen")
    }

    pub fn try_from_fen(fen: &str) -> Result<Position, String> {
        let mut state = State {
            castling_rights: ALL_RIGHTS,
            en_passant_target: None,
            side_to_move: WHITE,
            halfmove_clock: 0,
            fullmove_number: 1,
            prev_hashes: None,
        };

        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.is_empty() {
            return Err(format!("Not enough fields for FEN: {}", fen));
        }

        let mut grid = [NULL_PIECE; 64];

        for (i, row_str) in parts[0].split('/').enumerate() {
            if i >= 8 {
                break;
            }

            let row = 7 - i;
            let mut col = 0;
            for c in row_str.chars() {
                if ('1'..='8').contains(&c) {
                    col += c as usize - '1' as usize;
                } else {
                    if col >= 8 {
                        return Err(format!("Too many pieces on row {}", row + 1));
                    }
                    grid[Square::from(row as u8, col as u8).0 as usize] = Piece::try_from_char(c)?;
                }
                col += 1;
            }
        }

        if parts.len() > 1 {
            state.side_to_move = Side::try_from_str(parts[1])?;
        }

        if parts.len() > 2 {
            state.castling_rights = CastlingRights::try_from_str(parts[2])?;
        }

        if parts.len() > 3 {
            state.en_passant_target = Square::try_from_str(parts[3])?;
        }

        if parts.len() > 4 && parts[4] != "-" {
            match parts[4].parse::<u32>() {
                Ok(hmc) => state.halfmove_clock = hmc,
                Err(err) => return Err(err.to_string()),
            }
        }

        if parts.len() > 5 && parts[5] != "-" {
            match parts[5].parse::<u32>() {
                Ok(fmn) => state.fullmove_number = fmn,
                Err(err) => return Err(err.to_string()),
            }
        }

        Ok(Position::new(grid, state))
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
        fen.push_str(&format!("{}", self.state.side_to_move));
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
        castle::ALL_RIGHTS,
        piece::{
            BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
            NULL_PIECE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN,
            WHITE_ROOK,
        },
        position::{Position, State},
        side::WHITE,
        STARTING_POSITION_FEN,
    };

    #[test]
    fn parses_fen_starting_position() {
        let p = Position::from_fen(STARTING_POSITION_FEN);

        assert_eq!(
            p,
            Position {
                pieces: [
                    WHITE_ROOK,
                    WHITE_KNIGHT,
                    WHITE_BISHOP,
                    WHITE_QUEEN,
                    WHITE_KING,
                    WHITE_BISHOP,
                    WHITE_KNIGHT,
                    WHITE_ROOK,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    WHITE_PAWN,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    NULL_PIECE,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_PAWN,
                    BLACK_ROOK,
                    BLACK_KNIGHT,
                    BLACK_BISHOP,
                    BLACK_QUEEN,
                    BLACK_KING,
                    BLACK_BISHOP,
                    BLACK_KNIGHT,
                    BLACK_ROOK,
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
                state: State {
                    side_to_move: WHITE,
                    castling_rights: ALL_RIGHTS,
                    en_passant_target: None,
                    halfmove_clock: 0,
                    fullmove_number: 1,
                    prev_hashes: None
                },
                hash: 1307476362392126559
            }
        );
    }
}
