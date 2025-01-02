use console::Key;
use crate::structs::{ROWS, COLUMNS, Cords, Timer};
use std::process::exit;
use console::Term;

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
};


pub struct Player {
    pub display_char: char,
    pub lives: u8,
    pub current_position: Option<Cords>,
    pub start_position: Cords,
    pub death_timer: Timer, 
    pub key_reader: KeyReader,
}

impl Player {
    pub fn new(lives: u8) -> Self { 
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^', 
            lives,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(200),
            key_reader: KeyReader::new(),
        }
    }

    pub async fn use_key(&mut self) -> Option<Cords> {
        if let Some(Cords(x, y)) = self.current_position {
            match self.key_reader.read_key().await {
                Some(Key::ArrowLeft) => {
                    if y > 0 {
                        self.move_to(Cords(x, y - 1));
                    }
                }
                Some(Key::ArrowRight) => {
                    if y < COLUMNS - 1 {
                        self.move_to(Cords(x, y + 1));
                    }
                }
                Some(Key::ArrowUp) => {
                    return Some(Cords(x - 1, y));
                }
                Some(Key::CtrlC) => exit(0),
                _ => {}
            };
        }
        None
    }

    pub fn move_to(&mut self, new_position: Cords) {
        self.current_position = Some(new_position);
    }

    pub fn handle_collision(&mut self) -> Option<u8> {
        self.lives -= 1;

        if self.lives == 0 {
            None
        } else {
            self.current_position = None;
            Some(self.lives)
        }
    }

    pub fn respawn(&mut self, _can_respawn: bool) {
        if self.current_position.is_none() && self.death_timer.tick() {
            self.move_to(self.start_position);
        }
    }
}

pub struct KeyReader {
    jh: Option<tokio::task::JoinHandle<Key>>,
}

impl KeyReader {
    pub fn new() -> KeyReader {
        KeyReader {
            jh: Some(tokio::spawn(Self::await_key_press())),
        }
    }

    async fn await_key_press() -> Key {
        let term = Term::stdout();
        term.read_key().unwrap()
    }

    pub async fn read_key(&mut self) -> Option<Key> {
        if self.jh.as_ref().unwrap().is_finished() {
            let key = self.jh.take().unwrap().await.unwrap();
            self.jh = Some(tokio::spawn(Self::await_key_press()));
            Some(key)
        } else {
            None
        }
    }
}



//TODO make the code below a copy of tE code above but dont change the key detction becuse its for bevy
// pub fn handle_player_input(
//     mut keyboard_input_events: EventReader<KeyboardInput>,
//     mut player_position: ResMut<PlayerPosition>,
//     mut query: Query<(&mut GridPosition, &mut Transform, &GameEntity), With<GameEntity>>,
//     mut commands: Commands,
//     color_palette: Res<ColorsPalette>,
//     asset_server: Res<AssetServer>,
// ) {
//     let mut delta_x = 0;
//     let mut delta_y = 0;
//     let mut shoot = false;
//
//     // Process keyboard input
//     for event in keyboard_input_events.read() {
//         if let key_code = event.key_code {
//             match key_code {
//                 KeyCode::ArrowLeft => delta_x -= GRID_MOVE_DELTA,
//                 KeyCode::ArrowRight => delta_x += GRID_MOVE_DELTA,
//                 KeyCode::Space => shoot = true,
//                 _ => {}
//             }
//         }
//     }
//
//     // Update player position and transform
//     for (mut grid_pos, mut transform, game_entity) in query.iter_mut() {
//         if game_entity.entity_type == EntityType::Player {
//             // Update the player's grid position
//             grid_pos.x += delta_x;
//             grid_pos.y += delta_y;
//
//             transform.translation = grid_to_world(&grid_pos);
//
//             player_position.0 = GridPosition {
//                 x: grid_pos.x,
//                 y: grid_pos.y,
//             };
//         }
//     }
//
// }
//
