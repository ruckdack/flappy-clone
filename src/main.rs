mod entities;
mod interface_adapters;
mod services;

use entities::flappy_game::FlappyGame;

fn main() -> Result<(), String> {
    let mut flappy_game = FlappyGame::new(1200, 800, "test");
    flappy_game.run();

    Ok(())
}