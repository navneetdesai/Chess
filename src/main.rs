mod board;
mod chess;
mod error;
mod piece;
mod player;
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
    let name1 = get_name("player-1: (white)");
    let name2 = get_name("player-2: (black)");
    (String::from(name1.trim()), String::from(name2.trim()))
}

fn get_name(prompt: &str) -> String {
    print!("Please enter name for {}: ", prompt);
    let mut name = String::new();
    stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Umm, system crashed. Please restart.");
    name
}
