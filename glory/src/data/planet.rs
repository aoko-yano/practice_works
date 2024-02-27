pub mod tile;

use std::clone::Clone;
use std::vec::Vec;

use tile::Tile;

#[derive(Clone, Debug)]
pub struct Planet {
    pub tiles: Vec<Vec<Tile>>,
}

impl Planet {
    pub fn is_still_survives(&self) -> bool {
        for line in &self.tiles {
            for tile in line {
                if tile.is_still_survive() {
                    return true
                }
            }
        }
        false
    }

    pub fn is_finished(&self) -> bool {
        for line in &self.tiles {
            for tile in line {
                if tile.is_finished() {
                    return true
                }
            }
        }
        false
    }
}