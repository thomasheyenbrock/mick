mod fen;
mod legal_moves;
mod r#move;

use crate::{
    board::{Board, EMPTY},
    castle::CastlingRights,
    hash::DEFAULT_ZOBRISH_HASH,
    piece::Piece,
    side::{Side, BLACK, WHITE},
    square::Square,
    utils::grid_to_string_with_props,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pieces: [Piece; 64],
    piece_boards: [Board; 12],
    side_boards: [Board; 2],
    state: State,
    hash: u64,
}

impl Position {
    pub fn at(&self, sq: Square) -> Piece {
        unsafe { return *self.pieces.get_unchecked(sq.0 as usize) }
    }

    pub fn empty(&self) -> Board {
        !self.occupied()
    }

    pub fn new(
        pieces: [Piece; 64],
        piece_boards: [Board; 12],
        side_to_move: Side,
        castling_rights: CastlingRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        fullmove_number: u32,
    ) -> Self {
        let mut side_boards = [EMPTY; 2];
        for (i, board) in piece_boards.iter().enumerate() {
            side_boards[i % 2].flip_board(board);
        }

        let state = State {
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        };

        let hash = DEFAULT_ZOBRISH_HASH.position(&pieces, &state);

        Self {
            pieces,
            piece_boards,
            side_boards,
            state,
            hash,
        }
    }

    pub fn occupied(&self) -> Board {
        self.side(WHITE) | self.side(BLACK)
    }

    pub fn piece(&self, pc: Piece) -> Board {
        unsafe { return *self.piece_boards.get_unchecked(pc.0 as usize) }
    }

    pub fn side(&self, side: Side) -> Board {
        unsafe { return *self.side_boards.get_unchecked(side.0 as usize & 1) }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let props = vec![
            ("    side to move", format!("{}", self.state.side_to_move)),
            (
                " castling rights",
                format!("{}", self.state.castling_rights),
            ),
            (
                "      en-passant",
                self.state
                    .en_passant_target
                    .map_or("-".to_string(), |s| s.to_string()),
            ),
            (" half-move clock", self.state.halfmove_clock.to_string()),
            ("full-move number", self.state.fullmove_number.to_string()),
            ("             FEN", self.to_fen()),
            ("            hash", format!("{:016X}", self.hash)),
        ];
        let s = grid_to_string_with_props(
            |s: Square| -> char {
                let piece = self.pieces[s.0 as usize];
                if piece.is_some() {
                    piece.to_char()
                } else {
                    ' '
                }
            },
            props.as_slice(),
        );

        write!(f, "{}", &s)
    }
}
