use piston_window::Position;
use crate::data::{Data, Status};
use crate::data::planet::Planet;
use crate::piston::KeysState;

use put_people_to_data::put_people_to_data;
use update_tile::update_tile;

pub mod put_people_to_data;
pub mod update_tile;

pub fn update_data(data: &mut Data, _key: &KeysState) {
    update_history(data);
    update_status(data);
}

fn update_history(data: &mut Data) {
    match data.status {
        Status::Start => {
            put_people_to_data(data,&Position{ x: 0, y: 0 }, 1);
        },
        Status::Continue => {
            data.history.push(create_planet_from_past_state(data.history.last().unwrap()));
        },
        _ => {}
    }
}

fn create_planet_from_past_state(past: &Planet) -> Planet {
    let mut present = past.clone();
    for (_y, line) in present.tiles.iter_mut().enumerate() {
        for (_x, tile) in line.iter_mut().enumerate() {
            update_tile(tile, &past.tiles);
        }
    }
    present
}

fn update_status(data: &mut Data) {
    match data.status {
        Status::Start => {
            data.status = Status::Continue
        }
        Status::Continue => {
            if data.history.last().unwrap().is_still_survives() {
                data.status = Status::GameOver
            }
            if data.history.last().unwrap().is_finished() {
                data.status = Status::GameClear
            }
        },
        _ => {}
    }
}