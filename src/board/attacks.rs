use super::{Board, FILE_A, FILE_B, FILE_G, FILE_H, NOT_FILE_A, NOT_FILE_H};

impl Board {
    pub fn diagonal_attacks(self, occupied: Board) -> Board {
        let empty = !occupied;
        self.north_east_attacks(empty)
            | self.north_west_attacks(empty)
            | self.south_east_attacks(empty)
            | self.south_west_attacks(empty)
    }

    pub fn east_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 1);
        prop &= prop << 1;
        gen |= prop & (gen << 2);
        prop &= prop << 2;
        gen |= prop & (gen << 4);

        Board((gen << 1) & NOT_FILE_A.0)
    }

    pub fn knight_attacks(self) -> Board {
        let attacks_right_one = (self << 1) & !FILE_A;
        let attacks_right_two = (self << 2) & !(FILE_A | FILE_B);
        let attacks_left_one = (self >> 1) & !FILE_H;
        let attacks_left_two = (self >> 2) & !(FILE_H | FILE_G);

        let attacks_one = attacks_right_one | attacks_left_one;
        let attacks_two = attacks_right_two | attacks_left_two;

        (attacks_one << 16) | (attacks_one >> 16) | (attacks_two << 8) | (attacks_two >> 8)
    }

    pub fn north_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen |= prop & (gen << 8);
        prop &= prop << 8;
        gen |= prop & (gen << 16);
        prop &= prop << 16;
        gen |= prop & (gen << 32);

        Board(gen << 8)
    }

    pub fn north_east_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 9);
        prop &= prop << 9;
        gen |= prop & (gen << 18);
        prop &= prop << 18;
        gen |= prop & (gen << 36);

        Board((gen << 9) & NOT_FILE_A.0)
    }

    pub fn north_west_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen << 7);
        prop &= prop << 7;
        gen |= prop & (gen << 14);
        prop &= prop << 14;
        gen |= prop & (gen << 28);

        Board((gen << 7) & NOT_FILE_H.0)
    }

    pub fn south_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 8);
        prop &= prop >> 8;
        gen |= prop & (gen >> 16);
        prop &= prop >> 16;
        gen |= prop & (gen >> 32);

        Board(gen >> 8)
    }

    pub fn south_east_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 7);
        prop &= prop >> 7;
        gen |= prop & (gen >> 14);
        prop &= prop >> 14;
        gen |= prop & (gen >> 28);

        Board((gen >> 7) & NOT_FILE_A.0)
    }

    pub fn south_west_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 9);
        prop &= prop >> 9;
        gen |= prop & (gen >> 18);
        prop &= prop >> 18;
        gen |= prop & (gen >> 36);

        Board((gen >> 9) & NOT_FILE_H.0)
    }

    pub fn straight_attacks(self, occupied: Board) -> Board {
        let empty = !occupied;
        self.east_attacks(empty)
            | self.north_attacks(empty)
            | self.south_attacks(empty)
            | self.west_attacks(empty)
    }

    pub fn west_attacks(self, empty: Board) -> Board {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 1);
        prop &= prop >> 1;
        gen |= prop & (gen >> 2);
        prop &= prop >> 2;
        gen |= prop & (gen >> 4);

        Board((gen >> 1) & NOT_FILE_H.0)
    }
}
