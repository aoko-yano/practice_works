pub mod self_organize;
pub mod set_climate;

use crate::data::planet::tile::environment::nature::AreaType;
use rand::prelude::*;
use crate::calc::simulator::self_organize::self_organize;
use crate::calc::simulator::set_climate::set_climate;

pub fn generate_initial_area_types(x: usize, y: usize, sea_ratio: f64) -> Vec<Vec<AreaType>> {
    let mut area_types = Vec::<Vec<AreaType>>::new();
    for _i in 0..y {
        let mut line = Vec::<AreaType>::new();
        for _j in 0..x {
            line.push(select_sea_or_desert(sea_ratio));
        }
        area_types.push(line);
    }
    self_organize(&mut area_types);
    set_climate(&mut area_types);
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