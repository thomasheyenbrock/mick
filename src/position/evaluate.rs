use crate::{
    piece::{
        BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    r#move::Move,
    side::{Side, WHITE},
    Position,
};
use std::{
    cmp::{max, min},
    fmt::Display,
    time::Instant,
};

#[derive(Debug, PartialEq)]
pub enum DrawReason {
    FiftyMoveRule,
    InsufficientMaterial,
    Stalemate,
    ThreefoldRepetition,
}

impl Display for DrawReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FiftyMoveRule => write!(f, "Fifty moves without pawn move or capture"),
            Self::InsufficientMaterial => write!(f, "Insufficient material"),
            Self::Stalemate => write!(f, "Stalemate"),
            Self::ThreefoldRepetition => write!(f, "Repetition of moves"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Evaluation {
    Win(Side),
    Draw(DrawReason),
    None(i32),
}

impl Evaluation {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::None(_) => false,
            _ => true,
        }
    }

    pub fn to_score(&self) -> i32 {
        match self {
            Self::Win(side) => (1 - 2 * side.0 as i32) * i32::MAX,
            Self::Draw(_) => 0,
            Self::None(score) => *score,
        }
    }
}

#[derive(Debug, Default)]
struct Stats {
    nodes: u64,
}

impl Position {
    pub fn alphabeta(&mut self, depth: u8, alpha: i32, beta: i32) -> Move {
        let mut stats = Stats::default();

        let start = Instant::now();
        let (score, line) = self.alphabeta_with_stats(depth, alpha, beta, &mut stats);
        let duration = start.elapsed().as_millis();

        let nodes = stats.nodes;
        let nps = (stats.nodes as f64 / (duration as f64 / 1000f64)) as u64;

        println!(
            "info depth {depth} time {duration} nodes {nodes} nps {nps} score cp {score} pv {}",
            line.iter()
                .rev()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(" ")
        );

        *line.last().unwrap()
    }

    fn alphabeta_with_stats(
        &mut self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        stats: &mut Stats,
    ) -> (i32, Vec<Move>) {
        stats.nodes += 1;

        let (legal_moves, is_in_check) = self.legal_moves_vec();

        let evaluation = self.evaluate(legal_moves.len(), is_in_check);
        if depth == 0 || evaluation.is_terminal() {
            return (evaluation.to_score(), vec![]);
        }

        if self.state.side_to_move == WHITE {
            let mut value = i32::MIN;
            let mut best_line = vec![];

            for m in legal_moves.iter() {
                let state = self.state.clone();
                let hash = self.hash;
                let capture = self.make(*m);

                let (move_value, mut line) =
                    self.alphabeta_with_stats(depth - 1, alpha, beta, stats);
                if move_value > value {
                    value = move_value;
                    best_line = {
                        line.push(*m);
                        line
                    };
                }
                value = max(value, move_value);
                alpha = max(alpha, value);

                self.unmake(*m, capture, &state, hash);

                if value >= beta {
                    break;
                }
            }

            (value, best_line)
        } else {
            let mut value = i32::MAX;
            let mut best_line = vec![];

            for m in legal_moves.iter() {
                let state = self.state.clone();
                let hash = self.hash;
                let capture = self.make(*m);

                let (move_value, mut line) =
                    self.alphabeta_with_stats(depth - 1, alpha, beta, stats);
                if move_value < value {
                    value = move_value;
                    best_line = {
                        line.push(*m);
                        line
                    };
                }
                beta = min(beta, value);

                self.unmake(*m, capture, &state, hash);

                if value <= alpha {
                    break;
                }
            }

            (value, best_line)
        }
    }

    pub fn evaluate(&self, legal_move_count: usize, is_in_check: bool) -> Evaluation {
        if legal_move_count == 0 {
            // The side to move has no legal moves left
            if is_in_check {
                // Checkmate
                return Evaluation::Win(self.state.side_to_move);
            }

            // Stalemate
            return Evaluation::Draw(DrawReason::Stalemate);
        }

        // Fifty move rule
        if self.state.halfmove_clock >= 50 {
            return Evaluation::Draw(DrawReason::FiftyMoveRule);
        }

        let white_queens = self.piece(WHITE_QUEEN).occupied();
        let white_rooks = self.piece(WHITE_ROOK).occupied();
        let white_bishop_board = self.piece(WHITE_BISHOP);
        let white_bishops = white_bishop_board.occupied();
        let white_knights = self.piece(WHITE_KNIGHT).occupied();
        let white_pawns = self.piece(WHITE_PAWN).occupied();

        let black_queens = self.piece(BLACK_QUEEN).occupied();
        let black_rooks = self.piece(BLACK_ROOK).occupied();
        let black_bishop_board = self.piece(BLACK_BISHOP);
        let black_bishops = black_bishop_board.occupied();
        let black_knights = self.piece(BLACK_KNIGHT).occupied();
        let black_pawns = self.piece(BLACK_PAWN).occupied();

        let score_white = (white_queens * 900
            + white_rooks * 500
            + white_bishops * 300
            + white_knights * 300
            + white_pawns * 100) as i32;
        let score_black: i32 = (black_queens * 900
            + black_rooks * 500
            + black_bishops * 300
            + black_knights * 300
            + black_pawns * 100) as i32;

        // Insufficient material (king vs. king)
        if score_white == 0 && score_black == 0 {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Insufficient material (king vs. king & bishop)
        let black_has_only_bishops =
            black_pawns == 0 && black_knights == 0 && black_rooks == 0 && black_queens == 0;
        if score_white == 0 && black_has_only_bishops && black_bishops == 1 {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Insufficient material (king & bishop vs. king)
        let white_has_only_bishops =
            white_pawns == 0 && white_knights == 0 && white_rooks == 0 && white_queens == 0;
        if score_black == 0 && white_has_only_bishops && white_bishops == 1 {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Insufficient material (king vs. king & knight)
        if score_white == 0
            && black_pawns == 0
            && black_knights == 1
            && black_bishops == 0
            && black_rooks == 0
            && black_queens == 0
        {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Insufficient material (king & knight vs. king)
        if score_black == 0
            && white_pawns == 0
            && white_knights == 1
            && white_bishops == 0
            && white_rooks == 0
            && white_queens == 0
        {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Insufficient material (king & bishop vs. king & bishop on same colors)
        if white_has_only_bishops
            && black_has_only_bishops
            && white_bishops == 1
            && black_bishops == 1
            && {
                let is_white_bishop_on_white_square =
                    white_bishop_board.0 & 0xAA55_AA55_AA55_AA55 == 0;
                let is_black_bishop_on_white_square =
                    black_bishop_board.0 & 0xAA55_AA55_AA55_AA55 == 0;
                is_white_bishop_on_white_square == is_black_bishop_on_white_square
            }
        {
            return Evaluation::Draw(DrawReason::InsufficientMaterial);
        }

        // Threefold repetition
        if let Some(prev_hashes) = &self.state.prev_hashes {
            let num_hashes = prev_hashes.len();
            if num_hashes >= 4 {
                let mut count = 0;
                let mut index = num_hashes - 2;
                loop {
                    if prev_hashes[index] == self.hash {
                        count += 1;
                    }
                    if count >= 2 {
                        return Evaluation::Draw(DrawReason::ThreefoldRepetition);
                    }
                    if index < 2 {
                        break;
                    }
                    index -= 2;
                }
            }
        }

        Evaluation::None(score_white - score_black)
    }
}
