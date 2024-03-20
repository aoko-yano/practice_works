pub fn get_tile_position(x: i32, y: i32, image_width: f64, image_height: f64) -> (f64, f64) {
    (
        30.0 + x as f64 * image_width + (image_width / 2.0) * (if y % 2 == 0 {0.0} else {1.0}),
        15.0 + y as f64 * (image_height - 14.0),
    )
}