use std::collections::HashMap;
use crate::calc::tile_position::get_tile_position;
use crate::data::{Data, MouseButtonState, Position};
use crate::data::planet::tile::environment::nature::AreaType;
use crate::draw::Image;

pub fn mouse_click_event(
    data: &mut Data,
    mouse_button_state: &MouseButtonState,
    images: &HashMap<AreaType, Image>) {
    let tile_image = images.get(&AreaType::Desert).unwrap();
    let clicked_pos = get_clicked_tile(data, mouse_button_state, &tile_image);
    data.text = format!("kick mouse click event for: {:?}", mouse_button_state);
    match clicked_pos {
        Some(p) => {
            data.text += &*format!("{:?}", p);
        },
        None => {}
    }
}

const O: bool = true;
const X: bool = false;

const TILE_FORM: [[bool; 32]; 32] = [
    [X,X,X,X,X,X,X,X,X,X,X,X,X,X,O,O,O,O,X,X,X,X,X,X,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X],
    [X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X],
    [X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O],
    [X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X],
    [X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X],
    [X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,X,X,X,X,O,O,O,O,O,O,O,O,X,X,X,X,X,X,X,X,X,X,X,X],
    [X,X,X,X,X,X,X,X,X,X,X,X,X,X,O,O,O,O,X,X,X,X,X,X,X,X,X,X,X,X,X,X],
];

fn get_clicked_tile(
    data: &Data,
    mouse_button_state: &MouseButtonState,
    tile_image: &Image) -> Option<Position> {
    let cursor = &mouse_button_state.cursor;
    let tile_image_width = tile_image.size.width * tile_image.scale;
    let tile_image_height = tile_image.size.height * tile_image.scale;
    let tiles = &data.history.last().unwrap().tiles;
    for y in (0..tiles.len()).rev() {
        let line = tiles.get(y).unwrap();
        for x in 0..line.len() {
            let pos = get_tile_position(x, y, tile_image_width, tile_image_height);
            let relative_pos_x = cursor.x - pos.0 as i32;
            let relative_pos_y = cursor.y - pos.1 as i32;
            if relative_pos_x < 0 {
                continue;
            }
            if relative_pos_x >= tile_image_width as i32 {
                continue;
            }
            if relative_pos_y < 0 {
                continue;
            }
            if relative_pos_y >= tile_image_height as i32 {
                continue;
            }
            if TILE_FORM[relative_pos_y as usize][relative_pos_x as usize] {
                return Some(Position{ x: x as i32, y: y as i32 })
            }
        }
    }
    None
}