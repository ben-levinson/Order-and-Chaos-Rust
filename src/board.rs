use crate::board::GameOver::{ChaosWins, OrderWins};
use std::fmt;
use std::thread::current;

#[derive(Debug, Eq, PartialEq)]
pub enum GameOver {
    OrderWins,
    ChaosWins,
}

#[derive(Clone)]
pub struct Game {
    size: usize,
    board: Vec<String>,
    pieces_places: usize,
    num_to_win: usize,
}
//
impl Game {
    pub fn new() -> Self {
        Game {
            size: 6,
            board: vec![" ".to_owned(); 36],
            pieces_places: 0,
            num_to_win: 5,
        }
    }

    ///Returns an option, Some(location) if valid, None if passed space is invalid
    fn place(&mut self, piece: String, location: usize) -> Option<usize> {
        if self.board[location] != " " {
            //Invalid move
            return None;
        }

        self.board[location] = piece;
        self.pieces_places += 1;
        Some(location)
    }

    fn checkGameOver(&self, last_move: usize) -> Option<GameOver> {
        if self.pieces_places == self.size * self.size {
            return Some(ChaosWins);
        }

        //         check each direction for an order win
        if self.check_horizontal(last_move)
            || self.check_vertical(last_move)
            || self.check_primary_diag(last_move)
            || self.check_secondary_diag(last_move)
        {
            return Some(OrderWins);
        }

        None
    }

    fn check_horizontal(&self, start_loc: usize) -> bool {
        let piece_at_loc = self.board[start_loc].clone();
        let mut num_in_row = 0;

        for idx in
            self.size * (start_loc / self.size)..(self.size * (start_loc / self.size)) + self.size
        {
            if self.board[idx] == piece_at_loc {
                num_in_row += 1;
            }
            if self.board[idx] == " " {
                continue;
            }
            if self.board[idx] != piece_at_loc {
                break;
            }
        }

        if num_in_row >= self.num_to_win {
            return true;
        }

        false
    }

    fn check_vertical(&self, start_loc: usize) -> bool {
        let piece_at_loc = self.board[start_loc].clone();
        let mut num_in_row = 0;

        let mut curr = start_loc % self.size;
        while !self.board.get(curr).is_none() {
            if self.board[curr] == piece_at_loc {
                num_in_row += 1;
            }
            if self.board[curr] == " " {
                curr += self.size;
                continue;
            }
            if self.board[curr] != piece_at_loc {
                break;
            }
            curr += self.size;
        }

        if num_in_row >= self.num_to_win {
            return true;
        }
        false
    }

    fn check_primary_diag(&self, start_loc: usize) -> bool {
        let piece_at_loc = self.board[start_loc].clone();
        let mut num_in_row = 0;
        let mut curr = start_loc % 7;

        while !self.board.get(curr).is_none() {
            if self.board[curr] == piece_at_loc {
                num_in_row += 1;
            }
            if self.board[curr] == " " {
                curr += 7;
                continue;
            }
            if self.board[curr] != piece_at_loc {
                break;
            }
            curr += 7;
        }

        if num_in_row >= self.num_to_win {
            return true;
        }
        false
    }

    fn check_secondary_diag(&self, start_loc: usize) -> bool {
        let piece_at_loc = self.board[start_loc].clone();
        let mut num_in_row = 0;
        let mut curr = start_loc % self.size;

        while !self.board.get(curr).is_none() {
            if self.board[curr] == piece_at_loc {
                num_in_row += 1;
            }
            if self.board[curr] == " " {
                curr += 5;
                continue;
            }
            if self.board[curr] != piece_at_loc {
                break;
            }
            curr += 5;
        }

        if num_in_row >= self.num_to_win {
            return true;
        }
        false
    }

    pub fn make_move(&mut self, piece: String, location: usize) -> Option<GameOver> {
        if self.place(piece, location.clone()).is_none() {
            panic!("Invalid move");
        } else {
            return self.checkGameOver(location);
        }
        None
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "     {}    {}    {}    {}    {}   {}", 0, 1, 2, 3, 4, 5)?;
        writeln!(f, "  ------------------------------")?;
        for row in 0..self.size {
            write!(f, "{}", row);
            for col in 0..self.size {
                write!(f, " | {} ", self.board[row * self.size + col])?;
            }
            write!(f, "| ")?;
            writeln!(f, "\n  ------------------------------")?;
        }
        Ok(())
    }
}

//
//
#[cfg(test)]
mod test {
    use crate::board::Game;
    use crate::board::GameOver::OrderWins;

    #[test]
    fn test_horizontal_win_left() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let mut res = game.make_move(x.clone(), 0);

        assert_eq!(res, None);
        res = game.make_move(x.clone(), 1);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 2);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 3);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 4);
        assert_eq!(res, Some(OrderWins));
    }

    #[test]
    fn test_horizontal_win_right() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let mut res = game.make_move(x.clone(), 4);

        assert_eq!(res, None);
        res = game.make_move(x.clone(), 3);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 2);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 1);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 0);
        assert_eq!(res, Some(OrderWins));
    }

    #[test]
    fn test_horizontal_mix() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let mut res = game.make_move(x.clone(), 11);

        assert_eq!(res, None);
        res = game.make_move(x.clone(), 7);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 9);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 10);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 8);
        assert_eq!(res, Some(OrderWins));
    }

    #[test]
    fn test_vertical_loss() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let o = "o".to_owned();
        let mut res = game.make_move(x.clone(), 0);

        assert_eq!(res, None);
        res = game.make_move(o.clone(), 12);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 8);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 6);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 30);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 24);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 18);
        assert_eq!(res, None);
    }

    #[test]
    fn test_vertical_win() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let o = "o".to_owned();
        let mut res = game.make_move(x.clone(), 0);

        assert_eq!(res, None);
        res = game.make_move(x.clone(), 12);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 24);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 6);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 8);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 30);
        assert_eq!(res, None);
        res = game.make_move(x.clone(), 18);
        assert_eq!(res, Some(OrderWins));
    }

    #[test]
    fn test_diags1() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let o = "o".to_owned();
        let mut res = game.make_move(o.clone(), 0);

        assert_eq!(res, None);
        res = game.make_move(o.clone(), 7);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 24);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 19);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 14);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 9);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 4);
        assert_eq!(res, Some(OrderWins));
    }

    #[test]
    fn test_diags2() {
        let mut game = Game::new();
        let x = "x".to_owned();
        let o = "o".to_owned();
        let mut res = game.make_move(o.clone(), 6);

        assert_eq!(res, None);
        res = game.make_move(o.clone(), 13);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 20);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 27);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 34);
        assert_eq!(res, Some(OrderWins));
        res = game.make_move(o.clone(), 9);
        assert_eq!(res, None);
        res = game.make_move(o.clone(), 4);
        assert_eq!(res, None);
    }
}
