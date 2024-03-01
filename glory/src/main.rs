pub mod piston;
pub mod data;
pub mod update;
pub mod draw;

use piston_window::*;
use piston::{KeysState, create_window};

use update::create_data::create_data;
use update::update_data::update_data;
use draw::{load_image, draw};

fn main() {
    let mut key_state = KeysState::new();
    let mut window: PistonWindow = create_window();

    let mut data = create_data(24, 20);
    let images = load_image(&mut window);
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    draw(&images, &data, c, g);
                });
            }
            Event::Loop(Loop::Update(_)) => {
                update_data(&mut data, &key_state);
            }
            Event::Input(i, _) => {
                if let Input::Button(key) = i {
                    key_state.set(&key);
                }
            }
            _ => {}
        }
    }
}