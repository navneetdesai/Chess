use crate::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Square { piece }
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    pub fn has_piece(&self) -> bool {
        self.piece.is_some()
    }

    /// Returns the piece on the square
    /// Will panic if the square is empty
    /// Should be called after `has_piece`
    pub fn get_piece(&self) -> &Piece {
        self.piece.as_ref().unwrap()
    }

    pub fn place_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn remove_piece(&mut self) {
        self.piece = None;
    }
}
