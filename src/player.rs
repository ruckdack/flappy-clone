use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cmp;
use std::convert::TryInto;

pub struct Player {
    pub position: Point,
    pub velocity: Point,
    pub sprite: Rect,
}

impl Player {
    pub fn new(canvas: &Canvas<Window>) -> Result<Player, String> {
        let (width, _) = canvas.output_size()?;
        Ok(Player {
            position: Point::new((width/2 - 25).try_into().unwrap(), 0),
            velocity: Point::new(0, 0),
            sprite: Rect::new(0, 0, 50, 50)
        })
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let prev_color = canvas.draw_color();
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(self.position.x, self.position.y, self.sprite.width(), self.sprite.height()))?;
        canvas.set_draw_color(prev_color);

        Ok(())  
    }

    pub fn update(&mut self) {
        self.velocity = Point::new(self.velocity.x(), cmp::min(self.velocity.y() + 1, 20));
        self.position = Point::new(self.position.x() + self.velocity.x(), self.position.y() + self.velocity.y());
    }

    pub fn jump(&mut self) {
        self.velocity = Point::new(self.velocity.x(), -15);
    }
}