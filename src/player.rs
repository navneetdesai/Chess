use crate::piece::{Color};

pub struct Player {
    name: String,
    color: Color,
}

impl Player {
    pub fn new(name: String, color: Color) -> Self {
        Player { name, color }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}
