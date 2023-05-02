use crate::position::Position;

const DEBUG_DEPTH: u8 = 3;

pub fn perft(position: &Position, depth: u8) -> usize {
    let moves = position.legal_moves();

    if depth == 1 {
        if DEBUG_DEPTH == 1 {
            for m in moves.iter() {
                println!("{m}: 1")
            }
        }
        moves.len()
    } else {
        moves
            .iter()
            .map(|m| {
                let c = perft(position.clone().make(m), depth - 1);
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
        let position = Position::from_fen(Position::STARTING);
        assert_eq!(perft(&position, 1), 20);
        assert_eq!(perft(&position, 2), 400);
        assert_eq!(perft(&position, 3), 8902);
        assert_eq!(perft(&position, 4), 197281);
        assert_eq!(perft(&position, 5), 4865609);
    }
}
