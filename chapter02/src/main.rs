mod actor;
mod component;
mod game;
mod math;
mod sprite_component;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = Game::initialize()?;
    game.run_loop();

    Ok(())
}
