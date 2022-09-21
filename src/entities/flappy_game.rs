use super::game::{Game, GameBehaviour, GameEvent, EventType};

pub type FlappyGame = Game;

impl FlappyGame {
    fn jump(&mut self) {
        self.player().y_vel = -25;
    }
}

impl GameBehaviour for FlappyGame {
    fn process_events(&mut self, game_event: GameEvent) {
        match game_event {
            GameEvent::Space(EventType::Down) => self.jump(),
            GameEvent::W(EventType::Down) => self.player().y_vel = -5,
            GameEvent::A(EventType::Down) => self.player().x_vel = -5,
            GameEvent::S(EventType::Down) => self.player().y_vel = 5,
            GameEvent::D(EventType::Down) => self.player().x_vel = 5,
            GameEvent::W(EventType::Up) | 
            GameEvent::S(EventType::Up) => self.player().y_vel = 0,
            GameEvent::A(EventType::Up) |
            GameEvent::D(EventType::Up) => self.player().x_vel = 0,
            _ => {}
        }

    }
    fn update(&mut self) {
        self.player().x_pos += self.player().x_vel;
        self.player().y_vel += 1;
        self.player().y_pos += self.player().y_vel;
    }
}