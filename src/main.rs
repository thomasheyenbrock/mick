#![feature(test)]

mod board;
mod cache;
mod castle;
mod hash;
mod r#move;
mod move_list;
mod perft;
mod piece;
mod position;
mod side;
mod square;
mod utils;

extern crate clap;
extern crate num_cpus;
extern crate rand;
#[cfg(test)]
extern crate test;
extern crate threadpool;

use clap::{Parser, Subcommand};
pub use perft::perft;
pub use position::{Position, STARTING_POSITION_FEN};
use rand::RngCore;
use std::time::Instant;

#[derive(Subcommand)]
enum Commands {
    /// Run perft on the starting board position
    Perft,
    /// Create and print a set of numbers for Zorbist hashing
    Zorbist {
        #[arg(default_value_t = 0)]
        seed: u64,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Perft) => {
            let fen = STARTING_POSITION_FEN;
            let mut position = Position::from_fen(fen);

            let depth: usize = 7;
            println!(
                "Running performance test on starting position, depth {}",
                depth
            );
            let now = Instant::now();
            let move_count = perft(&mut position, depth, true, 1024 * 1024 * 4);
            let elapsed = now.elapsed();
            let sec =
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
            let nps = move_count as f64 / sec;

            println!(
                "Done. Total moves: {} ({:5} seconds, {:0} NPS)",
                move_count, sec, nps
            );
        }
        Some(Commands::Zorbist { seed }) => {
            use rand::rngs::SmallRng;
            use rand::SeedableRng;

            let mut small_rng = SmallRng::seed_from_u64(seed);

            let pieces = 12;
            let sides = 1; // Only one used when it's blacks turn
            let castle_rights = 2u32.pow(4);
            let files = 8; // Used to indicate the en-passant target
            let n = pieces + castle_rights + files + sides;

            for _ in 0..n {
                println!("{}", small_rng.next_u64());
            }
        }
        _ => todo!("not implemented"),
    }
}
