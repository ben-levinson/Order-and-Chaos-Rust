use std::cmp::{max, min};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum GameStatus {
    InProgress,
    OrderWins,
    ChaosWins,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveType {
    X,
    O,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    piece_type: MoveType,
    row: usize,
    col: usize,
}

impl Move {
    pub fn new(piece_type: MoveType, row: usize, col: usize) -> Self {
        Move {
            piece_type: piece_type,
            row: row,
            col: col,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardDirection {
    Row,
    Column,
    Diagonal,
    AntiDiagonal,
}

#[derive(Clone)]
pub struct Game {
    size: usize,
    num_to_win: usize,
    board: Vec<Option<MoveType>>,
    pieces_placed: usize,
    last_move: Option<(usize, usize)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            size: 6,
            board: vec![None; 36],
            pieces_placed: 0,
            num_to_win: 5,
            last_move: None,
        }
    }
    pub fn last_move(&self) -> Option<(usize, usize)> {
        self.last_move.clone()
    }

    pub fn flat_index(&self, row: usize, col: usize) -> Option<MoveType> {
        self.board[row * self.size + col].clone()
    }

    pub fn reset(&mut self) -> Game {
        self.board = vec![None; 36];
        self.clone()
    }

    fn num_consecutive(to_search: usize, f: &Fn(usize) -> Option<MoveType>) -> usize {
        let mut max_count = 0;
        let mut count = 0;
        let mut cell_type = MoveType::X;
        for i in 0..to_search {
            count = match f(i) {
                Some(cell) => {
                    if cell == cell_type {
                        count + 1
                    } else {
                        max_count = max(count, max_count);
                        cell_type = cell;
                        1
                    }
                }
                None => {
                    max_count = max(count, max_count);
                    0
                }
            }
        }
        max(count, max_count)
    }

    pub fn count_direction(&self, direction: BoardDirection, row: usize, col: usize) -> usize {
        //There are only 6 diagonals that allow for a win condition for Order,
        //Use the coordinates of the last move to determine if the move is on
        //one of these diagonals and if so, how many cells need to be checked
        let diag_min = min(col, row);
        let diag_search = match row as i64 - col as i64 {
            -1 => 5,
            0 => 6,
            1 => 5,
            _ => 0,
        };
        let anti_diag_min = min(col, self.size - row - 1);
        let anti_diag_search = match row + col {
            4 => 5,
            5 => 6,
            6 => 5,
            _ => 0,
        };
        match direction {
            BoardDirection::Row => Self::num_consecutive(self.size, &|i| self.flat_index(row, i)),
            BoardDirection::Column => {
                Self::num_consecutive(self.size, &|i| self.flat_index(i, col))
            }
            BoardDirection::Diagonal => Self::num_consecutive(diag_search, &|i| {
                self.flat_index(row + i - diag_min, col + i - diag_min)
            }),
            BoardDirection::AntiDiagonal => Self::num_consecutive(anti_diag_search, &|i| {
                self.flat_index(row + anti_diag_min - i, col + i - anti_diag_min)
            }),
        }
    }

    pub fn open_indicies(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut open = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if self.flat_index(row, col).is_none() {
                    open.push((row, col));
                }
            }
        }
        open.into_iter()
    }

    pub fn get_status(&self) -> GameStatus {
        // No move has been made yet (or the invarient is broken...)
        let (col, row) = match self.last_move {
            Some(pair) => pair,
            None => return GameStatus::InProgress,
        };
        // Short-circuit if we can
        if self.num_to_win > self.pieces_placed {
            return GameStatus::InProgress;
        }
        // Could return Chaos victory earlier if Order cannot win
        if (self.pieces_placed + 1) == self.size * self.size {
            return GameStatus::ChaosWins;
        }
        if self.count_direction(BoardDirection::Row, row, col) == self.num_to_win
            || self.count_direction(BoardDirection::Column, row, col) == self.num_to_win
            || self.count_direction(BoardDirection::Diagonal, row, col) == self.num_to_win
            || self.count_direction(BoardDirection::AntiDiagonal, row, col) == self.num_to_win
        {
            return GameStatus::OrderWins;
        }
        GameStatus::InProgress
    }

    pub fn make_move(&self, m: Move) -> Option<Game> {
        //        println!("Made move to {} {}", m.row, m.col);
        if self.flat_index(m.row, m.col).is_some() {
            None
        } else {
            let mut new_board = self.board.clone();
            new_board[m.row * self.size + m.col] = Some(m.piece_type);
            Some(Game {
                size: self.size,
                num_to_win: self.num_to_win,
                board: new_board,
                pieces_placed: self.pieces_placed + 1,
                last_move: Some((m.col, m.row)),
            })
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "     {}    {}    {}    {}    {}   {}", 0, 1, 2, 3, 4, 5)?;
        writeln!(f, "  ------------------------------")?;
        for row in 0..self.size {
            write!(f, "{}", row)?;
            for col in 0..self.size {
                if let Some(cell) = self.flat_index(row, col) {
                    write!(f, " | {:?} ", cell)?;
                } else {
                    write!(f, " |   ")?;
                }
            }
            write!(f, "| ")?;
            writeln!(f, "\n  ------------------------------")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Game, GameStatus, Move, MoveType};

    #[test]
    fn test_horizontal_win_left() {
        let mut game = Game::new();
        let x = MoveType::X;
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_horizontal_win_right() {
        let mut game = Game::new();
        let x = MoveType::X;
        game = game.make_move(Move::new(x, 5, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_vertical_win_up() {
        let mut game = Game::new();
        let x = MoveType::X;
        game = game.make_move(Move::new(x, 1, 5)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_vertical_win_down() {
        let mut game = Game::new();
        let x = MoveType::X;
        game = game.make_move(Move::new(x, 5, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 5, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 5, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 5, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 5, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    fn test_diagonal_win() {
        let mut game = Game::new();
        let x = MoveType::O;
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    fn test_anti_diagonal_win() {
        let mut game = Game::new();
        let x = MoveType::O;
        game = game.make_move(Move::new(x, 4, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_diagonal_win2() {
        let mut game = Game::new();
        let x = MoveType::O;
        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 5)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 5, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_anti_np_win() {
        let mut game = Game::new();
        let x = MoveType::X;
        let o = MoveType::O;
        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(o, 2, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(o, 4, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 5)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(o, 5, 4)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(o, 3, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
    }
}
