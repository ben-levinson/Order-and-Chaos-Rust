use crate::board::{BoardDirection, Game, GameStatus, Move, MoveType};
use rayon::prelude::*;
use std::f64::INFINITY;

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
    pub fn other_player(&self) -> Self {
        match self {
            Player::Order => Player::Chaos,
            Player::Chaos => Player::Order,
        }
    }
}

pub fn ai_move(game: Game, player: Player) -> Game {
    let best_move = mini_max(game.clone(), player).unwrap();
    game.make_move(best_move).unwrap()
}

pub fn mini_max(game: Game, player: Player) -> Option<Move> {
    if game.get_status() != GameStatus::InProgress {
        return None;
    }
    let other_player = player.other_player();
    let mut best_move = None;
    let mut best_score = -INFINITY;
    for (row, col) in game.open_indicies() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let curr_move = Move::new(move_type, row, col);
            let curr_game = game.make_move(curr_move).unwrap();
            let score = minimax_helper(3, curr_game, other_player);
            if score > best_score {
                best_score = score;
                best_move = Some(curr_move);
            }
        }
    }
    best_move
}

fn minimax_helper(depth: usize, game: Game, player: Player) -> f64 {
    let eval = match player {
        Player::Order => order_eval,
        Player::Chaos => chaos_eval,
    };
    if depth == 0 {
        return eval(game);
    }
    let mut best_score = -INFINITY;
    for (row, col) in game.open_indicies() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let curr_move = Move::new(move_type, row, col);
            let curr_game = match game.make_move(curr_move) {
                Some(g) => g,
                None => continue,
            };
            let score = minimax_helper(depth - 1, curr_game, player.other_player());
            if score > best_score {
                best_score = score;
            }
        }
    }
    return best_score;
}

fn order_eval(game: Game) -> f64 {
    let (col, row) = game
        .last_move()
        .expect("Eval should never be called on an empty board");
    let counts = &[
        game.count_direction(BoardDirection::Row, row, col),
        game.count_direction(BoardDirection::Column, row, col),
        game.count_direction(BoardDirection::Diagonal, row, col),
        game.count_direction(BoardDirection::AntiDiagonal, row, col),
    ];
    let mut score = 0.;
    for count in counts {
        score = match count {
            5 => score + 100.,
            4 => score + 25.,
            3 => score + 10.,
            2 => score + 5.,
            _ => score,
        }
    }
    score
}

fn chaos_eval(game: Game) -> f64 {
    if game.get_status() == GameStatus::ChaosWins {
        return 100.;
    }
    let mut score = 0.;
    for (row, col) in game.open_indicies() {
        let x_score = order_eval(
            game.clone()
                .make_move(Move::new(MoveType::X, row, col))
                .unwrap(),
        );
        let o_score = order_eval(
            game.clone()
                .make_move(Move::new(MoveType::O, row, col))
                .unwrap(),
        );
        if x_score > o_score && x_score > score {
            score = x_score;
        } else if o_score > x_score && o_score > score {
            score = o_score;
        }
    }
    -score
}
