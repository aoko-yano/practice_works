use std::collections::HashMap;
use piston_window::Position;
use crate::data::{Data, Status};
use crate::data::planet::Planet;
use crate::data::planet::tile::environment::biological_resource::BiologicalResource;
use crate::data::planet::tile::environment::Environment;
use crate::data::planet::tile::environment::natural_resource::NaturalResources;
use crate::data::planet::tile::environment::nature::Nature;
use crate::data::planet::tile::society::{Population, Society};
use crate::data::planet::tile::society::culture::Cultures;
use crate::data::planet::tile::society::technology::Technologies;
use crate::data::planet::tile::Tile;

pub fn create_data(x: usize, y: usize) -> Data {
    Data { status: Status::Start, history: vec!{ create_planet(x, y) } }
}

fn create_planet(x: usize, y: usize) -> Planet {
    let mut tiles = Vec::<Vec<Tile>>::new();
    for i in 0..y {
        let mut line = Vec::<Tile>::new();
        for j in 0..x {
            line.push(create_tile(i, j));
        }
        tiles.push(line);
    }
    Planet { tiles }
}

fn create_tile(x: usize, y: usize) -> Tile {
    let pos_x = x as i32;
    let pos_y = y as i32;
    Tile {
        society: create_empty_society(),
        environment: create_empty_environment(),
        position: Position{ x: pos_x, y: pos_y } }
}

fn create_empty_society() -> Society {
    Society {
        population: Population { number: 0 },
        cultures: Cultures { established_culture: Default::default() },
        technologies: Technologies { established_technology: HashMap::new() }
    }
}

fn create_empty_environment() -> Environment {
    Environment {
        biological_resource: BiologicalResource { living_species: Default::default() },
        natural_resources: NaturalResources { existing_natural_resource: Default::default() },
        nature: Nature {},
    }
}