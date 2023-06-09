use crate::{
    move_list::move_vec::{FindMoveResult, MoveVec},
    piece::NULL_PIECE,
    position::Evaluation,
    r#move::Move,
    side::Side,
    square::Square,
    utils::grid_to_string_with_props,
    Position, STARTING_POSITION_FEN,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style, terminal, ExecutableCommand, QueueableCommand, Result,
};
use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Stdout, Write},
    sync::mpsc::{channel, Receiver, Sender},
    thread::{sleep, spawn},
    time::Duration,
};

#[derive(Debug, PartialEq)]
enum State {
    Waiting { from: Option<Square> },
    Thinking,
    Terminal,
}

pub struct Game {
    stdout: Stdout,
    side: Side,
    position: Position,
    legal_moves: MoveVec,
    square: Option<Square>,
    state: State,
    channel: (Sender<Move>, Receiver<Move>),
}

impl Game {
    pub fn new(side: Side) -> Self {
        let position = Position::from_fen(STARTING_POSITION_FEN);
        let legal_moves = position.legal_moves_vec().0;

        Self {
            stdout: stdout(),
            side,
            position,
            legal_moves,
            square: Some(Square(0)),
            state: State::Waiting { from: None },
            channel: channel(),
        }
    }

    pub fn play(&mut self) -> Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        self.print_board()?;

        loop {
            if event::poll(Duration::from_millis(40))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Up if self.state != State::Terminal => {
                            if let Some(s) = self.square {
                                if s.rank_index() < 7 {
                                    self.square = Some(Square(s.0 + 8));
                                    self.print_board()?;
                                }
                            }
                        }
                        KeyCode::Down if self.state != State::Terminal => {
                            if let Some(s) = self.square {
                                if s.rank_index() > 0 {
                                    self.square = Some(Square(s.0 - 8));
                                    self.print_board()?;
                                }
                            }
                        }
                        KeyCode::Left if self.state != State::Terminal => {
                            if let Some(s) = self.square {
                                if s.file_index() > 0 {
                                    self.square = Some(Square(s.0 - 1));
                                    self.print_board()?;
                                }
                            }
                        }
                        KeyCode::Right if self.state != State::Terminal => {
                            if let Some(s) = self.square {
                                if s.file_index() < 7 {
                                    self.square = Some(Square(s.0 + 1));
                                    self.print_board()?;
                                }
                            }
                        }
                        KeyCode::Char(' ') if self.state != State::Terminal => {
                            self.select_square()?
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                }
            } else {
                if let Ok(m) = self.channel.1.try_recv() {
                    self.position.make(m);

                    let (legal_moves, is_in_check) = self.position.legal_moves_vec();
                    match self.position.evaluate(legal_moves.len(), is_in_check) {
                        Evaluation::Win(_) => {
                            self.state = State::Terminal;
                            self.print_board_with_props(vec![
                                String::from("   Checkmate"),
                                String::from("   Nice try, but Mick came out victorious!"),
                                String::from("   Press (q) to quit"),
                            ])?;
                        }
                        Evaluation::Draw(reason) => {
                            self.state = State::Terminal;
                            self.print_board_with_props(vec![
                                String::from("   Draw"),
                                format!("   {}", reason),
                                String::from("   Press (q) to quit"),
                            ])?;
                        }
                        _ => {
                            self.state = State::Waiting { from: None };
                            self.legal_moves = legal_moves;
                            self.print_board()?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn make(&mut self, m: Move) -> Result<bool> {
        self.position.make(m);

        let (legal_moves, is_in_check) = self.position.legal_moves_vec();

        match self.position.evaluate(legal_moves.len(), is_in_check) {
            Evaluation::Win(_) => {
                self.state = State::Terminal;
                self.print_board_with_props(vec![
                    String::from("   Checkmate"),
                    String::from("   Congrats, you beat Mick!"),
                    String::from("   Press (q) to quit"),
                ])?;
                return Ok(true);
            }
            Evaluation::Draw(reason) => {
                self.state = State::Terminal;
                self.print_board_with_props(vec![
                    String::from("   Draw"),
                    format!("   {}", reason),
                    String::from("   Press (q) to quit"),
                ])?;
                return Ok(true);
            }
            _ => {}
        }

        self.state = State::Thinking;
        self.print_board()?;

        let sender = self.channel.0.clone();
        spawn(move || {
            sleep(Duration::from_secs(1));

            let mut rng = thread_rng();
            let m = legal_moves
                .iter()
                .nth(rng.gen_range(0..legal_moves.len()))
                .unwrap();
            sender.send(*m)
        });

        Ok(false)
    }

    fn print_board(&mut self) -> Result<()> {
        self.print_board_with_props(if self.state == State::Thinking {
            vec![String::from("   Mick is thinking...")]
        } else {
            vec![]
        })
    }

    fn print_board_with_props(&mut self, props: Vec<String>) -> Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;

        let from = match self.state {
            State::Waiting { from } => from,
            _ => None,
        };
        let grid = grid_to_string_with_props(
            |s| self.position.at(s).to_symbol(),
            self.square,
            from,
            from.map(|s| self.legal_moves.from(s)),
            props.as_slice(),
        );
        for (line, row) in grid.split("\n").enumerate() {
            self.stdout.queue(cursor::MoveTo(0, line as u16))?;
            self.stdout.queue(style::Print(row))?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    fn select_square(&mut self) -> Result<()> {
        if self.position.state().side_to_move != self.side {
            return Ok(());
        }

        if let Some(s) = self.square {
            let mut is_finished = false;
            let piece = self.position.at(s);

            if let State::Waiting { from: Some(from) } = self.state {
                if from == s {
                    self.state = State::Waiting { from: None };
                } else if piece == NULL_PIECE || piece.side() == !self.side {
                    match self.legal_moves.find(from, s) {
                        FindMoveResult::None => {}
                        FindMoveResult::Move(m) => {
                            is_finished = self.make(m)?;
                        }
                        FindMoveResult::Promotions([m1, m2, m3, m4]) => {
                            self.print_board_with_props(vec![
                                String::from("To which piece kind do you want to promote to?"),
                                String::from("(1) Queen"),
                                String::from("(2) Rook"),
                                String::from("(3) Bishop"),
                                String::from("(4) Knight"),
                            ])?;
                            loop {
                                match event::read()? {
                                    Event::Key(key) => match key.code {
                                        KeyCode::Char('1') => {
                                            is_finished = self.make(m1)?;
                                            break;
                                        }
                                        KeyCode::Char('2') => {
                                            is_finished = self.make(m2)?;
                                            break;
                                        }
                                        KeyCode::Char('3') => {
                                            is_finished = self.make(m3)?;
                                            break;
                                        }
                                        KeyCode::Char('4') => {
                                            is_finished = self.make(m4)?;
                                            break;
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                } else if piece.side() == self.side {
                    self.state = State::Waiting { from: Some(s) };
                } else {
                    unreachable!("Square contains unknown piece {piece}")
                }
            } else if piece.side() == self.side {
                self.state = State::Waiting { from: Some(s) };
            }

            if !is_finished {
                self.print_board()?;
            }
        }

        Ok(())
    }
}
