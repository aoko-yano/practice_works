
extern crate piston_window;

use piston_window::*;

const WINDOW_TITLE: &str = "For the Glory of X-kind";

const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0,
};

pub struct KeysState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl KeysState {
    pub fn new() -> KeysState {
        return KeysState {
            up: false,
            down: false,
            left: false,
            right: false,
        };
    }

    pub fn set(&mut self, key: &ButtonArgs) {
        match key.button {
            Button::Keyboard(Key::Up) => {
                self.up = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Down) => {
                self.down = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Left) => {
                self.left = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Right) => {
                self.right = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            _ => {}
        }
    }
}

pub fn create_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(60);
    window.events.set_ups(1);

    window

}