use piston_window::Position;
use crate::data::planet::tile::environment::nature::AreaType;
use rand::prelude::*;
use crate::calc::next_tiles;

pub fn generate_initial_area_types(x: usize, y: usize, sea_ratio: f64) -> Vec<Vec<AreaType>> {
    let mut area_types = Vec::<Vec<AreaType>>::new();
    for _i in 0..y {
        let mut line = Vec::<AreaType>::new();
        for _j in 0..x {
            line.push(select_sea_or_desert(sea_ratio));
        }
        area_types.push(line);
    }
    self_organize(sea_ratio, &mut area_types);
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

fn self_organize(sea_ratio: f64, area_types: &mut Vec<Vec<AreaType>>) {
    for _i in 0..100 {
        let mut temp_area_types = update_area_types_for_self_organize(&area_types);
        std::mem::swap(&mut temp_area_types, area_types);
    }
}

fn update_area_types_for_self_organize(area_types: &Vec<Vec<AreaType>>) -> Vec<Vec<AreaType>> {
    let mut temp_area_types = Vec::<Vec<AreaType>>::new();
    let y = area_types.len();
    for i in 0..y {
        let line = area_types.get(i).unwrap();
        let x = line.len();
        let mut temp_line = Vec::<AreaType>::new();
        for j in 0..x {
            let pos = Position{x : j as i32, y: i as i32 };
            temp_line.push(
                update_area_type_for_self_organize(area_types, x, y, pos));
        }
        temp_area_types.push(temp_line);
    }
    temp_area_types
}

fn update_area_type_for_self_organize(
    area_types: &Vec<Vec<AreaType>>,
    x: usize, y: usize,
    pos: Position) -> AreaType {
    let next_tile_positions = next_tiles::get_next_tile_positions(x, y, pos);
    let mut sea_count = 0;
    let mut desert_count = 0;
    match next_tile_positions.up_right {
        Some(up_right) => {
            match get_area_type(area_types, up_right.x, up_right.y) {
                AreaType::Sea => sea_count += 1,
                AreaType::Desert => desert_count += 1,
                _ => {}
            }
        },
        _ => {}
    }
    match next_tile_positions.up_left {
        Some(up_left) => {
            match get_area_type(area_types, up_left.x, up_left.y) {
                AreaType::Sea => sea_count += 1,
                AreaType::Desert => desert_count += 1,
                _ => {}
            }
        },
        _ => {}
    }
    match get_area_type(area_types, next_tile_positions.right.x, next_tile_positions.right.y) {
        AreaType::Sea => sea_count += 1,
        AreaType::Desert => desert_count += 1,
        _ => {}
    }
    match get_area_type(area_types, next_tile_positions.left.x, next_tile_positions.left.y) {
        AreaType::Sea => sea_count += 1,
        AreaType::Desert => desert_count += 1,
        _ => {}
    }
    match next_tile_positions.down_right {
        Some(down_right) => {
            match get_area_type(area_types, down_right.x, down_right.y) {
                AreaType::Sea => sea_count += 1,
                AreaType::Desert => desert_count += 1,
                _ => {}
            }
        },
        _ => {}
    }
    match next_tile_positions.down_left {
        Some(down_left) => {
            match get_area_type(area_types, down_left.x, down_left.y) {
                AreaType::Sea => sea_count += 1,
                AreaType::Desert => desert_count += 1,
                _ => {}
            }
        },
        _ => {}
    }
    determine_area_type(sea_count, desert_count)
}

fn get_area_type(area_types: &Vec<Vec<AreaType>>, x: i32, y: i32) -> &AreaType {
    area_types.get(y as usize).unwrap().get(x as usize).unwrap()
}

fn determine_area_type(sea_count: i32, desert_count: i32) -> AreaType {
    let sea_score = sea_count as f64;
    let desert_score = desert_count as f64;
    let mut rng = rand::thread_rng();
    let val: f64 = rng.gen();
    if val < sea_score / (sea_score + desert_score) {
        return AreaType::Sea
    }
    AreaType::Desert
}