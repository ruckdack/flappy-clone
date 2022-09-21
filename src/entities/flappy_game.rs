use super::game::{Game, GameBehaviour, GameEvent};

pub struct FlappyGame {
    pub game: Game
}


impl FlappyGame {
    pub fn new() -> Result<FlappyGame, String> {
        Ok(FlappyGame { 
            game: Game::new(1200, 800, "test")?
        })
    }
    // pub fn run(&mut self) {
    //     self.run_internal(&mut self.game);
    // }
    fn jump(&mut self) {
        self.game.player().y_vel = -10
    }
}

impl GameBehaviour for FlappyGame {
    fn update(&mut self, game_event: GameEvent) {
        match game_event {
            GameEvent::Space => self.jump(),
            _ => {}
        }

    }
}