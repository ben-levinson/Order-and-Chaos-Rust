///There is no public API calls located in the strategy module. This module contains the
/// implementation details for an Order and Chaos AI.
extern crate rayon;
use crate::board::{BoardDirection, Game, GameStatus, Move, MoveType, Player, Strategy};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::f64::INFINITY;
use std::sync::mpsc::channel;

impl Strategy for Game {
    fn ai_move(&self, player: Player) -> Game {
        let best_move = mini_max(&self, player).unwrap();
        match self.make_move(best_move) {
            Some(g) => g,
            None => {
                println!("Illegal move");
                unreachable!();
            }
        }
    }
}

fn possible_moves(game: &Game) -> impl ParallelIterator<Item = (MoveType, usize, usize)> + '_ {
    game.open_indices()
        .map(|(row, col)| (MoveType::X, row, col))
        .chain(
            game.open_indices()
                .map(|(row, col)| (MoveType::O, row, col)),
        )
        .par_bridge()
}

fn mini_max(game: &Game, player: Player) -> Option<Move> {
    let depth = match player {
        Player::Order => 5,
        Player::Chaos => 4,
    };
    if game.get_status() != GameStatus::InProgress {
        println!("Game not in progress");
        return None;
    }
    let (sender, receiver) = channel();
    possible_moves(game).for_each_with(sender, |s, (move_type, row, col)| {
        let curr_move = Move::new(move_type, row, col);
        let curr_game = game.make_move(curr_move).unwrap();
        let score = alphabeta(curr_game, depth, -INFINITY, INFINITY, player.other_player());
        s.send((score, curr_move)).unwrap();
    });
    let mut best_score = match player {
        Player::Order => -INFINITY,
        Player::Chaos => INFINITY,
    };
    let mut best_moves = Vec::new();
    for (score, curr_move) in receiver {
        let status = game.make_move(curr_move).unwrap().get_status();
        if player == Player::Order && status == GameStatus::OrderWins {
            return Some(curr_move);
        }
        if score == best_score {
            best_moves.push(Some(curr_move));
            continue;
        }
        match player {
            Player::Order => {
                if score > best_score {
                    best_moves.clear();
                    best_score = score;
                    best_moves.push(Some(curr_move));
                }
            }
            Player::Chaos => {
                if best_score > score {
                    best_moves.clear();
                    best_score = score;
                    best_moves.push(Some(curr_move));
                }
            }
        }
    }
    *best_moves.choose(&mut rand::thread_rng()).unwrap()
}

fn alphabeta(game: Game, depth: usize, mut alpha: f64, mut beta: f64, player: Player) -> f64 {
    if depth == 0 || game.get_status() != GameStatus::InProgress {
        return eval(&game);
    }
    let mut value = match player {
        Player::Order => -INFINITY,
        Player::Chaos => INFINITY,
    };
    for (row, col) in game.open_indices() {
        for &move_type in &[MoveType::X, MoveType::O] {
            let curr_move = Move::new(move_type, row, col);
            let next_game = game.make_move(curr_move).expect("Failed to make move");
            let new_val = alphabeta(next_game, depth - 1, alpha, beta, player.other_player());
            match player {
                Player::Order => {
                    value = value.max(new_val);
                    alpha = alpha.max(new_val);
                }
                Player::Chaos => {
                    value = value.min(new_val);
                    beta = beta.min(new_val);
                }
            }
            if alpha >= beta {
                return value;
            }
        }
    }
    value
}

fn eval(game: &Game) -> f64 {
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

#[cfg(test)]
mod minmax_tests {
    use super::{eval, Player};
    use crate::board::{Game, GameStatus, Move, MoveType, Strategy};

    #[test]
    fn score_order_board() {
        let mut game = Game::new();
        let mut score;
        let x = MoveType::X;

        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = eval(&game);
        assert_eq!(score, 0.);

        game = game.make_move(Move::new(x, 2, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = eval(&game);
        assert_eq!(score, 5.);

        game = game.make_move(Move::new(x, 0, 1)).unwrap();
        score = eval(&game);
        assert_eq!(score, 0.);

        game = game.make_move(Move::new(x, 0, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = eval(&game);
        assert_eq!(score, 5.);

        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        score = eval(&game);
        assert_eq!(score, 20.);
    }

    /* The following tests take too long to run on Travis  */

    #[test]
    #[ignore]
    fn test_order_clear_win_horizontal() {
        let mut game = Game::new();
        let x = MoveType::X;
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 0, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.ai_move(Player::Order);
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    #[ignore]
    fn test_order_clear_win_vertical() {
        let mut game = Game::new();
        let x = MoveType::X;
        let o = MoveType::O;
        game = game.make_move(Move::new(o, 0, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 1, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.ai_move(Player::Order);
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    #[ignore]
    fn test_chaos_clear_block() {
        let mut game = Game::new();
        let x = MoveType::X;
        let o = MoveType::O;
        game = game.make_move(Move::new(x, 5, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 4, 1)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 3, 2)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(x, 2, 3)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.make_move(Move::new(o, 0, 5)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        println!("{}", game);
        game = game.ai_move(Player::Chaos);
        println!("{}", game);
        assert_eq!(game.get_status(), GameStatus::InProgress);
        assert_eq!(game.last_move().unwrap().1, 1);
        assert_eq!(game.last_move().unwrap().0, 4);
    }

    #[test]
    #[ignore]
    fn test_open_clear_win_anti_diag() {
        let mut game = Game::new();
        let x = MoveType::X;
        let o = MoveType::O;
        game = game.make_move(Move::new(o, 0, 0)).unwrap();
        game = game.make_move(Move::new(x, 0, 4)).unwrap();
        game = game.make_move(Move::new(x, 1, 1)).unwrap();
        game = game.make_move(Move::new(x, 1, 2)).unwrap();
        game = game.make_move(Move::new(x, 1, 4)).unwrap();
        game = game.make_move(Move::new(x, 2, 1)).unwrap();
        game = game.make_move(Move::new(o, 2, 2)).unwrap();
        game = game.make_move(Move::new(x, 2, 3)).unwrap();
        game = game.make_move(Move::new(o, 2, 4)).unwrap();
        game = game.make_move(Move::new(o, 3, 1)).unwrap();
        game = game.make_move(Move::new(x, 3, 2)).unwrap();
        game = game.make_move(Move::new(x, 4, 1)).unwrap();
        game = game.make_move(Move::new(o, 5, 0)).unwrap();
        assert_eq!(game.get_status(), GameStatus::InProgress);
        game = game.ai_move(Player::Order);
        assert_eq!(game.get_status(), GameStatus::OrderWins);
    }

    #[test]
    fn test_open_indices() {
        let mut game = Game::new();
        let x = MoveType::X;
        let o = MoveType::O;
        game = game.make_move(Move::new(x, 0, 0)).unwrap();
        game = game.make_move(Move::new(x, 1, 0)).unwrap();
        game = game.make_move(Move::new(o, 3, 2)).unwrap();
        game = game.make_move(Move::new(x, 2, 3)).unwrap();
        game = game.make_move(Move::new(o, 2, 4)).unwrap();
        game = game.make_move(Move::new(x, 4, 2)).unwrap();
        let mut count = 0;
        for cell in game.open_indices() {
            count += 1;
            assert_ne!(cell, (0, 0));
            assert_ne!(cell, (1, 0));
            assert_ne!(cell, (3, 2));
            assert_ne!(cell, (2, 3));
            assert_ne!(cell, (2, 4));
            assert_ne!(cell, (4, 2));
        }
        assert_eq!(count, 30);
    }

}
