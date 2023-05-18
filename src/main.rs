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
mod play;
mod position;
mod side;
mod square;
mod utils;

extern crate clap;
extern crate num_cpus;
#[cfg(test)]
extern crate test;
extern crate threadpool;

use clap::{Parser, Subcommand, ValueEnum};
use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use engine::engine_loop;
pub use perft::perft;
use play::Game;
pub use position::{Position, STARTING_POSITION_FEN};
use side::{Side, BLACK, WHITE};
use std::{error::Error, fmt::Display, io::stdout, time::Instant};

#[derive(Subcommand)]
enum Commands {
    /// Run perft on the starting board position
    Perft(PerftArgs),
    /// Start the engine
    Start,
    /// Play a game against the engine
    Play {
        #[arg(long, default_value_t = SideEnum::White)]
        side: SideEnum,
    },
}

#[derive(clap::Args)]
struct PerftArgs {
    #[arg(long)]
    depth: usize,

    #[arg(long)]
    fen: Option<String>,
}

#[derive(Clone, ValueEnum)]
enum SideEnum {
    White,
    Black,
}

impl Display for SideEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "white"),
            Self::Black => write!(f, "black"),
        }
    }
}

impl SideEnum {
    fn to_side(self) -> Side {
        match self {
            Self::White => WHITE,
            Self::Black => BLACK,
        }
    }
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
        Some(Commands::Play { side }) => {
            let mut stdout = stdout();

            enable_raw_mode()?;
            stdout.execute(cursor::Hide)?;

            let _ = Game::new(side.to_side()).play();

            disable_raw_mode()?;
            stdout.execute(cursor::Show)?;
        }
        _ => todo!("not implemented"),
    }

    Ok(())
}
