mod ship;
mod player;
mod game_state;
mod structs;

use crate::game_state::Game;

#[tokio::main]
async fn main() -> Result<(), String> {
    Game::start(&Game);
    Ok(())
}
