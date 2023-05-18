use std::{
    error::Error,
    io::stdin,
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

use crate::{r#move::Move, Position, STARTING_POSITION_FEN};

pub fn engine_loop() -> Result<(), Box<dyn Error>> {
    let mut position = Position::from_fen(STARTING_POSITION_FEN);
    let mut stop: Option<Sender<()>> = None;

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        let mut command_iter = buffer.trim().split_whitespace();
        let command = command_iter.next();
        match command {
            // Commands mentioned in the UCI spec
            Some("uci") => {
                println!("id name mick 0.1");
                println!("id author Thomas Heyenbrock");
                println!("uciok");
            }
            Some("isready") => println!("readyok"),
            Some("ucinewgame") => position = Position::from_fen(STARTING_POSITION_FEN),
            Some("position") => {
                let mut fen = String::from(command_iter.next().unwrap_or_default());
                let mut new_position = if fen == "startpos" {
                    Position::from_fen(STARTING_POSITION_FEN)
                } else {
                    let mut next = command_iter.next();
                    while let Some(value) = next {
                        if value == "moves" {
                            next = None;
                        } else {
                            fen.push_str(value);
                            next = command_iter.next();
                        }
                    }
                    Position::try_from_fen(&fen)
                        .unwrap_or(Position::from_fen(STARTING_POSITION_FEN))
                };
                new_position.state_mut().track_hashes();

                for value in command_iter {
                    if let Ok(m) = Move::try_from_str(value, &new_position) {
                        new_position.make(m);
                    }
                }

                position = new_position;
            }
            Some("go") => {
                let mut depth = 0;

                loop {
                    let next = command_iter.next();
                    match next {
                        Some("depth") => {
                            depth = command_iter
                                .next()
                                .and_then(|d| d.parse().ok())
                                .unwrap_or(0)
                        }
                        _ => break,
                    }
                }

                if depth > 0 {
                    let best_move = position.alphabeta(depth, i32::MIN, i32::MAX);
                    println!("bestmove {best_move}");
                }

                // let (tx, rx) = channel();
                // stop = Some(tx);
                // let mut position = position.clone();
                // spawn(move || {
                //     for m in line {
                //         println!("{m}");
                //     }
                //     // TODO: actually do a search here
                //     loop {
                //         match rx.try_recv() {
                //             Ok(_) => {
                //                 println!("bestmove e2e4");
                //                 break;
                //             }
                //             Err(_) => {}
                //         }
                //     }
                // });
            }
            Some("stop") => {
                stop.clone().map(|tx| tx.send(()));
            }
            Some("quit") => break,

            // Custom commands
            Some("d") => {
                println!("{}", position);
            }

            // Ignore everything else
            _ => {}
        }
    }

    Ok(())
}
