// TODO: refactor square
// TODO: refactor castle
// TODO: refactor zorbist -> hash
// TODO: refactor board
// TODO: refactor fen
// TODO: refactor move
// TODO: add move list
#![feature(int_roundings)]

mod board;
mod castle;
mod hash;
mod r#move;
mod perft;
mod piece;
mod position;
mod side;
mod square;
mod utils;

use clap::{Parser, Subcommand};
use perft::perft;
use position::Position;
use rand::RngCore;

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

/// Simple program to greet a person
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
            let mut p = Position::from_fen(Position::STARTING);
            println!("{}", p);
            println!("{}", perft(&mut p, 1));
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
