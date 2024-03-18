use crate::data::Position;

use crate::data::Data;
use crate::data::planet::Planet;
use crate::data::planet::tile::society::{Population, Society};
use crate::data::planet::tile::Tile;

pub fn put_people_to_data(data: &mut Data, position: &Position, number: usize) {
    let last_index = data.history.len() - 1;
    put_people_to_planet(data.history.get_mut(last_index).unwrap(), position, number);
}

fn put_people_to_planet(planet: &mut Planet, position: &Position, number: usize) {
    let x = position.x as usize;
    let y = position.y as usize;
    put_people_to_tile(planet.tiles.get_mut(y).unwrap().get_mut(x).unwrap(), number);
}

fn put_people_to_tile(tile: &mut Tile, number: usize) {
    put_people_to_society(&mut tile.society, number);
}

fn put_people_to_society(society: &mut Society, number: usize) {
    put_people(&mut society.population, number);
}

fn put_people(population: &mut Population, number: usize) {
    population.number += number;
}
