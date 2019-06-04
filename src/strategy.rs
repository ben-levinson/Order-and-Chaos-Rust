extern crate rayon;
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

pub fn ai_move(game: &Game, player: Player) -> Game {
    let best_move = mini_max(&game, player).unwrap();
    match game.make_move(best_move) {
        Some(g) => g,
        None => {
            println!("Illegal move");
            unreachable!();
        }
    }
}

pub fn mini_max(game: &Game, player: Player) -> Option<Move> {
    println!("Start minimax");
    if game.get_status() != GameStatus::InProgress {
        println!("Game not in progress");
        return None;
    }
    let mut best_move = None;
    let mut best_score = match player {
        Player::Order => -INFINITY,
        Player::Chaos => INFINITY,
    };
    for (row, col) in game.open_indicies() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let curr_move = Move::new(move_type, row, col);
            let curr_game = game.make_move(curr_move).unwrap();
            //let score = minimax_helper(1, curr_game, player);
            let score = alphabeta(curr_game, 4, -INFINITY, INFINITY, player.other_player());
            if score.is_nan() {
                continue;
            }
            dbg!(score);
            dbg!(curr_move);
            match player {
                Player::Order => {
                    if score >= best_score {
                        best_score = score;
                        best_move = Some(curr_move);
                    }
                }
                Player::Chaos => {
                    if best_score >= score {
                        best_score = score;
                        best_move = Some(curr_move);
                    }
                }
            }
        }
    }
    dbg!(best_move);
    best_move
}

fn minimax_helper(depth: usize, game: &Game, player: Player) -> f64 {
    let eval = match player {
        Player::Order => order_eval,
        Player::Chaos => chaos_eval,
    };
    if depth == 0 {
        return order_eval(game);
    }
    let mut best_score = -INFINITY;
    for (row, col) in game.open_indicies() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let curr_move = Move::new(move_type, row, col);
            let curr_game = match game.make_move(curr_move) {
                Some(g) => g,
                None => continue,
            };
            let score = minimax_helper(depth - 1, &curr_game, player.other_player());
            best_score = match player {
                Player::Order => best_score.max(score),
                Player::Chaos => best_score.min(score),
            };
        }
    }
    return best_score;
}

fn alphabeta(game: Game, depth: usize, mut alpha: f64, mut beta: f64, player: Player) -> f64 {
    if depth == 0 || game.get_status() != GameStatus::InProgress {
        return order_eval(&game);
    }
    match player {
        Player::Order => {
            let mut value = -INFINITY;
            for (row, col) in game.open_indicies() {
                for &move_type in &[MoveType::X, MoveType::O] {
                    let curr_move = Move::new(move_type, row, col);
                    let next_game = game.make_move(curr_move).expect("Failed to make move");
                    let new_val = alphabeta(next_game, depth - 1, alpha, beta, Player::Chaos);
                    value = value.max(new_val);
                    alpha = alpha.max(new_val);
                    if alpha >= beta {
                        return value;
                    }
                }
            }
            value
        }
        Player::Chaos => {
            let mut value = INFINITY;
            for (row, col) in game.open_indicies() {
                for &move_type in &[MoveType::X, MoveType::O] {
                    let curr_move = Move::new(move_type, row, col);
                    let next_game = game.make_move(curr_move).expect("Failed to make move");
                    let new_val = alphabeta(next_game, depth - 1, alpha, beta, Player::Order);
                    value = value.min(new_val);
                    beta = beta.min(new_val);
                    if alpha >= beta {
                        return value;
                    }
                }
            }
            value
        }
    }
}

fn order_eval(game: &Game) -> f64 {
    let mut score = match game.get_status() {
        GameStatus::OrderWins => return INFINITY,
        GameStatus::ChaosWins => return -INFINITY,
        GameStatus::InProgress => 0.,
    };
    let (col, row) = game
        .last_move()
        .expect("Eval should never be called on an empty board");
    let counts = &[
        game.count_direction(BoardDirection::Row, row, col),
        game.count_direction(BoardDirection::Column, row, col),
        game.count_direction(BoardDirection::Diagonal, row, col),
        game.count_direction(BoardDirection::AntiDiagonal, row, col),
    ];
    for count in counts {
        score += match count {
            4 => 25.,
            3 => 10.,
            2 => 5.,
            _ => 0.,
        }
    }
    score
}

fn chaos_eval(game: &Game) -> f64 {
    if game.get_status() == GameStatus::ChaosWins {
        return -INFINITY;
    }
    let mut score = 0.;
    for (row, col) in game.open_indicies() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let new_score = order_eval(
                &game.make_move(Move::new(move_type, row, col))
                     .unwrap()
            );
            if new_score > score {
                score = new_score;
            }
        }
    }
    score
}


#[cfg(test)]
mod minmax_tests {
    use crate::board::{Game, GameStatus, Move, MoveType};
    use super::{Player, ai_move, order_eval};

    #[test]
    fn score_order_board() {
        let mut game = Game::new();
        let mut score;
        let x = MoveType::X;

        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = order_eval(&game);
        assert_eq!(score, 0.);

        game = game.make_move(Move::new(x, 2, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = order_eval(&game);
        assert_eq!(score, 5.);

        game = game.make_move(Move::new(x, 0, 1)).unwrap();
        score = order_eval(&game);
        assert_eq!(score, 0.);

        game = game.make_move(Move::new(x, 0, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = order_eval(&game);
        assert_eq!(score, 5.);

        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = order_eval(&game);
        assert_eq!(score, 20.);
    }

    #[test]
    fn test_order_clear_win_horizontal() {
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
        game = ai_move(&game, Player::Order);
        println!("{}", game);
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    fn test_chaos_clear_block_horizontal() {
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
        game = ai_move(&game, Player::Chaos);
        assert_eq!(game.get_status(), GameStatus::InProgress);
        assert_eq!(game.last_move().unwrap().1, 4);
        assert_eq!(game.last_move().unwrap().0, 0);
    }
}
