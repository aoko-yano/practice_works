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