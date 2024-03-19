pub mod piston;
pub mod data;
pub mod update;
pub mod draw;
pub mod calc;

use piston_window::*;
use piston::{create_window};

use update::create_data::create_data;
use update::update_data::update_data;
use draw::{draw, load_image, load_glyphs};
use data::MouseButtonState;

fn main() {
    let mut window: PistonWindow = create_window();

    let mut data = create_data(38, 27, 0.7);
    let images = load_image(&mut window);
    let mut glyphs = load_glyphs(&mut window);
    let mut mouse_button_state = MouseButtonState {
        cursor: data::Position{ x: 0, y: 0 },
        pressed: None,
        released: None
    };
    while let Some(e) = window.next() {
        e.mouse_cursor(|p| {
            mouse_button_state.cursor = data::Position { x: p[0] as i32, y: p[1] as i32}
        });
        if let Some(Button::Mouse(b)) = e.press_args() {
            mouse_button_state.pressed = Some(b);
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Mouse(b) => {
                    mouse_button_state.released = Some(b);
                },
                _ => {},
            }
        };
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, d| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    draw(&images, &mut glyphs, &data, c, g, d);
                });
            },
            Event::Loop(Loop::Update(_)) => {
                update_data(&mut data, &mut mouse_button_state);
            },
            _ => {}
        }
    }
}