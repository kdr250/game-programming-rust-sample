mod game;

use anyhow::Result;
use game::Game;

fn main() -> Result<()> {
    let mut game = Game::initialize()?;
    game.run_loop();

    Ok(())
}
