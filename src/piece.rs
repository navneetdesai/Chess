use crate::*;
use std::fmt::Display;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Piece {
    pub fn get_color(&self) -> &Color {
        match self {
            Piece::King(color) => color,
            Piece::Queen(color) => color,
            Piece::Rook(color) => color,
            Piece::Bishop(color) => color,
            Piece::Knight(color) => color,
            Piece::Pawn(color) => color,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Piece::King(color) => match color {
                Color::White => write!(f, "♔"),
                Color::Black => write!(f, "{}", String::from("♚").black()),
            },
            Piece::Queen(color) => match color {
                Color::White => write!(f, "♕"),
                Color::Black => write!(f, "{}", String::from("♛").black()),
            },
            Piece::Rook(color) => match color {
                Color::White => write!(f, "♖"),
                Color::Black => write!(f, "{}", String::from("♜").black()),
            },
            Piece::Bishop(color) => match color {
                Color::White => write!(f, "♗"),
                Color::Black => write!(f, "{}", String::from("♝").black()),
            },
            Piece::Knight(color) => match color {
                Color::White => write!(f, "♘"),
                Color::Black => write!(f, "{}", String::from("♞").black()),
            },
            Piece::Pawn(color) => match color {
                Color::White => write!(f, "♙"),
                Color::Black => write!(f, "{}", String::from("♟").black()),
            },
        }
    }
}
