use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS, Level, GameLevel};
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use bevy::prelude::*;
use bevy::ecs::system::Resource;
use crate::structs::ShipAction;
use crate::player::Player;
use crate::settings::GameSettings;

#[derive(Resource)]
pub struct GameState {
    pub game_board: HashMap<Cords, Ship>,
    pub tick_count: u32,
    pub player: Player,
    pub gamelevel: GameLevel,

    //SCORE
    pub score: u32,
}

impl GameState {
    pub fn new() -> GameState {
        let game_level = GameLevel::new(Level::Easy);
        let level_status = game_level.get_level_status();

        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            player: Player::new(level_status.1),
            gamelevel: game_level,
            //SCORE
            score: 0,
        }
    }

    //SCORE
    pub fn increase_score(&mut self, points: u32) {
        self.score += points;
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
        if let Some(ship) = self.game_board.remove(&cords) {
            if ship.is_fly() {
                self.increase_score(100);
                println!("Score updated: {}", self.score);
            }
            return Some(ship);
        }
        None
    }

    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    pub fn ship_actions(&mut self, game_settings: &GameSettings) -> Result<(), String> {
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
                    ShipAction::Remove => {
                        self.remove_ship(cords);
                    }
                    ShipAction::Shoot => {
                        let shoot_position = Cords(cords.0 + 1, cords.1);
                        self.add_ship(cords, current_ship)?;
                        self.add_ship(shoot_position, Ship::new_bullet(true, 15))?;
                    }
                    ShipAction::Move(new_cords, wrapped) => {
                        if !wrapped || (wrapped && current_ship.wrap()) {
                            self.add_ship(new_cords, current_ship)?;
                        }
                    }
                    ShipAction::Nothing => self.add_ship(cords, current_ship)?,
                }
            }
        }
        Ok(())
    }

    pub fn player_actions(&mut self, game_settings: &GameSettings) {
        if let Some(player_pos) = self.player.current_position {
            if let Some(_) = self.game_board.get(&player_pos) {
                if game_settings.invocable {
                    self.remove_ship(player_pos);
                    self.player.respawn(true);
                    return;
                }

                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());

                if let Some(lives) = self.player.handle_collision() {
                } else {
                    exit(0);
                }
            }
        }

        self.player.respawn(self.game_board.get(&self.player.start_position).is_none());
    }

    pub async fn start_game(&mut self, game_settings: &GameSettings) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10));
            self.tick_count += 1;
            self.ship_actions(&game_settings)?;
            self.player_actions(&game_settings);
        }
    }
}