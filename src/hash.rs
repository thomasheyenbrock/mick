use crate::{castle::Castle, piece::Piece, position::State, side::Side, square::Square};

#[derive(Debug, PartialEq)]
pub struct Zobrist {
    pieces: [u64; 12],
    castling_rights: [u64; 16],
    en_passant_file: [u64; 8],
    castles: [[u64; 2]; 2],
    side_to_move: u64,
}

/// Default zobrist_keys generated with seed = 1
pub static DEFAULT_ZOBRISH_HASH: Zobrist = Zobrist {
    pieces: [
        16257666806172921645,
        12079090740189436754,
        11577349684372483860,
        8265070477432972399,
        17204147743807346879,
        10840387247671765879,
        11023604230088064055,
        15372782004648025408,
        17607845492419163657,
        4820222721347483354,
        9222096121752829227,
        10997107696558716930,
    ],

    castling_rights: [
        5901611952838449075,
        16928860654864062033,
        3006969943347880664,
        16761043879460025667,
        15332107909825879061,
        1522114280938701486,
        1327047711097840467,
        7301561155728042398,
        4479697827097181280,
        2468172810615015476,
        11492078287679521532,
        11685917599786030246,
        10403772991926020454,
        17478376828681188621,
        15580394547059712216,
        4575347809368850956,
    ],

    en_passant_file: [
        3348961424409688254,
        2427135436213657123,
        2898060206792371384,
        14683683615540271254,
        952321900370658987,
        5796203641919266782,
        9554333785051357809,
        12082317543310182802,
    ],

    castles: [
        [10993434298710260570, 16775444492128222288],
        [13399088802984349794, 13139464958289927509],
    ],

    side_to_move: 5703255076737973876,
};

impl Zobrist {
    /// Generates the hash of the entire position
    pub fn position(&self, grid: &[Piece; 64], state: &State) -> u64 {
        let mut hash = 0u64;

        for (idx, &pc) in grid.iter().enumerate().filter(|&(_, &pc)| pc.is_some()) {
            hash ^= self.piece_square(pc, Square(idx as u8));
        }

        hash ^= self.castling_rights[state.castling_rights.0 as usize];

        hash ^= self.ep_hash(state.en_passant_target);

        hash
    }

    /// Generates the difference between two states
    pub fn state(&self, before: &State, after: &State) -> u64 {
        self.castling_rights[before.castling_rights.0 as usize]
            ^ self.castling_rights[after.castling_rights.0 as usize]
            ^ self.ep_hash(before.en_passant_target)
            ^ self.ep_hash(after.en_passant_target)
            ^ self.side_to_move
    }

    pub fn castle(&self, castle: Castle, side_to_move: Side) -> u64 {
        return unsafe {
            *self
                .castles
                .get_unchecked(castle.0 as usize)
                .get_unchecked(side_to_move.0 as usize)
        };
    }

    pub fn capture(&self, captured: Piece, capture_square: Square) -> u64 {
        self.piece_square(captured, capture_square)
    }

    pub fn push(&self, pc_from: Piece, from: Square, pc_to: Piece, to: Square) -> u64 {
        self.piece_square(pc_from, from) ^ self.piece_square(pc_to, to)
    }

    fn piece_square(&self, piece: Piece, square: Square) -> u64 {
        return unsafe {
            (*self.pieces.get_unchecked(piece.0 as usize)).rotate_left(square.0 as u32)
        };
    }

    fn ep_hash(&self, ep_square: Option<Square>) -> u64 {
        if let Some(square) = ep_square {
            unsafe {
                *self
                    .en_passant_file
                    .get_unchecked(square.file_index() as usize)
            }
        } else {
            0
        }
    }
}
