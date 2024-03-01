use std::collections::HashMap;
use piston_window::*;
use crate::data::Data;
use crate::data::planet::tile::Tile;
use crate::draw::ImageKey::GreenTile;

pub struct Image {
    texture: G2dTexture,
    size: Size,
    scale: f64,
}

#[derive(Eq, Hash, PartialEq)]
pub enum ImageKey {
    GreenTile,
}

pub fn load_image(window: &mut PistonWindow) -> HashMap<ImageKey, Image> {
    let mut map = HashMap::<ImageKey, Image>::new();
    map.insert(
        GreenTile,
        Image {
            texture: Texture::from_path(
                &mut window.create_texture_context(),
                "resource/test.png",
                Flip::None,
                &TextureSettings::new())
                .unwrap(),
            size: Size {
                width: 32.0,
                height: 32.0},
            scale: 1.0
        },
    );
    map
}

pub fn draw(images: &HashMap<ImageKey, Image>, data: &Data, c: Context, g: &mut G2d) {
    // TODO: implementation.
    let planet = data.history.last().unwrap();
    for (y, line) in planet.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            draw_tile(images, x, y, tile, c, g);
        }
    }
}

fn draw_tile(images: &HashMap<ImageKey, Image>, x: usize, y: usize, _tile: &Tile, c: Context, g: &mut G2d) {
    let tile_image = images.get(&GreenTile).unwrap();
    let transform = c
        .transform
        .trans(
            80.0 + x as f64 * tile_image.size.width + (tile_image.size.width / 2.0) * (if y % 2 == 0 {0.0} else {1.0}),
            40.0 + y as f64 * (tile_image.size.height - 14.0),
        );
    image(&tile_image.texture, transform, g);
}