
extern crate piston_window;

use piston_window::*;

const WINDOW_TITLE: &str = "For the Glory of X-kind";

const WINDOW_SIZE: Size = Size {
    width: 1280.0,
    height: 720.0,
};

pub fn create_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(60);
    window.events.set_ups(60);

    window

}