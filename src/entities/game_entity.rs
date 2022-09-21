use sdl2::pixels::Color;

pub struct GameEntity {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

impl GameEntity {
    pub fn new(x_pos: i32, y_pos: i32, width: u32, height: u32, color: Color) -> GameEntity {
        GameEntity {
            x_pos,
            y_pos,
            x_vel: 0,
            y_vel: 0,
            width,
            height,
            color
        }
    }
}