mod board;
mod chess;
mod piece;
mod player;
mod error;
mod square;
use crate::chess::Chess;
use colored::*;
use std::io::{stdout, Write};

fn main() {
    let (name1, name2) = crate::get_player_names();
    let mut game = Chess::new(String::from(name1), String::from(name2));
    game.start();
}

/// Gets the player names
pub fn get_player_names() -> (String, String) {
    print!("Please enter name for player 1(white): ");
    let mut name1 = String::new();
    stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut name1)
        .expect("Umm, system crashed. Please restart.");
    print!("\nPlease enter name for player 2(black): ");
    let mut name2 = String::new();
    stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut name2)
        .expect("Umm, system crashed. Please restart.");
    (String::from(name1.trim()), String::from(name2.trim()))
}
