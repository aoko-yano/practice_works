use crate::calc::ahead::get_some_hexes_ahead;
use crate::data::planet::tile::environment::nature::AreaType;
use crate::data::Position;

pub fn set_climate(area_types: &mut Vec<Vec<AreaType>>) {
    let mut temp_area_types = Vec::<Vec<AreaType>>::new();
    let y = area_types.len();
    for i in 0..y {
        let line = area_types.get(i).unwrap();
        let mut temp_line = Vec::<AreaType>::new();
        let x = line.len();
        for j in 0.. x {
            let tile = line.get(j).unwrap();
            let pos = Position { x: j as i32, y: i as i32 };
            temp_line.push(select_area_type(&area_types, x, y, &pos, tile));
        }
        temp_area_types.push(temp_line);
    }
    std::mem::swap(&mut temp_area_types, area_types);
}

fn select_area_type(area_types: &Vec<Vec<AreaType>>, x: usize, y: usize, pos: &Position, tile: &AreaType) -> AreaType {
    if *tile == AreaType::Sea {
        return AreaType::Sea
    }
    match get_distance_from_sea(area_types, x, y, pos) {
        Ok(distance) => {
            let equator_y = y as f64 / 2.0;
            let fixed_latitude = (equator_y - pos.y as f64).abs() / equator_y;
            let default_temperature = 30.0 - fixed_latitude * 30.0;
            if default_temperature < 7.0 {
                if default_temperature < 5.0 {
                    // EF: frozen
                    return AreaType::Snow
                }
                // ET: tundora
                return AreaType::Green
            }
            let fixed_distance = distance as f64 / equator_y;
            let distance_score = 1.0 + (1.0 + fixed_distance).log(1.2);
            let humidity = (fixed_latitude - 0.3).abs() * (1.1 - fixed_latitude) / distance_score;
            if humidity < 0.075 {
                if distance_score < 1.5 {
                    // BS: step
                    return AreaType::Green
                }
                // BW: desert
                return AreaType::Desert
            }
            // A&C&D
            return AreaType::Forest
        },
        Err(_e) => {
            return AreaType::Desert
        }
    }
}

fn get_distance_from_sea(area_types: &Vec<Vec<AreaType>>, x: usize, y:usize, pos: &Position) -> anyhow::Result<usize> {
    let max_distance = std::cmp::max(x, y) / 2 + 1;
    for dist in 1..max_distance {
        match get_some_hexes_ahead(x, y, &pos, dist) {
            Ok(on_boundary) => {
                for p in on_boundary {
                    if *area_types.get(p.y as usize).unwrap().get(p.x as usize).unwrap() == AreaType::Sea {
                        return Ok(dist)
                    }
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Err(anyhow::anyhow!("The sea does not exists on this planet"))
}