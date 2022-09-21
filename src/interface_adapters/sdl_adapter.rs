use std::collections::LinkedList;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::entities::game::{GameEvent, EventType};
use crate::entities::game_entity::GameEntity;

pub struct SdlAdapter {
    canvas: Canvas<Window>,
    event_pump: EventPump,
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

        canvas.set_draw_color(Color::WHITE);
        let event_pump = sdl_context.event_pump()?;
        Ok(SdlAdapter {
            canvas,
            event_pump
        })

    }
    pub fn render_game(&mut self, game_entites: &LinkedList<GameEntity>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        for entity in game_entites {
            self.canvas.set_draw_color(entity.color());
            self.canvas.fill_rect(
                Rect::new(
                    entity.x_pos - entity.width_i32() / 2,
                    entity.y_pos - entity.height_i32() / 2,
                    entity.width(),
                    entity.height()
                )
            )?;
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
                    return Some(GameEvent::Space(EventType::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    return Some(GameEvent::Space(EventType::Up))
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    return Some(GameEvent::W(EventType::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    return Some(GameEvent::W(EventType::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    return Some(GameEvent::A(EventType::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    return Some(GameEvent::A(EventType::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    return Some(GameEvent::S(EventType::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    return Some(GameEvent::S(EventType::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    return Some(GameEvent::D(EventType::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    return Some(GameEvent::D(EventType::Up));
                },
                _ => {
                    return None;
                }
            }
        }
        None
        
    }
}
