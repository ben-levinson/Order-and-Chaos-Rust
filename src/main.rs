use project::board::{Game, GameOver};
use std::io::{self, BufRead};

fn read_input() -> Option<(String, usize)> {
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
    Some((piece.to_owned(), row*6+col))
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
        let (piece, loc) = input.unwrap();

        match game.make_move(piece, loc) {
            Some(winner) => {
                match winner {
                    GameOver::OrderWins => println!("Order wins"),
                    GameOver::ChaosWins => println!("Chaos wins"),
                }
                println!("Game Over!");
                break;
            }
            None => println!("{}", game),
        };
        print!("{}[2J", 27 as char);
    }
}
