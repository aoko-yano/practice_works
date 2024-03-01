use crate::data::planet::tile::environment::nature::AreaType;
use rand::prelude::*;

pub fn generate_initial_area_types(x: usize, y: usize, sea_ratio: f64) -> Vec<Vec<AreaType>> {
    let mut area_types = Vec::<Vec<AreaType>>::new();
    for i in 0..y {
        let mut line = Vec::<AreaType>::new();
        for j in 0..x {
            line.push(select_sea_or_desert(sea_ratio));
        }
        area_types.push(line);
    }
    area_types
}

fn select_sea_or_desert(sea_ratio: f64) -> AreaType {
    let mut rng = rand::thread_rng();
    let val: f64 = rng.gen();
    if val < sea_ratio {
        return AreaType::Sea
    }
    AreaType::Desert
}