use std::{
    error::Error,
    io::stdin,
    sync::mpsc::{channel, Sender, TryRecvError},
    thread::spawn,
};

use crate::{r#move::Move, Position, STARTING_POSITION_FEN};

pub fn event_loop() -> Result<(), Box<dyn Error>> {
    let mut position = None;
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
            Some("ucinewgame") => position = None,
            Some("position") => {
                let mut fen = String::from(command_iter.next().unwrap_or_default());
                if fen == "startpos" {
                    position = Some(Position::from_fen(STARTING_POSITION_FEN));
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
                    position = Position::try_from_fen(&fen).ok();
                }

                if let Some(ref mut position) = position {
                    for value in command_iter {
                        if let Ok(m) = Move::try_from_str(value, &position) {
                            position.make(m);
                        }
                    }
                }
            }
            Some("go") => {
                if let Some(ref position) = position {
                    let (tx, rx) = channel();
                    stop = Some(tx);
                    spawn(move || {
                        // TODO: actually do a search here
                        loop {
                            match rx.try_recv() {
                                Ok(_) | Err(TryRecvError::Disconnected) => {
                                    println!("bestmove e2e4");
                                    break;
                                }
                                Err(TryRecvError::Empty) => {}
                            }
                        }
                    });
                }
            }
            Some("stop") => {
                stop.clone().map(|tx| tx.send(()));
            }
            Some("quit") => break,

            // Custom commands
            Some("d") => {
                if let Some(ref position) = position {
                    println!("{}", position);
                } else {
                    println!("No position");
                }
            }

            // Ignore everything else
            _ => {}
        }
    }

    Ok(())
}
