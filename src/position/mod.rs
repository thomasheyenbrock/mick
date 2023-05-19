mod evaluate;
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

pub const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    /// In the order of moves played, meaning the last number in the list is the hash of the position one move prior
    pub prev_hashes: Option<Vec<u64>>,
}

impl State {
    pub fn track_hashes(&mut self) {
        if self.prev_hashes.is_none() {
            // TODO: benchmark the best initial capacity
            self.prev_hashes = Some(Vec::with_capacity(50));
        }
    }
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

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn new(pieces: [Piece; 64], state: State) -> Position {
        let mut piece_boards = [EMPTY; 12];
        let mut side_boards = [EMPTY; 2];

        for (idx, pc) in pieces.iter().enumerate().filter(|&(_, &pc)| pc.is_some()) {
            let bb_mask = Board::new(Square(idx as u8));
            side_boards[pc.side().0 as usize] |= bb_mask;
            piece_boards[pc.0 as usize] |= bb_mask;
        }

        let hash = DEFAULT_ZOBRISH_HASH.position(&pieces, &state);

        Position {
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

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
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
            |s| {
                let piece = self.at(s);
                if piece.is_some() {
                    Some(piece.to_char())
                } else {
                    None
                }
            },
            None,
            None,
            None,
            props.as_slice(),
        );

        write!(f, "{}", &s)
    }
}
