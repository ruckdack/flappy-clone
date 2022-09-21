use super::game::{Game, GameBehaviour, GameEvent, EventType};

pub type FlappyGame = Game;

impl FlappyGame {
    fn jump(&mut self) {
        self.player().y_vel = -50;
    }
}

impl GameBehaviour for FlappyGame {
    fn process_events(&mut self, game_event: GameEvent) {
        match game_event {
            GameEvent::Space(EventType::Down) => self.jump(),
            GameEvent::W(EventType::Down) |
            GameEvent::Up(EventType::Down) => self.player().y_vel = -5,
            GameEvent::A(EventType::Down) |
            GameEvent::Left(EventType::Down) => self.player().x_vel = -5,
            GameEvent::S(EventType::Down) |
            GameEvent::Down(EventType::Down) => self.player().y_vel = 5,
            GameEvent::D(EventType::Down) |
            GameEvent::Right(EventType::Down) => self.player().x_vel = 5,
            GameEvent::W(EventType::Up) |
            GameEvent::Up(EventType::Up) | 
            GameEvent::S(EventType::Up) |
            GameEvent::Down(EventType::Up) => self.player().y_vel = 0,
            GameEvent::A(EventType::Up) |
            GameEvent::Left(EventType::Up) |
            GameEvent::D(EventType::Up) | 
            GameEvent::Right(EventType::Up) => self.player().x_vel = 0,
            _ => {}
        }

    }

    fn update(&mut self) {
        self.player().x_pos += self.player().x_vel;
        self.player().y_vel += 1;
        self.player().y_pos += self.player().y_vel;

        if self.leaves_window_y(self.entities.front().unwrap()) {
            self.player().y_vel = -self.player().y_vel;
        }
        self.clip_player();
    }
}