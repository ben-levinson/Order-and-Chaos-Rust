use project::board::{Cell, Game, GameStatus};
use std::io::{self, BufRead};

fn read_input() -> Option<(Cell, usize, usize)> {
    let mut buffer = String::new();

    let user_input = io::stdin().lock().read_line(&mut buffer);
    if let Ok(n) = user_input {
        if n == 0 {
            return None;
        }
    }

    let mut wh = buffer.split_whitespace();
    let piece = wh.next().unwrap();
    //            println!("piece {} == x {}", piece, piece == "x");
    if piece != "x" && piece != "o" {
        panic!("ERROR");
    }
    let row = wh.next().unwrap().parse::<usize>().unwrap();
    let col = wh.next().unwrap().parse::<usize>().unwrap();
    let cell_type = if piece == "x" { Cell::X } else { Cell::O };
    Some((cell_type, col, row))
}

fn main() {
    let mut game = Game::new();
    let mut turn = false;
    loop {

        let input = read_input();

        if input.is_none() {
            break;
        }
        if turn {
            println!("Order's turn");
            turn = !turn;
        } else {
            println!("Chaos' turn");
            turn = !turn;
        }
        let (piece, col, row) = input.unwrap();
        game = game.make_move(piece, col, row).expect("Illegal move!");
        match game.get_status() {
            GameStatus::InProgress => println!("{}", game),
            GameStatus::OrderWins => {
                println!("Order wins");
                break;
            }
            GameStatus::ChaosWins => {
                println!("Chaos wins");
                break;
            }
        };
        print!("{}[2J", 27 as char);
    }
}
