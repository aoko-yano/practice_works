pub mod society;
pub mod environment;

use crate::data::Position;
use society::technology::Technology;

#[derive(Clone, Debug)]
pub struct Tile {
    pub society: society::Society,
    pub environment: environment::Environment,
    pub position: Position,
}

impl Tile {
    pub fn is_still_survive(&self) -> bool {
        self.society.population.number > 0
    }

    pub fn is_finished(&self) -> bool {
        self.society.technologies.established_technology.contains_key(&Technology::Developed)
    }
}