use piston_window::MouseButton;
use crate::data::planet::tile::Tile;

pub mod planet;

#[derive(PartialEq)]
pub enum Status {
    Start,
    Continue,
    GameClear,
    GameOver,
}

pub struct Data {
    pub status: Status,
    pub history: Vec<planet::Planet>,
    pub text: String,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct MouseButtonState {
    pub cursor: Position,
    pub pressed: Option<MouseButton>,
    pub released: Option<MouseButton>,
}

pub enum Drawable {
    TILE(Tile),
}