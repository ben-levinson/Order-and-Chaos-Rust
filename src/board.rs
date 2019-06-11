use std::cmp::{max, min};
use std::fmt;

///A Player is either Order or Chaos.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    Order,
    Chaos,
}

impl Player {
    ///Print out the current player type.
    pub fn display(&self) -> &'static str {
        match self {
            Player::Order => "Order",
            Player::Chaos => "Chaos",
        }
    }
    ///Get the type of the other player.
    pub fn other_player(&self) -> Self {
        match self {
            Player::Order => Player::Chaos,
            Player::Chaos => Player::Order,
        }
    }
}

///A game of Order and Chaos may be in progress, or won by either player.
#[derive(Debug, Eq, PartialEq)]
pub enum GameStatus {
    InProgress,
    OrderWins,
    ChaosWins,
}

///A move in Order and Chaos is either placing an X piece or an O piece.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveType {
    X,
    O,
}

const SIZE: usize = 6;

///A move consists of a piece and a location in the board.
#[derive(Clone, Copy, Debug)]
pub struct Move {
    piece_type: MoveType,
    row: usize,
    col: usize,
}

impl Move {
    ///Creates a new move with the specified MoveType and location.
    pub fn new(piece_type: MoveType, row: usize, col: usize) -> Self {
        Move {
            piece_type: piece_type,
            row: row,
            col: col,
        }
    }
}

///Defines directions for checking if the game is in a won state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardDirection {
    Row,
    Column,
    Diagonal,
    AntiDiagonal,
}

///A game of order and chaos.
#[derive(Clone)]
pub struct Game {
    size: usize,
    num_to_win: usize,
    board: [[Option<MoveType>; SIZE]; SIZE],
    pieces_placed: usize,
    last_move: Option<(usize, usize)>,
}

impl Game {
    ///Create a new game.
    pub fn new() -> Self {
        Game {
            size: SIZE,
            board: [[None; SIZE];SIZE],
            pieces_placed: 0,
            num_to_win: 5,
            last_move: None,
        }
    }
    ///Return the coordinates of the last move made. Returns none if the game is over.
    pub fn last_move(&self) -> Option<(usize, usize)> {
        self.last_move.clone()
    }

    ///Query the board for the Piece at a given location.
    pub fn index(&self, row: usize, col: usize) -> Option<MoveType> {
        self.board[row][col].clone()
    }

    ///Get the size of the board.
    pub fn size(&self) -> usize {
        self.size
    }

    ///Get the number of like pieces in a given direction.
    fn num_consecutive(to_search: usize, f: impl Fn(usize) -> Option<MoveType>) -> usize {
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

    ///Counts the number of like pieces in a given direction on the board. Used
    ///to determine whether a win state has been reached.
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
            BoardDirection::Row => Self::num_consecutive(self.size, &|i| self.index(row, i)),
            BoardDirection::Column => Self::num_consecutive(self.size, &|i| self.index(i, col)),
            BoardDirection::Diagonal => Self::num_consecutive(diag_search, &|i| {
                self.index(row + i - diag_min, col + i - diag_min)
            }),
            BoardDirection::AntiDiagonal => Self::num_consecutive(anti_diag_search, &|i| {
                self.index(row + anti_diag_min - i, col + i - anti_diag_min)
            }),
        }
    }

    ///Get a list of the open spaces in the game.
    pub fn open_indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter()
                .enumerate()
                .flat_map(move |(y, column)| match column {
                    Some(_) => None,
                    None => Some((x, y)),
                }))
    }

    ///Query the status of the game. Has a player won or is the game still in progress.
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
        if self.pieces_placed == self.size * self.size {
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

    ///Places a piece with a location specified by the move into the game. Returns none if the game
    /// has been won or if the move is invalid.
    pub fn make_move(&self, m: Move) -> Option<Game> {
        if self.index(m.row, m.col).is_some() || self.get_status() != GameStatus::InProgress {
            None
        } else {
            let mut new_board = self.board.clone();
            new_board[m.row][m.col] = Some(m.piece_type);
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
    ///Command line representation of the game. Useful for visualizing AI moves, or tests on Travis.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "     {}    {}    {}    {}    {}   {}", 0, 1, 2, 3, 4, 5)?;
        writeln!(f, "  ------------------------------")?;
        for row in 0..self.size {
            write!(f, "{}", row)?;
            for col in 0..self.size {
                if let Some(cell) = self.index(row, col) {
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

///Provides the strategy for an AI player to play the game. A new AI player can
/// be implemented simply by defining a new implementation for ai_move.
pub trait Strategy {
    ///Specified computer player makes a move in the current game.
    fn ai_move(&self, player: Player) -> Self;
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
