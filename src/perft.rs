use crate::{
    move_list::{move_counter::MoveCounter, move_vec::MoveVec},
    position::Position,
};

const DEBUG_DEPTH: u8 = 0;

pub fn perft(position: &mut Position, depth: u8) -> u64 {
    if depth == 1 {
        let mut list = MoveCounter::new();
        position.legal_moves(&mut list);
        list.moves
    } else {
        let mut list = MoveVec::new();
        position.legal_moves(&mut list);
        list.iter()
            .map(|m| {
                let mut position = position.clone();
                position.make(*m);
                let c = perft(&mut position, depth - 1);
                if DEBUG_DEPTH == depth {
                    println!("{m}: {c}")
                }
                c
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{perft, position::Position};

    #[test]
    fn starting_position() {
        let mut position = Position::from_fen(Position::STARTING);
        assert_eq!(perft(&mut position, 1), 20);
        assert_eq!(perft(&mut position, 2), 400);
        assert_eq!(perft(&mut position, 3), 8902);
        assert_eq!(perft(&mut position, 4), 197281);
        assert_eq!(perft(&mut position, 5), 4865609);
    }
}
