mod game;

use game::Game;

fn main() {
    if let Ok(mut game) = Game::initialize() {
        game.run_loop();
    }
}
