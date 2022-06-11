use std::fmt::format;
use crate::board::Board;
use crate::error::Error;
use crate::piece::{Color, Piece};
use crate::player::Player;
use std::io::{stdin, stdout, Write};

const PLAYERS: usize = 2;
const ROWS: isize = 8;
const COLS: isize = 8;
const LEGAL_KNIGHT_MOVES: [(isize, isize); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
];

const LEGAL_KING_MOVES: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const LEGAL_PAWN_MOVES: [(isize, isize); 4] = [(1, 0), (2, 0), (1, 1), (1, -1)];

pub struct Chess {
    chessboard: Board,
    players: [Player; 2],
    current_turn: usize,
    castling_rights: [[bool; 2]; PLAYERS], // queen side and king side castling per player
    move_counter: usize,
}

impl Chess {
    /// Returns a new instance of the game
    pub fn new(player1: String, player2: String) -> Self {
        Chess {
            chessboard: Board::new(),
            players: [
                Player::new(player1, Color::White),
                Player::new(player2, Color::Black),
            ],
            current_turn: 0,
            castling_rights: [[true, true], [true, true]],
            move_counter: 0,
        }
    }

    /// Driver code for the game (1v1 terminal)
    pub fn start(&mut self) {
        loop {
            self.chessboard.pretty_print();
            let current_player = &self.players[self.current_turn];
            print!("{}: ", current_player.get_name());
            let (source, destination) = Self::get_move();
            match self.make_a_move(source, destination) {
                Ok(()) => {
                    if self.is_under_checkmate(self.players[self.current_turn].get_color().other())
                    {
                        println!(
                            "Checkmate! {} wins!",
                            self.players[self.current_turn].get_name()
                        );
                        break;
                    }
                    self.current_turn = (self.current_turn + 1) % 2;
                }
                Err(e) => match e {
                    Error::InvalidMove(msg) => println!("Invalid Move: {}", msg),
                    Error::InvalidDestination(msg) => println!("Invalid Destination: {}", msg),
                    Error::InvalidSource(msg) => println!("Invalid Source: {}", msg),
                    Error::KingUnderCheck(msg) => println!("King under check{}", msg),
                    Error::Checkmate(msg) => println!("Checkmate: {}", msg),
                    Error::InvalidPromotion(msg) => println!("Invalid Promotion: {}", msg),
                    Error::Dummy => println!("Dummy"),
                },
            }
        }
    }

    /// Driver code for the game (1v1 terminal)
    fn make_a_move(
        &mut self,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        let piece = self._validate_move_generic(source, destination)?;
        let destination_piece = self.get_piece(destination.0, destination.1).clone();
        let initial_king_position = self
            .chessboard
            .get_king_position(*(self.players[self.current_turn].get_color()));
        let initial_castling_rights = self.castling_rights.clone();
        match piece {
            Piece::Pawn(color) => self.move_pawn(color, source, destination, false)?,
            Piece::Rook(color) => self.move_rook(color, source, destination)?,
            Piece::Knight(color) => self.move_knight(color, source, destination)?,
            Piece::Bishop(color) => self.move_bishop(color, source, destination)?,
            Piece::Queen(color) => self.move_queen(color, source, destination)?,
            Piece::King(color) => self.move_king(color, source, destination)?,
        }
        if self.is_under_check(*piece.get_color()) {
            self.revert_game_state(
                source,
                destination,
                piece,
                destination_piece,
                initial_king_position,
                initial_castling_rights,
            )?;
        }
        Ok(())
    }

    /// Reverts the game state (should be called if a move leads to or maintains check for the current player)
    fn revert_game_state(
        &mut self,
        source: (isize, isize),
        destination: (isize, isize),
        piece: Piece,
        destination_piece: Option<Piece>,
        initial_king_position: (isize, isize),
        initial_castling_rights: [[bool; 2]; 2],
    ) -> Result<(), Error> {
        self.chessboard.set_piece(source.0, source.1, piece.clone());
        match destination_piece.is_some() {
            true => {
                self.chessboard
                    .set_piece(destination.0, destination.1, destination_piece.unwrap())
            }
            false => self.chessboard.remove_piece(destination.0, destination.1),
        }
        self.chessboard
            .set_king_position(*piece.get_color(), initial_king_position);
        self.castling_rights = initial_castling_rights;
        Err(Error::KingUnderCheck(format!(
            "Cannot move! King is/will be under check"
        )))
    }

    /// Validations:
    /// 1. Source is valid
    /// 2. Destination is valid
    /// 3. Source has a piece of right color
    /// 4. Destination is empty or has a piece of the opposite color
    fn _validate_move_generic(
        &mut self,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<Piece, Error> {
        if destination.1 > COLS - 1
            || destination.1 < 0
            || destination.0 > ROWS - 1
            || destination.0 < 0
        {
            return Err(Error::InvalidDestination(format!(
                "Destination square out of the board: {:?}",
                destination
            )));
        }
        if source.1 > COLS - 1 || source.1 < 0 || source.0 > ROWS - 1 || source.0 < 0 {
            return Err(Error::InvalidSource(format!(
                "Source square out of the board: {:?}",
                source
            )));
        }
        let piece = self.get_piece(source.0, source.1);
        if piece.is_none() {
            return Err(Error::InvalidSource(format!("No piece at {:?}", source)));
        }
        let piece = piece.unwrap();
        if piece.get_color() != self.players[self.current_turn].get_color() {
            return Err(Error::InvalidMove(format!("Not your turn")));
        }
        let destination_piece = self.get_piece(destination.0, destination.1).clone();
        if destination_piece.is_some()
            && destination_piece.unwrap().get_color() == piece.get_color()
        {
            return Err(Error::InvalidMove(format!("Can't capture your own piece")));
        }
        Ok(piece)
    }

    /// Validates pawn move and changes game state
    fn move_pawn(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
        check: bool,
    ) -> Result<(), Error> {
        let (x, starting_x, front_square) = match color {
            Color::White => (2, 6, 1),
            Color::Black => (-2, 1, -1)
        };
        // Non-capturing move
        if source.1 == destination.1 {
            if source.0 == destination.0 + x && source.0 == starting_x {
                if self.get_piece(destination.0 + front_square, destination.1).is_some()
                    || self.get_piece(destination.0, destination.1).is_some() {
                        return Err(Error::InvalidMove(format!(
                            "Can't move to {:?}, obstacles between source and destination",
                            destination
                        )));
                }
            } else if source.0 == destination.0 + front_square {
                if self.get_piece(destination.0, destination.1).is_some(){
                    return Err(Error::InvalidMove(format!(
                            "Can't move to {:?}, cannot capture this piece",
                            destination
                        )));
                }
            } else {
                return Err(Error::InvalidMove(format!(
                        "Can't move to {:?}, Pawn moves one square ahead or captures diagonally",
                        destination
                    )));
            }
        // Capturing move
        } else if (source.1 == destination.1 - 1 || source.1 == destination.1 + 1)
            && source.0 == destination.0 + front_square {
            let destination_piece = self.get_piece(destination.0, destination.1);
            if destination_piece.is_none() {
                return Err(Error::InvalidMove(format!(
                        "Can't move to {:?}, capturing move should have a piece at destination",
                        destination
                    )));
            }
        } else {
            return Err(Error::InvalidMove(format!("Invalid pawn move!")));
        }
        self.promote_pawn(color, source, destination, check)?;
        self._move_piece(source, destination);
        Ok(())
    }

    /// Promotes the pawn to the required piece otherwise returns an error
    fn promote_pawn(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
        check: bool,
    ) -> Result<(), Error> {
        if (destination.0 == 0 && color == Color::White)
            || (destination.0 == 7 && color == Color::Black) && !check
        {
            let mut piece = String::new();
            println!("Enter Promotion (Q|R|N|B):");
            stdin()
                .read_line(&mut piece)
                .expect("Oops! Something went wrong. Please restart.");
            let piece = piece.trim();
            match piece {
                "Q" => self
                    .chessboard
                    .set_piece(source.0, source.1, Piece::Queen(color)),
                "R" => self
                    .chessboard
                    .set_piece(source.0, source.1, Piece::Rook(color)),
                "N" => self
                    .chessboard
                    .set_piece(source.0, source.1, Piece::Knight(color)),
                "B" => self
                    .chessboard
                    .set_piece(source.0, source.1, Piece::Bishop(color)),
                _ => {
                    println!("Invalid promotion");
                    return Err(Error::InvalidPromotion(format!("Invalid promotion")));
                }
            }
        }
        Ok(())
    }

    /// Moves the rook if the move is valid
    fn move_rook(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        // validate its either in the same row or same column
        if source.0 != destination.0 && source.1 != destination.1 {
            return Err(Error::InvalidMove(format!("Invalid move")));
        }
        match source.0 == destination.0 {
            true => {
                // move along the same row
                match source.1 > destination.1 {
                    // move to the left if true else right
                    true => self.rook_validation_helper(
                        source,
                        destination,
                        destination.1 + 1,
                        source.1,
                        "ROW",
                    )?,
                    false => self.rook_validation_helper(
                        source,
                        destination,
                        source.1 + 1,
                        destination.1,
                        "ROW",
                    )?,
                }
            }
            false => {
                // move along the same column
                match source.0 > destination.0 {
                    // move up if true else down
                    true => self.rook_validation_helper(
                        source,
                        destination,
                        destination.0 + 1,
                        source.0,
                        "COLUMN",
                    )?,
                    false => self.rook_validation_helper(
                        source,
                        destination,
                        source.0 + 1,
                        destination.0,
                        "COLUMN",
                    )?,
                }
            }
        }
        // update castling rights
        match color {
            Color::White => match source.1 {
                0 => self.castling_rights[0][0] = false,
                7 => self.castling_rights[0][1] = false,
                _ => (),
            },
            Color::Black => match source.1 {
                0 => self.castling_rights[1][0] = false,
                7 => self.castling_rights[1][1] = false,
                _ => (),
            },
        }
        self._move_piece(source, destination);
        Ok(())
    }

    /// Validation helper for rook
    /// Returns Error if there is a piece in between start and end along the dimension
    fn rook_validation_helper(
        &self,
        source: (isize, isize),
        destination: (isize, isize),
        start: isize,
        end: isize,
        direction: &str,
    ) -> Result<(), Error> {
        for index in start..end {
            let piece = if direction == "ROW" {
                self.get_piece(source.0, index)
            } else {
                self.get_piece(index, source.1)
            };
            if piece.is_some() {
                return Err(Error::InvalidMove(format!(
                    "Can't move to {:?}",
                    destination
                )));
            }
        }
        Ok(())
    }

    /// Moves the Knight if source -> destination is valid for that Knight
    fn move_knight(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        let diff = (destination.0 - source.0, destination.1 - source.1);
        match LEGAL_KNIGHT_MOVES.contains(&diff) {
            true => {
                self._move_piece(source, destination);
                Ok(())
            }
            false => Err(Error::InvalidMove(format!(
                "Can't move to {:?}",
                destination
            ))),
        }
    }

    /// Moves the bishop if possible else returns an Error
    fn move_bishop(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        if source.0 + source.1 != destination.0 + destination.1
            && source.0 - source.1 != destination.0 - destination.1
        {
            return Err(Error::InvalidMove(format!("Invalid move")));
        }
        match source.0 > destination.0 {
            true => match source.1 > destination.1 {
                true => self.bishop_validation_helper(
                    source,
                    destination,
                    1,
                    source.0 - destination.0,
                    "top-left",
                )?,
                false => self.bishop_validation_helper(
                    source,
                    destination,
                    1,
                    source.0 - destination.0,
                    "top-right",
                )?,
            },
            false => match source.1 > destination.1 {
                true => self.bishop_validation_helper(
                    source,
                    destination,
                    1,
                    source.1 - destination.1,
                    "down-left",
                )?,
                false => self.bishop_validation_helper(
                    source,
                    destination,
                    1,
                    source.1 - destination.1,
                    "down-right",
                )?,
            },
        }
        self._move_piece(source, destination);
        Ok(())
    }

    /// Returns an error if the movement of bishop is blocked by another piece
    fn bishop_validation_helper(
        &self,
        source: (isize, isize),
        destination: (isize, isize),
        start: isize,
        end: isize,
        direction: &str,
    ) -> Result<(), Error> {
        for index in start..end {
            let piece = match direction {
                "top-left" => self.get_piece(source.0 - index, source.1 - index),
                "top-right" => self.get_piece(source.0 - index, source.1 + index),
                "down-left" => self.get_piece(source.0 + index, source.1 - index),
                "down-right" => self.get_piece(source.0 + index, source.1 + index),
                _ => return Err(Error::InvalidMove(format!("Invalid move"))),
            };
            if piece.is_some() {
                return Err(Error::InvalidMove(format!(
                    "Can't move to {:?}",
                    destination
                )));
            }
        }
        Ok(())
    }

    /// Moves the queen if possible otherwise returns Error
    fn move_queen(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        if source.0 == destination.0 || source.1 == destination.1 {
            self.move_rook(color, source, destination)
        } else if source.0 + source.1 == destination.0 + destination.1
            || source.0 - source.1 == destination.0 - destination.1
        {
            self.move_bishop(color, source, destination)
        } else {
            Err(Error::InvalidMove(format!("Invalid queen move")))
        }
    }

    /// Moves the King if legal otherwise returns an error
    fn move_king(
        &mut self,
        color: Color,
        source: (isize, isize),
        destination: (isize, isize),
    ) -> Result<(), Error> {
        for move_ in &LEGAL_KING_MOVES {
            if source.0 + move_.0 == destination.0 && source.1 + move_.1 == destination.1 {
                self.remove_castling_rights(&color);
                self._move_piece(source, destination);
                self.chessboard.set_king_position(color, destination);
                return Ok(());
            }
        }

        // Castling logic
        if source.1 - destination.1 == -2 && source.0 == destination.0 {
            // castling
            if color == Color::White {
                if self.castling_rights[0][1]
                    && self.get_piece(source.0, source.1 + 1).is_none()
                    && self.get_piece(source.0, source.1 + 2).is_none()
                {
                    self.king_castling_helper(
                        source,
                        destination,
                        (0, 1),
                        color,
                        (source.0, source.1 + 3),
                        (source.0, source.1 + 1),
                    )?;
                }
            } else {
                if self.castling_rights[1][1]
                    && self.get_piece(source.0, source.1 + 1).is_none()
                    && self.get_piece(source.0, source.1 + 2).is_none()
                {
                    self.king_castling_helper(
                        source,
                        destination,
                        (1, 1),
                        color,
                        (source.0, source.1 + 3),
                        (source.0, source.1 + 1),
                    )?;
                }
            }
        } else if source.1 - destination.1 == 3 && source.0 == destination.0 {
            if color == Color::White {
                if self.castling_rights[0][0]
                    && self.get_piece(source.0, source.1 - 1).is_none()
                    && self.get_piece(source.0, source.1 - 2).is_none()
                    && self.get_piece(source.0, source.1 - 3).is_none()
                {
                    self.king_castling_helper(
                        source,
                        destination,
                        (0, 0),
                        color,
                        (source.0, source.1 - 4),
                        (source.0, source.1 - 2),
                    )?;
                }
            } else {
                if self.castling_rights[1][0]
                    && self.get_piece(source.0, source.1 - 1).is_none()
                    && self.get_piece(source.0, source.1 - 2).is_none()
                    && self.get_piece(source.0, source.1 - 3).is_none()
                {
                    self.king_castling_helper(
                        source,
                        destination,
                        (1, 0),
                        color,
                        (source.0, source.1 - 4),
                        (source.0, source.1 - 2),
                    )?;
                }
            }
        }
        Err(Error::InvalidMove(format!("Invalid King move")))
    }

    /// Castles the king if possible otherwise returns an error
    fn king_castling_helper(
        &mut self,
        source: (isize, isize),
        destination: (isize, isize),
        castling_rights: (usize, usize),
        color: Color,
        rook_source: (isize, isize),
        rook_destination: (isize, isize),
    ) -> Result<(), Error> {
        self._move_piece(source, destination);
        self._move_piece(rook_source, rook_destination);
        self.castling_rights[castling_rights.0][castling_rights.1] = false;
        self.chessboard.set_king_position(color, destination);
        Ok(())
    }

    /// Removes castling rights for side color
    /// Should be called only when a King is moved
    fn remove_castling_rights(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.castling_rights[0] = [false, false];
            }
            Color::Black => {
                self.castling_rights[1] = [false, false];
            }
        }
    }

    /// Moves a piece from source to destination
    /// Bounds checking should be done by the caller
    fn _move_piece(&mut self, source: (isize, isize), destination: (isize, isize)) {
        let piece = self.get_piece(source.0, source.1).unwrap();
        let mut squares = &mut self.chessboard.squares;
        squares[destination.0 as usize][destination.1 as usize].place_piece(piece.clone());
        squares[source.0 as usize][source.1 as usize].remove_piece();
    }

    /// Returns true if the King is under check
    fn is_under_check(&mut self, color: Color) -> bool {
        let king_position = self.chessboard.get_king_position(color);
        let other_color = color.clone().other();
        self.is_under_check_by_rook_queen(king_position, other_color)
            || self.is_under_check_by_bishop_queen(king_position, other_color)
            || self.is_under_check_by_knight(king_position, other_color)
            || self.is_under_check_by_pawn(king_position, other_color)
    }

    /// Returns true if the King at king_position is under check by a rook or queen of opposite color
    fn is_under_check_by_rook_queen(&self, king_position: (isize, isize), color: Color) -> bool {
        let mut row = king_position.0;
        let mut file = king_position.1;
        // check right
        while file < 7 {
            file += 1;
            match self.is_under_attack_by_rqb(color, row, file, "rq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check left
        file = king_position.1;
        while file > 0 {
            file -= 1;
            match self.is_under_attack_by_rqb(color, row, file, "rq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check up
        file = king_position.1;
        while row < 7 {
            row += 1;
            match self.is_under_attack_by_rqb(color, row, file, "rq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check down
        row = king_position.0;
        while row > 0 {
            row -= 1;
            match self.is_under_attack_by_rqb(color, row, file, "rq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        false
    }

    /// Returns true if piece at (row, file) is one of opponent's "pieces"
    fn is_under_attack_by_rqb(
        &self,
        color: Color,
        mut row: isize,
        mut file: isize,
        pieces: &str,
    ) -> Result<bool, Error> {
        let piece = self.get_piece(row, file);
        if piece.is_some() {
            return if pieces == "rq"
                && (piece.unwrap() == Piece::Rook(color) || piece.unwrap() == Piece::Queen(color))
                || pieces == "bq"
                    && (piece.unwrap() == Piece::Bishop(color)
                        || piece.unwrap() == Piece::Queen(color))
            {
                Ok(true)
            } else {
                Err(Error::Dummy) // this is necessary because of the loop in caller
            };
        }
        Ok(false)
    }

    /// Returns true if king is exposed to opponents bishop or queen along the diagonals
    fn is_under_check_by_bishop_queen(&self, king_position: (isize, isize), color: Color) -> bool {
        let mut row = king_position.0;
        let mut file = king_position.1;
        // check right up
        while row < 7 && file < 7 {
            row += 1;
            file += 1;
            match self.is_under_attack_by_rqb(color, row, file, "bq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check left up
        row = king_position.0;
        file = king_position.1;
        while row < 7 && file > 0 {
            row += 1;
            file -= 1;
            match self.is_under_attack_by_rqb(color, row, file, "bq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check right down
        row = king_position.0;
        file = king_position.1;
        while row > 0 && file < 7 {
            row -= 1;
            file += 1;
            match self.is_under_attack_by_rqb(color, row, file, "bq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        // check left down
        row = king_position.0;
        file = king_position.1;
        while row > 0 && file > 0 {
            row -= 1;
            file -= 1;
            match self.is_under_attack_by_rqb(color, row, file, "bq") {
                Ok(true) => return true,
                Err(_) => break,
                _ => continue,
            }
        }
        false
    }

    /// Returns true if the king_position is under check by any Knight
    /// color: opponent color
    fn is_under_check_by_knight(&self, king_position: (isize, isize), color: Color) -> bool {
        for move_ in LEGAL_KNIGHT_MOVES {
            let possible_enemy_knight_position =
                (king_position.0 + move_.0, king_position.1 + move_.1);
            let piece = self.get_piece(
                possible_enemy_knight_position.0,
                possible_enemy_knight_position.1,
            );
            match piece {
                Some(Piece::Knight(piece_color)) => {
                    if *piece_color == color {
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
    }

    /// Returns true if the king_position is under check by opponent pawn
    fn is_under_check_by_pawn(&self, king_position: (isize, isize), color: Color) -> bool {
        let mut row = king_position.0;
        let mut file = king_position.1;
        // check right
        row = if color == Color::White {
            row + 1
        } else {
            row - 1
        };
        if row < 0 || row > 7 {
            return false;
        }
        file += 1;
        let piece = if file < 8 {
            self.get_piece(row, file)
        } else {
            &None
        };
        let under_attack_from_right = self.under_attack_from_pawn(color, piece);
        file -= 1;
        let piece = if file >= 0 {
            self.get_piece(row, file)
        } else {
            &None
        };
        let under_attack_from_left = self.under_attack_from_pawn(color, piece);
        under_attack_from_left || under_attack_from_right
    }

    /// Helper method for is_under_check_by_pawn
    fn under_attack_from_pawn(&self, color: Color, piece: &Option<Piece>) -> bool {
        if piece.is_some() && *piece.unwrap().get_color() == color {
            if piece.unwrap() == Piece::Pawn(color) {
                return true;
            }
        }
        false
    }

    /// Returns true the game is in checkmate
    fn is_under_checkmate(&mut self, color: Color) -> bool {
        if !self.is_under_check(color) {
            return false;
        }
        let mut king_position = self.chessboard.get_king_position(color);
        let current_state = self.chessboard.clone();
        let current_castling_rights = self.castling_rights.clone();

        for move_ in &LEGAL_KING_MOVES {
            let destination = (king_position.0 + move_.0, king_position.1 + move_.1);
            match self.make_a_move(king_position, destination) {
                Ok(()) => {
                    self.chessboard = current_state;
                    self.castling_rights = current_castling_rights;
                    return false;
                }
                _ => {}
            }
            for row in 0..ROWS {
                for col in 0..COLS {
                    let piece = self.chessboard.get_piece(row, col);
                    if piece.is_some() && *piece.unwrap().get_color() == color {
                        let piece = piece.unwrap();
                        let result = match piece {
                            Piece::Pawn(color) => self.checkmate_helper_pawn(color, row, col),
                            Piece::Rook(color) => self.checkmate_helper_rook(color, row, col),
                            Piece::Knight(color) => self.checkmate_helper_knight(color, row, col),
                            Piece::Bishop(color) => self.checkmate_helper_bishop(color, row, col),
                            Piece::Queen(color) => self.checkmate_helper_queen(color, row, col),
                            Piece::King(color) => true,
                        };

                        if !result {
                            self.chessboard = current_state;
                            self.castling_rights = current_castling_rights;
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn checkmate_helper_pawn(&mut self, color: Color, row: isize, col: isize) -> bool {
        // create an array of possible
        let multiplier = if color == Color::White { -1 } else { 1 };

        for move_ in &LEGAL_PAWN_MOVES {
            let destination = (row + move_.0 * multiplier, col + move_.1);
            match self.make_a_move((row, col), destination) {
                Ok(()) => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn checkmate_helper_rook(&mut self, color: Color, row: isize, col: isize) -> bool {
        for i in 0..8 {
            let destinations = [(i, col), (row, i)];
            for destination in destinations {
                match self.make_a_move((row, col), destination) {
                    Ok(()) => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn checkmate_helper_knight(&mut self, color: Color, row: isize, col: isize) -> bool {
        for move_ in &LEGAL_KNIGHT_MOVES {
            let destination = (row + move_.0, col + move_.1);
            match self.make_a_move((row, col), destination) {
                Ok(()) => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn checkmate_helper_bishop(&mut self, color: Color, row: isize, col: isize) -> bool {
        for index in 0..8 {
            let destinations = vec![
                (row - index, col - index),
                (row - index, col + index),
                (row + index, col - index),
                (row + index, col + index),
            ];
            for destination in destinations {
                match self.make_a_move((row, col), destination) {
                    Ok(()) => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        true
    }

    /// Returns true if (row, col) is under checkmate for color by queen
    fn checkmate_helper_queen(&mut self, color: Color, row: isize, col: isize) -> bool {
        self.checkmate_helper_bishop(color, row, col) || self.checkmate_helper_rook(color, row, col)
    }

    fn is_under_stalemate(&mut self, color: Color) -> bool {
        let king_position = self.chessboard.get_king_position(color);
        for move_ in LEGAL_KING_MOVES {
            let destination = (king_position.0 + move_.0, king_position.1 + move_.1);
            match self.is_under_check(color) {
                false => return false,
                true => (),
            }
        }
        !self.is_under_check(color)
    }

    /// Returns the piece at (row, col) else None
    fn get_piece(&self, row: isize, file: isize) -> &Option<Piece> {
        if row < ROWS && row >= 0 && file < COLS && file >= 0 {
            return self.chessboard.get_piece(row, file);
        }
        &None
    }

    /// Prompts the user for source and destination
    /// Extracts the row and column from the input and returns a tuple
    fn get_move() -> ((isize, isize), (isize, isize)) {
        let mut source = String::new();
        let mut destination = String::new();
        stdout().flush().unwrap();
        let source = Self::get_position("Source", source);
        let destination = Self::get_position("Destination", destination);
        (source, destination)
    }

    /// Prompts for input until input is valid
    fn get_position(str: &str, mut input: String) -> (isize, isize) {
        let mut position = (-1, -1);
        while position == (-1, -1) {
            println!("Enter {}:", str);
            stdin()
                .read_line(&mut input)
                .expect("Oops! Something went wrong. Please restart.");
            match Self::extract_position(&input) {
                (row, file) => {
                    if row < ROWS && file < COLS && row >= 0 && file >= 0 {
                        position = (row, file);
                    }
                }
            }
        }
        position
    }

    /// Returns the 0-indexed (row, col) extracted from the string
    fn extract_position(str: &str) -> (isize, isize) {
        let mut chars = str.chars();
        let file = chars.next().unwrap() as usize - 97;
        let row = 8 - (chars.next().unwrap() as usize - 48);
        (row as isize, file as isize)
    }
}
