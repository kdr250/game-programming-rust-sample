mod actors;
mod components;
mod entity_manager;
mod game;
mod math;
mod texture_manager;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = Game::initialize()?;
    game.run_loop();

    Ok(())
}
