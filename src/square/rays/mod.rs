mod consts;

use self::consts::{DIAGONAL_MAGICS, SHARED_ATTACKS, STRAIGHT_MAGICS};
use super::Square;
use crate::board::Board;

impl Square {
    pub fn diagonal_attacks(self, occupied: Board) -> Board {
        let magic = unsafe { *DIAGONAL_MAGICS.get_unchecked(self.0 as usize) };
        let mult = (occupied & magic.mask).0.wrapping_mul(magic.magic_number);
        let index = (mult >> 55) as usize;
        let offset = index + (magic.offset as usize);

        unsafe { *SHARED_ATTACKS.get_unchecked(offset) }
    }

    pub fn straight_attacks(self, occupied: Board) -> Board {
        let magic = unsafe { *STRAIGHT_MAGICS.get_unchecked(self.0 as usize) };
        let mult = (occupied & magic.mask).0.wrapping_mul(magic.magic_number);
        let index = (mult >> 52) as usize;
        let offset = index + (magic.offset as usize);

        unsafe { *SHARED_ATTACKS.get_unchecked(offset) }
    }
}
