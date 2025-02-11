use console::Key;
use crate::structs::{ROWS, COLUMNS, Cords, Timer};
use std::process::exit;
use console::Term;

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
};

#[derive(Component)]
pub struct Player {
    pub file_path: String,
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
            file_path: String::from("assets/spaceship.png"),
            lives,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(200),
            key_reader: KeyReader::new(),
        }
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
