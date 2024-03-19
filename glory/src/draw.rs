use std::collections::HashMap;
use piston_window::*;
use piston_window::types::Color;
use crate::data::{Data, Position};
use crate::data::planet::tile::environment::nature::AreaType;
use crate::data::planet::tile::Tile;
use crate::calc::tile_position::get_tile_position;

pub struct Image {
    texture: G2dTexture,
    pub size: Size,
    pub scale: f64,
}

fn load_tile_image(window: &mut PistonWindow, path: &str) -> Image {
    Image {
        texture: Texture::from_path(
            &mut window.create_texture_context(),
            &path,
            Flip::None,
            &TextureSettings::new())
            .unwrap(),
        size: Size {
            width: 32.0,
            height: 32.0},
        scale: 1.0
    }
}

pub fn load_image(window: &mut PistonWindow) -> HashMap<AreaType, Image> {
    let mut map = HashMap::<AreaType, Image>::new();
    map.insert(
        AreaType::Green,
        load_tile_image(window, "resource/green.png"),
    );
    map.insert(
        AreaType::Sea,
        load_tile_image(window, "resource/sea.png"),
    );
    map.insert(
        AreaType::Desert,
        load_tile_image(window, "resource/desert.png"),
    );
    map.insert(
        AreaType::Snow,
        load_tile_image(window, "resource/snow.png"),
    );
    map.insert(
        AreaType::Forest,
        load_tile_image(window, "resource/forest.png"),
    );
    map
}

pub fn load_glyphs(window: &mut PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    window.load_font(font).unwrap()
}

pub fn draw(
    images: &HashMap<AreaType, Image>,
    glyphs: &mut Glyphs,
    data: &Data,
    c: Context,
    g: &mut G2d,
    d: &mut gfx_device_gl::Device) {
    draw_tiles(images, &data, c, g);
    draw_text(&data.text, glyphs, c, g, d);
}

fn draw_tiles(
    images: &HashMap<AreaType, Image>,
    data: &Data,
    c: Context,
    g: &mut G2d) {
    let planet = data.history.last().unwrap();
    for (y, line) in planet.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            draw_tile(images, x, y, tile, c, g);
        }
    }
}

fn draw_tile(
    images: &HashMap<AreaType, Image>,
    x: usize,
    y: usize,
    tile: &Tile,
    c: Context,
    g: &mut G2d) {
    let tile_image = images.get(&tile.environment.nature.area_type).unwrap();
    let image_width = tile_image.size.width * tile_image.scale;
    let image_height = tile_image.size.height * tile_image.scale;
    let tile_position = get_tile_position(x, y, image_width, image_height);
    let transform = c
        .transform
        .trans(
            tile_position.0,
            tile_position.1,
        )
        .scale(tile_image.scale, tile_image.scale);
    image(&tile_image.texture, transform, g);
}

fn draw_text(
    text: &String,
    glyphs: &mut Glyphs,
    c: Context,
    g: &mut G2d,
    d: &mut gfx_device_gl::Device) {
    let text_color: Color = [1.0, 1.0, 1.0, 1.0];
    let pos = Position {x: 20, y: 540};
    Text::new_color(text_color, 20)
        .draw(
            text,
            glyphs,
            &c.draw_state,
            c.transform.trans(pos.x as f64, pos.y as f64),
            g,
        )
        .unwrap();
    glyphs.factory.encoder.flush(d);
}