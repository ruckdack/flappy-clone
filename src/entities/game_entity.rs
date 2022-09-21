use sdl2::pixels::Color;

pub struct GameEntity {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    width: u32,
    height: u32,
    color: Color,
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
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn width_i32(&self) -> i32 {
        i32::try_from(self.width).unwrap()
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn height_i32(&self) -> i32 {
        i32::try_from(self.height).unwrap()
    }
    pub fn color(&self) -> Color {
        self.color
    }
}