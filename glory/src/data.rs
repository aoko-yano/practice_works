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
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
