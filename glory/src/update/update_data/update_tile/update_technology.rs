use crate::data::Position;
use crate::data::planet::tile::society::technology::{MAX_TECH_LEVEL, Technologies, Technology};
use crate::data::planet::tile::Tile;

pub fn update_technology(
    technologies: &mut Technologies,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    attempt_to_clear_all_technologies(technologies, position, past_tiles);
    attempt_to_establish_primitive_technology(technologies, position, past_tiles);
    attempt_to_study_primitive_technology(technologies, position, past_tiles);
    attempt_to_establish_developed_technology(technologies, position, past_tiles);
}

fn attempt_to_clear_all_technologies(
    technologies: &mut Technologies,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    let x = position.x as usize;
    let y = position.y as usize;
    let past_tile = past_tiles.get(y).unwrap().get(x).unwrap();
    let past_population = past_tile.society.population.number;
    if past_population > 0 {
        return
    }
    technologies.established_technology.clear();
}

fn attempt_to_establish_primitive_technology(
    technologies: &mut Technologies,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    let x = position.x as usize;
    let y = position.y as usize;
    let past_tile = past_tiles.get(y).unwrap().get(x).unwrap();
    let past_population = past_tile.society.population.number;
    if past_population == 0 {
        return
    }
    let past_technology = &past_tile.society.technologies.established_technology;
    if past_technology.contains_key(&Technology::Primitive) {
        return
    }
    technologies.established_technology.insert(Technology::Primitive, 0);
}

fn attempt_to_study_primitive_technology(
    technologies: &mut Technologies,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    let x = position.x as usize;
    let y = position.y as usize;
    let past_tile = past_tiles.get(y).unwrap().get(x).unwrap();
    let past_population = past_tile.society.population.number;
    if past_population == 0 {
        return
    }
    let past_technology = &past_tile.society.technologies.established_technology;
    if !past_technology.contains_key(&Technology::Primitive) {
        return
    }
    let primitive_max = MAX_TECH_LEVEL.get(&Technology::Primitive).unwrap();
    if past_technology[&Technology::Primitive] == *primitive_max {
        return
    }
    *technologies.established_technology.entry(Technology::Primitive).or_insert(0) += 1;
    if technologies.established_technology.get(&Technology::Primitive).unwrap() > primitive_max {
        technologies.established_technology.insert(Technology::Primitive, *primitive_max);
    }
}

fn attempt_to_establish_developed_technology(
    technologies: &mut Technologies,
    position: &Position,
    past_tiles: &Vec<Vec<Tile>>) {
    let x = position.x as usize;
    let y = position.y as usize;
    let past_tile = past_tiles.get(y).unwrap().get(x).unwrap();
    let past_population = past_tile.society.population.number;
    if past_population == 0 {
        return
    }
    let past_technology = &past_tile.society.technologies.established_technology;
    if !past_technology.contains_key(&Technology::Primitive) {
        return
    }
    let primitive_max = MAX_TECH_LEVEL.get(&Technology::Primitive).unwrap();
    if past_technology[&Technology::Primitive] < *primitive_max {
        return
    }
    technologies.established_technology.insert(Technology::Developed, 0);
}