use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS};
use crossterm::execute;
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::structs::ShipAction;

use crate::player::Player;

pub struct GameState {
    pub game_board: HashMap<Cords, Ship>,
    pub tick_count: u32,
    pub player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            player: Player::new(),
        }
    }

    pub fn display_board(&self) {
        execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0));
        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");

        for row in 0..ROWS {
            print!("           |");
            for col in 0..COLUMNS {
                let position = Cords(row, col);

                if row == ROWS - 1 && col < (self.player.lives -1 as u8).into() {
                    print!("{}", self.player.display_char);
                } else if self.player.current_position == Some(position) {
                    print!("{}", self.player.display_char);
                } else if let Some(ship) = self.game_board.get(&position) {
                    print!("{}", ship.display_char());
                } else {
                    print!(" ");
                }
            }
            println!("|           ");
        }

        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");
    }

    pub fn add_ship(&mut self, cords: Cords, ship: Ship) -> Result<(), String> {
        if cords.0 >= ROWS || cords.1 >= COLUMNS {
            return Err(format!("Coordinates are out of bounds."));
        } else if let Some(_existing_ship) = self.remove_ship(cords) {
            self.game_board.insert(cords, Ship::new_explosion());
        } else {
            self.game_board.insert(cords, ship);
        }
        Ok(())
    }

    pub fn remove_ship(&mut self, cords: Cords) -> Option<Ship> {
        self.game_board.remove(&cords)
    }

    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    pub fn ship_actions(&mut self) -> Result<(), String> {
        let to_update = self
            .game_board
            .iter()
            .map(|(cords, ship)| (*cords, ship.get_id()))
            .collect::<Vec<(Cords, Uuid)>>();

        for (cords, shipid) in to_update {
            if let Some(mut current_ship) = self.game_board.remove(&cords) {
                if current_ship.get_id() != shipid {
                    continue;
                }
                match current_ship.get_action(cords, &mut self.game_board) {
                    ShipAction::Remove => {}
                    ShipAction::Shoot => {
                        let shoot_position = Cords(cords.0 + 1, cords.1);
                        self.add_ship(cords, current_ship)?;
                        self.add_ship(shoot_position, Ship::new_bullet(true))?;
                    }
                    ShipAction::Move(new_cords, wrapped) => {
                        if !wrapped || wrapped && current_ship.wrap() {
                            self.add_ship(new_cords, current_ship)?;
                        }
                    }
                    ShipAction::Nothing => self.add_ship(cords, current_ship)?,
                }
            }
        }
        Ok(())
    }

    pub async fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            if self.game_board.get(&player_pos).is_some() {
                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());
                
                if let Some(lives) = self.player.handle_collision() {
                    println!("oh crap...lives left: {}", lives -1);
                } else {
                    println!("ow, you died.");
                    exit(0);
                }
            }
        }

        self.player.respawn(self.game_board.get(&self.player.start_position).is_none());

        if let Some(bullet_pos) = self.player.use_key().await {
            self.add_ship(bullet_pos, Ship::new_bullet(false)).ok();
        }
    }

    pub async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10));
            self.tick_count += 1;

            self.ship_actions()?;
            self.player_actions().await;


            self.display_board();
        }
    }
}
