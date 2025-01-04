mod ship;
mod player;
mod game_state;
mod structs;
mod game;


use game::Game;

#[tokio::main]
async fn main() -> Result<(), String> {
    let game_instance = Game;
    game_instance.start();
    Ok(())
}
