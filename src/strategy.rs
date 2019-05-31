use crate::board::{BoardDirection, Game, GameStatus, Move, MoveType};
use rayon::prelude::*;
use std::cmp::max;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    Order,
    Chaos,
}

impl<'a> Player {
    pub fn display(&self) -> &'a str {
        match self {
            Player::Order => "Order",
            Player::Chaos => "Chaos",
        }
    }
}

pub fn mini_max(game: Game, player: Player) -> Move {
    
    Move::new(MoveType::O, 0, 0)
}

fn order_eval(game: Game) -> isize {
    let (col, row) = game
        .last_move()
        .expect("Eval should never be called on an empty board");
    let counts = &[
        game.count_direction(BoardDirection::Row, row, col),
        game.count_direction(BoardDirection::Column, row, col),
        game.count_direction(BoardDirection::Diagonal, row, col),
        game.count_direction(BoardDirection::AntiDiagonal, row, col),
    ];
    let mut score = 0;
    for count in counts {
        score = match count {
            5 => score + 100,
            4 => score + 25,
            3 => score + 10,
            2 => score + 5,
            _ => score,
        }
    }
    score
}

fn chaos_eval(game: Game) -> isize {
    if game.get_status() == GameStatus::ChaosWins {
        return 100;
    }
    let mut score = 0;
    for (row, col) in game.open_indicies() {
        score += max(
            order_eval(game.make_move(Move::new(MoveType::X, row, col)).unwrap()),
            order_eval(game.make_move(Move::new(MoveType::O, row, col)).unwrap()),
        )
    }
    -score
}
