use std::collections::LinkedList;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::entities::game::GameEvent;
use crate::entities::game_entity::GameEntity;

pub struct SdlAdapter {
    canvas: Canvas<Window>,
    event_pump: EventPump
}

impl SdlAdapter {
    pub fn new(width: u32, height: u32, title: &str) -> Result<SdlAdapter, String>{
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let event_pump = sdl_context.event_pump()?;

        Ok(SdlAdapter {
            canvas,
            event_pump
        })

    }
    pub fn render_game(&mut self, game_entites: &LinkedList<GameEntity>) -> Result<(), String> {
        self.canvas.clear();
        for entity in game_entites {
            self.canvas.set_draw_color(entity.color);
            self.canvas.fill_rect(Rect::new(entity.x_pos - i32::try_from(entity.width / 2).unwrap(), entity.y_pos - i32::try_from(entity.height / 2).unwrap(), entity.width, entity.height))?;
        }
        self.canvas.present();
        Ok(())
    }
    pub fn poll_event(&mut self) -> Option<GameEvent> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Some(GameEvent::Quit);
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    return Some(GameEvent::Space);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    return Some(GameEvent::W);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    return Some(GameEvent::A);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    return Some(GameEvent::S);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    return Some(GameEvent::D);
                },
                _ => {
                    return None;
                }
            }
        }
        None
        
    }
}
