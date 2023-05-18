use crate::{
    cache::Cache,
    move_list::{move_counter::MoveCounter, move_vec::MoveVec},
    Position,
};
use num_cpus;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

/// Returns the number of nodes at the provided depth
/// cache_bytes_per_thread must be of form 2^N bytes
/// if multi_threading_enabled is set to true search will
/// run concurrently accross threads equal to your CPU count
pub fn perft(
    position: &mut Position,
    depth: usize,
    multi_threading_enabled: bool,
    cache_bytes_per_thread: usize,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth <= 3 {
        return perft_inner(position, depth, true);
    }

    if !multi_threading_enabled {
        if cache_bytes_per_thread > 0 {
            let mut cache = Cache::new(cache_bytes_per_thread).unwrap();
            return perft_with_cache_inner(position, depth, &mut cache, true);
        } else {
            return perft_inner(position, depth, true);
        }
    }

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut moves = MoveVec::new();
    position.legal_moves(&mut moves);
    let moves_len = moves.len();

    for &m in moves.iter() {
        let tx = tx.clone();
        let mut position_local = position.clone();

        pool.execute(move || {
            position_local.make(m);

            let count: u64;
            if cache_bytes_per_thread > 0 {
                let mut cache = Cache::new(cache_bytes_per_thread).unwrap();
                count = perft_with_cache_inner(&mut position_local, depth - 1, &mut cache, false);
            } else {
                count = perft_inner(&mut position_local, depth - 1, false);
            }

            println!("{m}: {count}");

            tx.send(count).unwrap();
        });
    }

    return rx.iter().take(moves_len).sum();
}

fn perft_inner(position: &mut Position, depth: usize, should_print: bool) -> u64 {
    if depth == 1 && !should_print {
        let mut counter = MoveCounter::new();
        position.legal_moves(&mut counter);
        return counter.moves;
    }

    let mut moves = MoveVec::new();
    position.legal_moves(&mut moves);

    let state = position.state().clone();
    let hash = position.hash();
    let mut total = 0;
    for &m in moves.iter() {
        let capture = position.make(m);

        let count = perft_inner(position, depth - 1, false);
        if should_print {
            println!("{m}: {count}");
        }
        total += count;

        position.unmake(m, capture, &state, hash);
    }

    total
}

fn perft_with_cache_inner(
    position: &mut Position,
    depth: usize,
    cache: &mut Cache,
    should_print: bool,
) -> u64 {
    let hash = position.hash();

    let result = cache.probe(hash, depth);
    if let Some(value) = result {
        return value;
    }

    let mut total = 0;
    if depth == 1 && !should_print {
        let mut counter = MoveCounter::new();
        position.legal_moves(&mut counter);
        total = counter.moves as u64;
    } else {
        let mut moves = MoveVec::new();
        position.legal_moves(&mut moves);

        let state = position.state().clone();
        let hash = position.hash();
        for &m in moves.iter() {
            let capture = position.make(m);

            let count = perft_with_cache_inner(position, depth - 1, cache, false);
            if should_print {
                println!("{m}: {count}");
            }
            total += count;

            position.unmake(m, capture, &state, hash);
        }
    }

    cache.save(hash, total, depth as i16);

    total
}

#[cfg(test)]
mod test {
    use crate::{move_list::move_vec::MoveVec, perft, Position, STARTING_POSITION_FEN};

    #[test]
    fn p() {
        let mut position = Position::from_fen(STARTING_POSITION_FEN);
        assert_eq!(perft(&mut position, 1, true, 1024 * 1024 * 4), 20);

        let mut position = Position::from_fen(STARTING_POSITION_FEN);
        assert_eq!(perft(&mut position, 2, true, 1024 * 1024 * 4), 400);

        let mut position = Position::from_fen(STARTING_POSITION_FEN);
        assert_eq!(perft(&mut position, 3, true, 1024 * 1024 * 4), 8902);

        let mut position = Position::from_fen(STARTING_POSITION_FEN);
        assert_eq!(perft(&mut position, 4, true, 1024 * 1024 * 4), 197281);

        let mut position = Position::from_fen(STARTING_POSITION_FEN);
        assert_eq!(perft(&mut position, 5, true, 1024 * 1024 * 4), 4865609);
    }

    #[bench]
    fn l(b: &mut test::Bencher) {
        let position = Position::from_fen(STARTING_POSITION_FEN);
        b.iter(|| {
            for _ in 0..1000 {
                position.legal_moves(&mut MoveVec::new());
            }
        })
    }
}
