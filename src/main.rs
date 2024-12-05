mod ship;
mod player;
mod game_state;
mod structs;

use ship::Ship;
use game_state::GameState;
use crate::structs::Cords;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut game = GameState::new();


    game.add_ship(Cords(2, 3), Ship::new_fly())?;

    game.add_ship(Cords(2, 5), Ship::new_fly())?;

    game.add_ship(Cords(3, 3), Ship::new_fly())?;

    game.add_ship(Cords(3, 5), Ship::new_fly())?;
    game.start_game().await?;
    Ok(())
}
