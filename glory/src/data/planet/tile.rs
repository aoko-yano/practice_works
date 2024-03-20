pub mod society;
pub mod environment;

use std::collections::HashMap;
use piston_window::{Context, G2d, image, Transformed};

use society::technology::Technology;

use crate::data::{Clickable, Position};
use crate::calc::tile_position::get_tile_position;
use crate::data::planet::tile::environment::nature::AreaType;
use crate::draw::Image;

#[derive(Clone, Debug)]
pub struct Tile {
    pub society: society::Society,
    pub environment: environment::Environment,
    pub position: Position,
}

impl Tile {
    pub fn is_still_survive(&self) -> bool {
        self.society.population.number > 0
    }

    pub fn is_finished(&self) -> bool {
        self.society.technologies.established_technology.contains_key(&Technology::Developed)
    }

    pub fn draw(
        &self,
        images: &HashMap<AreaType, Image>,
        c: Context,
        g: &mut G2d,
        drawn_clickable_items: &mut Vec<Clickable>) {
        let tile_image = images.get(&self.environment.nature.area_type).unwrap();
        let image_width = tile_image.size.width * tile_image.scale;
        let image_height = tile_image.size.height * tile_image.scale;
        let tile_image_position = get_tile_position(
            self.position.x,
            self.position.y,
            image_width,
            image_height);
        let transform = c
            .transform
            .trans(
                tile_image_position.0,
                tile_image_position.1,
            )
            .scale(tile_image.scale, tile_image.scale);
        image(&tile_image.texture, transform, g);
        drawn_clickable_items.push(Clickable::TILE(self.clone()));
    }
}