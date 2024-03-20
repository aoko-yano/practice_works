use std::collections::HashMap;
use crate::calc::tile_position::get_tile_position;
use crate::data::{Data, Drawable, MouseButtonState};
use crate::data::planet::tile::environment::nature::AreaType;
use crate::draw::Image;

pub fn mouse_click_event(
    data: &mut Data,
    mouse_button_state: &MouseButtonState,
    images: &HashMap<AreaType, Image>,
    drawn_items: &Vec<Drawable>) {
    data.text = format!("Clicked:{:?},{:?}", mouse_button_state.cursor, mouse_button_state.pressed);
    let clicked_item = get_clicked_item(mouse_button_state, images, drawn_items);
    match clicked_item {
        Some(item) => {
            match item {
                Drawable::TILE(tile) => {
                    data.text += &*format!("Tile:{:?}", tile.position);
                }
            }
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

fn get_clicked_item(
    mouse_button_state: &MouseButtonState,
    images: &HashMap<AreaType, Image>,
    drawn_items: &Vec<Drawable>) -> Option<Drawable> {
    let cursor = &mouse_button_state.cursor;
    for item in drawn_items.iter().rev() {
        match item {
            Drawable::TILE(tile) => {
                let tile_image = images.get(&AreaType::Desert).unwrap();
                let tile_image_width = tile_image.size.width * tile_image.scale;
                let tile_image_height = tile_image.size.height * tile_image.scale;
                let pos = get_tile_position(
                    tile.position.x,
                    tile.position.y,
                    tile_image_width,
                    tile_image_height);
                let relative_pos_x = ((cursor.x - pos.0 as i32) as f64 / tile_image.scale) as i32;
                let relative_pos_y = ((cursor.y - pos.1 as i32) as f64 / tile_image.scale) as i32;
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
                    return Some(Drawable::TILE(tile.clone()))
                }
            }
        }
    }
    None
}