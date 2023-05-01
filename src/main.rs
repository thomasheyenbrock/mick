use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// run perft on the starting board position
    Perft,
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
        Some(Commands::Perft) => println!("perft"),
        _ => todo!("not implemented"),
    }
}
