mod actors;
mod components;
mod game;
mod graphics;
mod math;
mod system;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = Game::initialize()?;
    game.run_loop();

    Ok(())
}
