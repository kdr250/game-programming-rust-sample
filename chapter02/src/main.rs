mod actors;
mod components;
mod game;
mod math;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let game = Game::initialize()?;
    game.borrow_mut().run_loop();

    Ok(())
}
