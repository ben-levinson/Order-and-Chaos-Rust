use std::cmp::{max, min};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum GameStatus {
    InProgress,
    OrderWins,
    ChaosWins,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    X,
    O,
}

#[derive(Clone)]
pub struct Game {
    size: usize,
    num_to_win: usize,
    board: Vec<Option<Cell>>,
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

    pub fn flat_index(&self, i: usize, j: usize) -> Option<Cell> {
        self.board[j * self.size + i].clone()
    }

    fn num_consecutive(to_search: usize, f: &Fn(usize) -> Option<Cell>) -> usize {
        let mut max_count = 0;
        let mut count = 0;
        let mut cell_type = Cell::X;
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
        if false
        // Check col
        || Self::num_consecutive(self.size, &|j|
            self.flat_index(col, j)) == self.num_to_win
        // Check row
        || Self::num_consecutive(self.size, &|i|
            self.flat_index(i, row)) == self.num_to_win
        // Check diagonal
        || Self::num_consecutive(diag_search, &|i|
            self.flat_index(col + i - diag_min, row + i - diag_min)) == self.num_to_win
        // Check anti-diagonal
        || Self::num_consecutive(anti_diag_search, &|i|
            self.flat_index(col + i - anti_diag_min, row + anti_diag_min - i)) == self.num_to_win
        {
            return GameStatus::OrderWins;
        }
        GameStatus::InProgress
    }

    pub fn make_move(&self, piece: Cell, col: usize, row: usize) -> Option<Game> {
        if self.flat_index(col, row).is_some() {
            None
        } else {
            let mut new_board = self.board.clone();
            new_board[row * self.size + col] = Some(piece);
            Some(Game {
                size: self.size,
                num_to_win: self.num_to_win,
                board: new_board,
                pieces_placed: self.pieces_placed + 1,
                last_move: Some((col, row)),
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
                if let Some(cell) = self.flat_index(col, row) {
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
    use super::{Game, GameStatus, Cell};

    #[test]
    fn test_horizontal_win_left() {
        let mut game = Game::new();
        let x = Cell::X;
        game = game.make_move(x, 0, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 2, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 4, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_horizontal_win_right() {
        let mut game = Game::new();
        let x = Cell::X;
        game = game.make_move(x, 5, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 4, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 2, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_vertical_win_up() {
        let mut game = Game::new();
        let x = Cell::X;
        game = game.make_move(x, 1, 5).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_vertical_win_down() {
        let mut game = Game::new();
        let x = Cell::X;
        game = game.make_move(x, 5, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 5, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 5, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 5, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 0, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 5, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    fn test_diagonal_win() {
        let mut game = Game::new();
        let x = Cell::O;
        game = game.make_move(x, 0, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 2, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 4, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    
    #[test]
    fn test_anti_diagonal_win() {
        let mut game = Game::new();
        let x = Cell::O;
        game = game.make_move(x, 4, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 2, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 1, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 0, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 0, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_diagonal_win2() {
        let mut game = Game::new();
        let x = Cell::O;
        game = game.make_move(x, 1, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 2, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 4, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 0, 5).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 5, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }
    #[test]
    fn test_anti_np_win() {
        let mut game = Game::new();
        let x = Cell::X;
        let o = Cell::O;
        game = game.make_move(x, 1, 0).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(o, 2, 1).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 3, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(o, 4, 3).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(x, 0, 5).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(o, 5, 4).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(o, 3, 2).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
    }
}

