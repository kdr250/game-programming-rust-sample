mod actor;
mod anim_sprite_component;
mod bg_sprite_component;
mod component;
mod game;
mod math;
mod ship;
mod sprite_component;

use crate::game::*;
use anyhow::Result;

fn main() -> Result<()> {
    let game = Game::initialize()?;
    game.borrow_mut().run_loop();

    Ok(())
}
