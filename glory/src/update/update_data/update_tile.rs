pub mod update_technology;
pub mod update_culture;
pub mod update_biological_resource;
pub mod update_natural_resource;
pub mod update_nature;

use crate::data::Position;
use crate::data::planet::tile::environment::Environment;
use crate::data::planet::tile::society::Society;
use crate::data::planet::tile::Tile;

pub fn update_tile(tile: &mut Tile, past_tiles: &Vec<Vec<Tile>>) {
    update_society(
        &mut tile.society,
        &tile.position,
        past_tiles);
    update_environment(
        &mut tile.environment,
        &tile.position,
        past_tiles);
}

fn update_society(society: &mut Society, position: &Position, past_tiles: &Vec<Vec<Tile>>) {
    update_culture::update_culture(
        &mut society.cultures,
        &position, past_tiles);
    update_technology::update_technology(
        &mut society.technologies,
        &position, past_tiles);
}

fn update_environment(
    environment: &mut Environment,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    update_biological_resource::update_biological_resource(
        &mut environment.biological_resource,
        &position,
        past_tiles);
    update_natural_resource::update_natural_resource(
        &mut environment.natural_resources,
        &position,
        past_tiles);
    update_nature::update_nature(
        &mut environment.nature,
        &position,
        past_tiles);
}