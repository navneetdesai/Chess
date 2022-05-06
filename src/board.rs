use crate::piece::Color::{Black, White};
use crate::piece::{Color, Piece};
use crate::square::Square;
use colored::*;
use std::fmt::format;

const ROWS: usize = 8;
const FILES: usize = 8;

#[derive(Debug, Clone)]
pub struct Board {
    pub(crate) squares: Vec<Vec<Square>>,
    white_king_position: (isize, isize),
    black_king_position: (isize, isize),
}

impl Board {
    /// Creates a new board with the standard chess configuration
    pub fn new() -> Self {
        // create board with starting configuration
        let mut squares = Self::init_empty_board();

        // place pieces according to standard chess starting configuration
        for file in 0..=7 {
            squares[1][file] = Square::new(Some(Piece::Pawn(Black)));
            squares[6][file] = Square::new(Some(Piece::Pawn(White)));
            match file {
                4 => {
                    Self::place_piece(&mut squares, Piece::King(Black), Piece::King(White), file);
                }
                3 => {
                    Self::place_piece(&mut squares, Piece::Queen(Black), Piece::Queen(White), file);
                }
                0 | 7 => {
                    Self::place_piece(&mut squares, Piece::Rook(Black), Piece::Rook(White), file);
                }
                2 | 5 => {
                    Self::place_piece(
                        &mut squares,
                        Piece::Bishop(Black),
                        Piece::Bishop(White),
                        file,
                    );
                }
                1 | 6 => {
                    Self::place_piece(
                        &mut squares,
                        Piece::Knight(Black),
                        Piece::Knight(White),
                        file,
                    );
                }
                _ => (),
            }
        }
        Board {
            squares,
            white_king_position: (7, 4),
            black_king_position: (0, 4),
        }
    }

    fn place_piece(squares: &mut Vec<Vec<Square>>, piece1: Piece, piece2: Piece, file: usize) {
        squares[0][file].place_piece(piece1);
        squares[7][file].place_piece(piece2);
    }

    pub fn get_piece(&self, row: isize, file: isize) -> &Option<Piece> {
        let board = &self.squares;
        &board[row as usize][file as usize].piece
    }

    pub fn set_piece(&mut self, row: isize, file: isize, piece: Piece) {
        let mut board = &mut self.squares;
        board[row as usize][file as usize].place_piece(piece);
    }

    pub fn remove_piece(&mut self, row: isize, file: isize) {
        let mut board = &mut self.squares;
        board[row as usize][file as usize].remove_piece();
    }

    pub fn get_king_position(&self, color: Color) -> (isize, isize) {
        match color {
            Color::White => self.white_king_position,
            Color::Black => self.black_king_position,
        }
    }

    pub fn set_king_position(&mut self, color: Color, position: (isize, isize)) {
        match color {
            Color::White => self.white_king_position = position,
            Color::Black => self.black_king_position = position,
        }
    }

    /// Returns a new board with all squares set to None
    fn init_empty_board() -> Vec<Vec<Square>> {
        // have to take this approach because Square does not implement Copy
        let mut squares: Vec<Vec<Square>> = vec![];
        for _ in 0..ROWS {
            let mut row = vec![];
            for _ in 0..=FILES {
                row.push(Square::new(None));
            }
            squares.push(row);
        }
        squares
    }

    /// Prints the board to the terminal in a user-friendly format
    pub fn pretty_print(&self) {
        println!("{}", self.chess_print());
    }

    /// Returns a string representation of the board
    /// that can be printed to the terminal or
    /// transferred over the network
    pub fn chess_print(&self) -> String {
        let board = &self.squares;
        let mut repr = format!("{}", "  a  b  c  d  e  f  g  h  \n");
        for row in 0..ROWS {
            repr = format!("{}{}", repr, 8 - row);
            for file in 0..FILES {
                let character = match &board[row][file].piece {
                    None => String::from("   "),
                    Some(piece) => String::from(format!(" {} ", piece)),
                };
                if (row + file) % 2 == 0 {
                    repr = format!("{}{}", repr, character.on_white());
                } else {
                    repr = format!("{}{}", repr, character.on_cyan());
                }
            }
            repr = format!("{}\n", repr);
        }
        repr = format!("{}{}\n", repr, "  a  b  c  d  e  f  g  h  \n");
        repr
    }
}
