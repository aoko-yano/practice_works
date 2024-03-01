use std::collections::HashMap;
use piston_window::Position;
use crate::data::{Data, Status};
use crate::data::planet::Planet;
use crate::data::planet::tile::environment::biological_resource::BiologicalResource;
use crate::data::planet::tile::environment::Environment;
use crate::data::planet::tile::environment::natural_resource::NaturalResources;
use crate::data::planet::tile::environment::nature::{AreaType, Nature};
use crate::data::planet::tile::society::{Population, Society};
use crate::data::planet::tile::society::culture::Cultures;
use crate::data::planet::tile::society::technology::Technologies;
use crate::data::planet::tile::Tile;
use crate::simulator::generate_initial_area_types;

pub fn create_data(x: usize, y: usize, sea_ratio: f64) -> Data {
    Data { status: Status::Start, history: vec!{ create_planet(x, y, sea_ratio) } }
}

fn create_planet(x: usize, y: usize, sea_ratio: f64) -> Planet {
    let mut tiles = Vec::<Vec<Tile>>::new();
    let initial_area_type = generate_initial_area_types(x, y, sea_ratio);
    for i in 0..y {
        let mut line = Vec::<Tile>::new();
        for j in 0..x {
            let area_type = initial_area_type.get(i).unwrap().get(j).unwrap();
            line.push(create_tile(j, i, area_type));
        }
        tiles.push(line);
    }
    Planet { tiles }
}

fn create_tile(x: usize, y: usize, area_type: &AreaType) -> Tile {
    let pos_x = x as i32;
    let pos_y = y as i32;
    Tile {
        society: create_empty_society(),
        environment: create_empty_environment(area_type),
        position: Position{ x: pos_x, y: pos_y } }
}

fn create_empty_society() -> Society {
    Society {
        population: Population { number: 0 },
        cultures: Cultures { established_culture: Default::default() },
        technologies: Technologies { established_technology: HashMap::new() }
    }
}

fn create_empty_environment(area_type: &AreaType) -> Environment {
    Environment {
        biological_resource: BiologicalResource { living_species: Default::default() },
        natural_resources: NaturalResources { existing_natural_resource: Default::default() },
        nature: Nature { area_type: area_type.clone() },
    }
}