mod game;
mod math;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = Game::initialize()?;
    game.run_loop();

    Ok(())
}
