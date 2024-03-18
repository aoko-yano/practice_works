use crate::data::Position;

#[derive(Debug)]
pub struct NextTilePositions {
    pub up_right: Option<Position>,
    pub up_left: Option<Position>,
    pub right: Position,
    pub left: Position,
    pub down_right: Option<Position>,
    pub down_left: Option<Position>,
}

pub fn get_next_tile_positions(x: usize, y: usize, pos: &Position) -> NextTilePositions {
    NextTilePositions{
        up_right: calc_up_right_position(x, y, pos),
        up_left: calc_up_left_position(x, y, pos),
        right: calc_right_position(x, y, pos),
        left: calc_left_position(x, y, pos),
        down_right: calc_down_right_position(x, y, pos),
        down_left: calc_down_left_position(x, y, pos),
    }
}

fn get_x_plus_1(x: i32, pos_x: i32) -> i32 {
    if pos_x == x - 1 { 0 } else { pos_x + 1 }
}

fn get_x_minus_1(x: i32, pos_x: i32) -> i32 {
    if pos_x == 0i32 { x - 1 } else { pos_x - 1 }
}

fn calc_up_right_position(x: usize, _y: usize, pos: &Position) -> Option<Position> {
    if pos.y == 0 {
        return None
    }
    Some(Position {
        x: {
            if pos.y % 2 == 0 {
                pos.x
            } else {
                get_x_plus_1(x as i32, pos.x)
            }
        },
        y: pos.y - 1
    })
}

fn calc_up_left_position(x: usize, _y: usize, pos: &Position) -> Option<Position> {
    if pos.y == 0 {
        return None
    }
    Some(Position {
        x: {
            if pos.y % 2 == 0 {
                get_x_minus_1(x as i32, pos.x)
            } else {
                pos.x
            }
        },
        y: pos.y - 1
    })
}

fn calc_right_position(x: usize, _y: usize, pos: &Position) -> Position {
    Position {
        x: get_x_plus_1(x as i32, pos.x),
        y: pos.y
    }
}

fn calc_left_position(x: usize, _y: usize, pos: &Position) -> Position {
    Position {
        x: get_x_minus_1(x as i32, pos.x),
        y: pos.y
    }
}

fn calc_down_right_position(x: usize, y: usize, pos: &Position) -> Option<Position> {
    if pos.y == (y as i32 - 1) {
        return None
    }
    Some(Position {
        x: {
            if pos.y % 2 == 0 {
                pos.x
            } else {
                get_x_plus_1(x as i32, pos.x)
            }
        },
        y: pos.y + 1
    })
}

fn calc_down_left_position(x: usize, y: usize, pos: &Position) -> Option<Position> {
    if pos.y == (y as i32 - 1) {
        return None
    }
    Some(Position {
        x: {
            if pos.y % 2 == 0 {
                get_x_minus_1(x as i32, pos.x)
            } else {
                pos.x
            }
        },
        y: pos.y + 1
    })
}