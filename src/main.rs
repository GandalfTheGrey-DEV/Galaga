mod ship;
mod player;
mod game_state;
mod structs;

use ship::Ship;
use game_state::GameState;
use crate::structs::{Cords};
use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal;

#[tokio::main]
async fn main() -> Result<(), String> {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
    let mut game = GameState::new();
    
    
    game.add_ship(Cords(2, 3), Ship::new_fly())?;
    game.add_ship(Cords(3, 4), Ship::new_fly())?;
    game.add_ship(Cords(2, 5), Ship::new_fly())?;
    game.add_ship(Cords(3, 6), Ship::new_fly())?;
    game.add_ship(Cords(2, 7), Ship::new_fly())?;
    game.add_ship(Cords(3, 8), Ship::new_fly())?;
    game.add_ship(Cords(2, 9), Ship::new_fly())?;
    game.add_ship(Cords(3, 10), Ship::new_fly())?;
    

    game.start_game().await?;
    Ok(())
}
