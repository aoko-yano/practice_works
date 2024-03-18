use std::collections::HashSet;
use crate::data::Position;
use crate::calc::next_tiles::get_next_tile_positions;

pub fn get_some_hexes_ahead(x: usize, y: usize, pos: &Position, radius: usize) -> anyhow::Result<HashSet<Position>> {
    let mut inside_new = HashSet::<Position>::new();
    inside_new.insert(pos.clone());
    let mut inside_old = HashSet::<Position>::new();
    let mut temp_radius = 0;
    loop {
        let on_boundary: HashSet<Position> = inside_new
            .difference(&inside_old).into_iter().map(|it| { it.clone() }).collect();
        if temp_radius == radius {
            return Ok(on_boundary);
        }
        if on_boundary.is_empty() {
            return Err(anyhow::anyhow!("Too large radius for this planet"))
        }
        update(x, y, &on_boundary, &mut inside_new, &mut inside_old);
        temp_radius += 1;
    }
}

fn update(
    x: usize,
    y: usize,
    on_boundary: &HashSet<Position>,
    inside_new: &mut HashSet<Position>,
    inside_old: &mut HashSet<Position>) {
    let mut inside_temp = inside_new.clone();
    for temp_pos in on_boundary {
        let next_tiles = get_next_tile_positions(x, y, &temp_pos);
        match next_tiles.up_left {
            Some(s) => { inside_temp.insert(s); },
            _ => {}
        }
        match next_tiles.up_right {
            Some(s) => { inside_temp.insert(s); },
            _ => {}
        }
        inside_temp.insert(next_tiles.left);
        inside_temp.insert(next_tiles.right);
        match next_tiles.down_left {
            Some(s) => { inside_temp.insert(s); },
            _ => {}
        }
        match next_tiles.down_right {
            Some(s) => { inside_temp.insert(s); },
            _ => {}
        }
    }
    std::mem::swap(inside_new, &mut inside_temp);
    std::mem::swap(inside_old, &mut inside_temp);
}