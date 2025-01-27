use bevy::prelude::{Commands, ResMut, Resource, Timer, Time};
use bevy::utils::Instant;
use crate::game_state::GameState;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS};

#[derive(Resource)]
pub struct GameSettings {
    //below settings
    pub auto_pilot: bool,
    pub auto_shoot: bool,
    pub invocable: bool,
    pub fly_move: bool,
    pub laser_shoot: bool,

    //no need to touch below
    pub keyboard_enabled: bool,
    pub gamepad_enabled: bool,
    pub number_of_flys: u32,
    last_move_time: Instant,
    last_shot_time: Instant,
}

impl GameSettings {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            auto_pilot: false,
            auto_shoot: false,
            invocable: false,
            keyboard_enabled: true,
            gamepad_enabled: true,
            fly_move: true,
            laser_shoot: true,
            number_of_flys: 10,
            last_move_time: now,
            last_shot_time: now,

        }
    }

    pub fn calculate_fly_value(&self) -> u32 {
        self.number_of_flys * 2
    }

    pub fn set_fly_speed(&self, laser_speed: u32, value: u32) -> u32 {
        if laser_speed == 10 {
            match value {
                1 => 100,
                10 => 500,
                2..=9 => 100 + ((value - 1) as f32 * 50.0).round() as u32,
                _ => {
                    eprintln!("Invalid value: fly_speed must be between 1 and 10. Using default value of 1.");
                    100
                }
            }
        } else {
            match value {
                1 => 10,
                10 => 500,
                2..=9 => 10 + ((value - 1) as f32 * 54.444).round() as u32,
                _ => {
                    eprintln!("Invalid value: fly_speed must be between 1 and 10. Using default value of 1.");
                    10
                }
            }
        }
    }
    pub fn set_laser_speed(&self, value: u32) -> u32 {
        match value {
            1 => 10,
            10 => 100,
            2..=9 => 10 + ((value - 1) * 10),
            _ => panic!("Invalid value: laser_speed must be between 1 and 10."),
        }
    }


    pub fn set_keyboard_enabled(&mut self, enabled: bool) {
        self.keyboard_enabled = enabled;
    }

    pub fn set_gamepad_enabled(&mut self, enabled: bool) {
        self.gamepad_enabled = enabled;
    }

    pub fn set_no_death(&mut self, enabled: bool) {
        self.invocable = enabled;
    }

    pub fn set_auto_move(&mut self, enabled: bool) {
        self.auto_pilot = enabled;
    }

    pub fn set_auto_shoot(&mut self, enabled: bool) {
        self.auto_shoot = enabled;
    }

    pub fn handle_auto_move(&mut self, game_state: &mut GameState, time: &Time) {
        if self.auto_pilot {
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
                            .add_ship(bullet_position, Ship::new_bullet(false, 15))
                            .ok();

                        self.last_shot_time = Instant::now();
                    }
                }
            }
        }
    }
}