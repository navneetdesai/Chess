use crate::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Square { piece }
    }
    
    pub fn get_piece(&self) -> &Option<Piece> {
        &self.piece
    }

    pub fn place_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn remove_piece(&mut self) {
        self.piece = None;
    }
}
