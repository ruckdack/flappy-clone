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
    D(EventType),
    Up(EventType),
    Left(EventType),
    Down(EventType),
    Right(EventType)
}

pub enum EventType {
    Up,
    Down
}

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub entities: LinkedList<GameEntity>, // make private
    sdl_adapter: SdlAdapter,
}

impl Game {
    pub fn new(width: u32, height: u32, title: &str) -> Game {
        let player = GameEntity::new(100, 100, 50, 50, Color::BLUE);
        let mut entities = LinkedList::new();
        entities.push_back(player);
        entities.push_back(GameEntity::new(200, 200, 100, 100, Color::GREEN));
        Game {
            entities,
            sdl_adapter: SdlAdapter::new(width, height, title).unwrap(),
            width,
            height
        }
    }

    // refactor player in general
    pub fn player(&mut self) -> &mut GameEntity {
        self.entities.front_mut().unwrap()
    }
    pub fn clip_player(&mut self) {
         if self.player().x_pos < 0 {
            self.player().x_pos = 0;
        }
        if self.player().x_pos + self.player().width_i32() > self.width_i32() {
            self.player().x_pos = self.width_i32() - self.player().width_i32();
        }
        if self.player().y_pos < 0 {
            self.player().y_pos = 0;
        }
        if self.player().y_pos + self.player().height_i32() > self.height_i32() {
            self.player().y_pos = self.height_i32() - self.player().height_i32();
        }
    }
    // until here

    pub fn width_i32(&self) -> i32{
        i32::try_from(self.width).unwrap()
    }

    pub fn height_i32(&self) -> i32{
        i32::try_from(self.height).unwrap()
    }

    pub fn leaves_window_x(&self, e: &GameEntity) -> bool {
        return e.x_pos < 0 || e.x_pos + e.width_i32() > self.width_i32();
    }

    pub fn leaves_window_y(&self, e: &GameEntity) -> bool {
        return e.y_pos < 0 || e.y_pos + e.width_i32() > self.height_i32();
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
            if collide(&self.entities.front().unwrap(), self.entities.back().unwrap()) {
                println!("collision");
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

pub fn collide(e1: &GameEntity, e2: &GameEntity) -> bool {
    let x_overlap = e1.x_pos < e2.x_pos + e2.width_i32() && e2.x_pos < e1.x_pos + e1.width_i32();
    let y_overlap = e1.y_pos < e2.y_pos + e2.height_i32() && e2.y_pos < e1.y_pos + e1.height_i32();
    return x_overlap && y_overlap;
}