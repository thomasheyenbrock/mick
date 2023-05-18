use crate::{
    piece::NULL_PIECE,
    side::{Side, WHITE},
    square::Square,
    utils::grid_to_string,
    Position, STARTING_POSITION_FEN,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style, terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{
    error::Error,
    io::{self, stdout, Stdout, Write},
};

pub struct Game {
    stdout: Stdout,
    side: Side,
    position: Position,
    square: Option<Square>,
    from: Option<Square>,
}

impl Game {
    pub fn new(side: Side) -> Self {
        Self {
            stdout: stdout(),
            side,
            position: Position::from_fen(STARTING_POSITION_FEN),
            square: Some(Square(0)),
            from: None,
        }
    }

    pub fn play(&mut self) -> Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        self.print_board()?;

        loop {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Up => {
                        if let Some(s) = self.square {
                            if s.rank_index() < 7 {
                                self.square = Some(Square(s.0 + 8));
                                self.print_board()?;
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(s) = self.square {
                            if s.rank_index() > 0 {
                                self.square = Some(Square(s.0 - 8));
                                self.print_board()?;
                            }
                        }
                    }
                    KeyCode::Left => {
                        if let Some(s) = self.square {
                            if s.file_index() > 0 {
                                self.square = Some(Square(s.0 - 1));
                                self.print_board()?;
                            }
                        }
                    }
                    KeyCode::Right => {
                        if let Some(s) = self.square {
                            if s.file_index() < 7 {
                                self.square = Some(Square(s.0 + 1));
                                self.print_board()?;
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(s) = self.square {
                            let piece = self.position.at(s);

                            if let Some(from) = self.from {
                                if from == s {
                                    self.from = None;
                                } else if piece == NULL_PIECE {
                                    self.stdout.execute(cursor::MoveTo(0, 20))?;
                                    self.stdout.execute(style::Print("TODO: Check if you can move the piece to this empty square"))?;
                                    return Ok(());
                                } else if piece.side() == self.side {
                                    self.from = Some(s);
                                } else if piece.side() == !self.side {
                                    self.stdout.execute(cursor::MoveTo(0, 20))?;
                                    self.stdout.execute(style::Print(
                                        "TODO: Check if you can capture this enemy piece",
                                    ))?;
                                    return Ok(());
                                } else {
                                    unreachable!("Square contains unknown piece {piece}")
                                }
                            } else if piece.side() == self.side {
                                self.from = Some(s);
                            }

                            self.print_board()?;
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn print_board(&mut self) -> Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;

        let grid = grid_to_string(|s| self.position.at(s).to_symbol(), self.square, self.from);
        for (line, row) in grid.split("\n").enumerate() {
            self.stdout.queue(cursor::MoveTo(0, line as u16))?;
            self.stdout.queue(style::Print(row))?;
        }

        self.stdout.flush()?;
        Ok(())
    }
}
