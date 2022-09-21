use std::collections::LinkedList;
use std::time::Duration;
use sdl2::pixels::Color;

use crate::interface_adapters::sdl_adapter::SdlAdapter;
use crate::entities::game_entity::GameEntity;

pub enum GameEvent {
    Quit,
    Space(EventType),
    W(EventType),
    A(EventType),
    S(EventType),
    D(EventType)
}

pub enum EventType {
    Up,
    Down
}

pub struct Game {
    entities: LinkedList<GameEntity>,
    sdl_adapter: SdlAdapter,
}

impl Game {
    pub fn new(width: u32, height: u32, title: &str) -> Game {
        let player = GameEntity::new(100, 100, 50, 50, Color::BLUE);
        let mut entities = LinkedList::new();
        entities.push_back(player);
        Game {
            entities,
            sdl_adapter: SdlAdapter::new(width, height, title).unwrap()
        }
    }
    pub fn player(&mut self) -> &mut GameEntity {
        self.entities.front_mut().unwrap()
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            match self.sdl_adapter.poll_event() {
                None => (),
                Some(GameEvent::Quit) => {
                    return Ok(());
                },
                Some(game_event) => self.process_events(game_event)
            }
            self.update();
            match self.render() {
                Err(x) => return Err(x),
                _ => {}
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    fn render(&mut self) -> Result<(), String> {
        self.sdl_adapter.render_game(&self.entities)
    }
}

pub trait GameBehaviour {
    fn process_events(&mut self, game_event: GameEvent);
    fn update(&mut self);
}

