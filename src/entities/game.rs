use std::collections::LinkedList;
use std::time::Duration;
use sdl2::pixels::Color;

use crate::interface_adapters::sdl_adapter::SdlAdapter;
use crate::entities::game_entity::GameEntity;

pub enum GameEvent {
    Quit,
    Space,
    W,
    A,
    S,
    D
}

pub struct Game {
    entities: LinkedList<GameEntity>,
    sdl_adapter: SdlAdapter,
    width: u32,
    height: u32,
}

impl Game {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Game, String> {
        let player = GameEntity::new(10, 10, 10, 10, Color::BLUE);
        let mut entities = LinkedList::new();
        entities.push_back(player);
        Ok(Game {
            entities,
            sdl_adapter: SdlAdapter::new(width, height, title).unwrap(),
            width,
            height,
        })
    }
    pub fn player(&mut self) -> &mut GameEntity {
        self.entities.front_mut().unwrap()
    }
}

pub trait GameBehaviour {
    fn run_internal(&mut self, game: &mut Game) {
        'running: loop {
            match game.sdl_adapter.poll_event() {
                None => (),
                Some(GameEvent::Quit) => {
                    break 'running;
                },
                Some(game_event) => self.update(game_event)
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    fn render(game: &mut Game) -> Result<(), String> {
        game.sdl_adapter.render_game(&game.entities)
    }
    fn update(&mut self, game_event: GameEvent);
}

