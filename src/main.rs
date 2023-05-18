#![feature(test)]

mod board;
mod cache;
mod castle;
mod engine;
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
#[cfg(test)]
extern crate test;
extern crate threadpool;

use clap::{Parser, Subcommand};
use engine::engine_loop;
pub use perft::perft;
pub use position::{Position, STARTING_POSITION_FEN};
use std::{error::Error, time::Instant};

#[derive(Subcommand)]
enum Commands {
    /// Run perft on the starting board position
    Perft(PerftArgs),
    /// Start the engine
    Start,
}

#[derive(clap::Args)]
struct PerftArgs {
    #[arg(long)]
    depth: usize,

    #[arg(long)]
    fen: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Perft(args)) => {
            let fen = args.fen.unwrap_or(String::from(STARTING_POSITION_FEN));
            let mut position = Position::from_fen(&fen);

            let now = Instant::now();
            let move_count = perft(&mut position, args.depth, true, 1024 * 1024 * 4);
            let elapsed = now.elapsed();

            let sec =
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
            let nps = move_count as f64 / sec;

            println!("\nNodes searched: {move_count}");
            println!("Time: {sec:5} sec");
            println!("NPS: {nps:0}");
        }
        Some(Commands::Start) => engine_loop()?,
        _ => todo!("not implemented"),
    }

    Ok(())
}
