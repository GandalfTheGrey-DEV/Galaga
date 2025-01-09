use bevy::prelude::{Commands, ResMut, Resource, Timer, Time};
use bevy::utils::Instant;
use crate::game_state::GameState;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS};

#[derive(Resource)]
pub struct GameSettings {
    pub auto_move: bool,
    pub auto_shoot: bool,
    pub no_death: bool,
    last_move_time: Instant,
    last_shot_time: Instant,
}

impl GameSettings {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            auto_move: false,
            auto_shoot: false,
            no_death: true,
            last_move_time: now,
            last_shot_time: now,
        }
    }

    pub fn set_no_death(&mut self, enabled: bool) {
        self.no_death = enabled;
    }

    pub fn set_auto_move(&mut self, enabled: bool) {
        self.auto_move = enabled;
    }

    pub fn set_auto_shoot(&mut self, enabled: bool) {
        self.auto_shoot = enabled;
    }

    pub fn handle_auto_move(&mut self, game_state: &mut GameState, time: &Time) {
        if self.auto_move {
            if self.last_move_time.elapsed().as_millis() >= 500 {
                if let Some(Cords(x, y)) = game_state.player.current_position {
                    let new_y = (y + 1) % COLUMNS;
                    game_state.player.move_to(Cords(x, new_y));
                }
                self.last_move_time = Instant::now();
            }
        }
    }

    pub fn handle_auto_shoot(&mut self, game_state: &mut GameState, commands: &mut Commands, time: &Time) {
        if self.auto_shoot {
            if self.last_shot_time.elapsed().as_millis() >= 500 {
                if let Some(Cords(x, y)) = game_state.player.current_position {
                    let mut fly_detected = false;

                    for row in (0..x).rev() {
                        if game_state.game_board.contains_key(&Cords(row, y)) {
                            fly_detected = true;
                            break;
                        }
                    }

                    for col in (0..y).rev() {
                        if game_state.game_board.contains_key(&Cords(x, col)) {
                            fly_detected = true;
                            break;
                        }
                    }

                    for col in (y + 1)..COLUMNS {
                        if game_state.game_board.contains_key(&Cords(x, col)) {
                            fly_detected = true;
                            break;
                        }
                    }

                    if fly_detected {
                        let bullet_position = Cords(x - 1, y);
                        game_state
                            .add_ship(bullet_position, Ship::new_bullet(false))
                            .ok();

                        self.last_shot_time = Instant::now();
                    }
                }
            }
        }
    }
}