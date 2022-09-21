mod entities;
mod interface_adapters;
mod services;

use entities::{flappy_game::FlappyGame, game::GameBehaviour};

fn main() -> Result<(), String> {
    let mut flappy_game = FlappyGame::new()?;
    flappy_game.run_internal(&mut flappy_game.game);

    Ok(())
}