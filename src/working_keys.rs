use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::process::Command;
use std::process::exit;
use console::{Key, Term};

const ROWS: usize = 5;
const COLUMNS: usize = 10;

pub type Cords = (u32, u32);


trait Ship {
    ///  Returns the character to display the ship on the game board.
    fn display_char(&self) -> char;

    /// Updates the position of the ship based on its internal behavior.
    /// If the ship moves, returns the new position; otherwise, returns `None`.
    fn update_position(&mut self, current_coords: Cords) -> Option<Cords>;
}

///  Represents the player's ship.
struct Player;

impl Ship for Player {
    ///  Returns the character representing the player's ship (`P`).
    fn display_char(&self) -> char {
        'P'
    }

    ///  The player's ship doesn't move automatically.
    fn update_position(&mut self, _current_coords: Cords) -> Option<Cords> {
        None
    }
}

struct Fly {
    direction_index: usize, //  Tracks the current movement direction.
}

impl Fly {
    ///  Creates a new enemy spaceship with an initial movement direction.
    fn new() -> Self {
        Fly { direction_index: 0 }
    }


    fn movement_directions() -> Vec<(i32, i32)> {
        vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
    }
}

impl Ship for Fly {
    /// Returns the character representing an enemy spaceship (`F`).
    fn display_char(&self) -> char {
        'F'
    }

    ///  Updates the spaceship's position based on its current movement direction.
    /// The spaceship moves cyclically through `movement_directions`.
    fn update_position(&mut self, current_coords: Cords) -> Option<Cords> {
        let directions = Fly::movement_directions();
        let direction = directions[self.direction_index];
        let new_x = (current_coords.0 as i32 + direction.0) as u32;
        let new_y = (current_coords.1 as i32 + direction.1) as u32;

        // Cycle to the next direction.
        self.direction_index = (self.direction_index + 1) % directions.len();
        Some((new_x, new_y))
    }
}

///  Represents a bullet fired by either the player or enemies.
struct Bullet {
    direction: BulletDirection, // Tracks the direction the bullet is moving.
}

///  Enum for bullet movement direction (up or down).
enum BulletDirection {
    Up,
    Down,
}

impl Bullet {
    fn new(direction: BulletDirection) -> Self {
        Bullet { direction }
    }
}

impl Ship for Bullet {
    ///  Returns the character representing a bullet (`|`).
    fn display_char(&self) -> char {
        '|'
    }

    /// Moves the bullet in its current direction.
    /// Bullets move vertically until they leave the game board.
    fn update_position(&mut self, current_coords: Cords) -> Option<Cords> {
        match self.direction {
            BulletDirection::Up => {
                if current_coords.0 > 0 {
                    Some((current_coords.0 - 1, current_coords.1))
                } else {
                    None
                }
            }
            BulletDirection::Down => {
                if current_coords.0 < (ROWS as u32) - 1 {
                    Some((current_coords.0 + 1, current_coords.1))
                } else {
                    None
                }
            }
        }
    }
}

/// Represents an explosion (created after a collision).
struct ExplosionShip;

impl Ship for ExplosionShip {
    /// Returns the character representing an explosion (`*`).
    fn display_char(&self) -> char {
        '*'
    }

    /// Explosions don't move after being created.
    fn update_position(&mut self, _current_coords: Cords) -> Option<Cords> {
        None
    }
}


struct KeyReader {
    jh: tokio::task::JoinHandle<Key>
}

impl KeyReader {
    pub fn new() -> KeyReader {
        KeyReader {
            jh: tokio::spawn(Self::await_key_press())
        }
    }

    async fn await_key_press() -> Key {
        let term = Term::stdout();
        term.read_key().unwrap()
    }

    pub async fn read_key(&self) -> Option<Key> {
        if self.jh.is_finished() {
            let key = self.jh.unwrap()?;
            self.jh = tokio::spawn(Self::await_key_press())
            Some(key)
        } else {
            None
        }
    }
}


///  Represents the entire game state, including the game board and ships.
struct GameState {
    game_board: HashMap<Cords, Box<dyn Ship>>, 
    tick_count: u32,
    key: KeyReader,   
}

impl GameState {
    ///  Initializes a new, empty game state.
    fn new() -> GameState {
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
        }
    }

    /// Displays the current state of the game board.
    /// Clears the console and redraws the board with all entities.
    fn display_board(&self) {
        Command::new("clear").status().ok(); // Clear the screen (ignore errors).
        println!("Game Board:");
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                if let Some(ship) = self.game_board.get(&(row as u32, col as u32)) {
                    print!("{}", ship.display_char());
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    fn check_key(&self) {
        
    }

    ///  Adds a new ship to the game board at the specified coordinates.
    /// Handles collisions by creating an explosion.
    pub fn add_ship(&mut self, x: u32, y: u32, ship: Box<dyn Ship>) -> Result<(), String> {
        if x >= ROWS as u32 || y >= COLUMNS as u32 {
            return Err(format!("Coordinates ({}, {}) are out of bounds.", x, y));
        }

        let position = (x, y);

        if let Some(existing_ship) = self.game_board.remove(&position) {
            self.collision(existing_ship, ship); // 💥 Handle collision.
            self.game_board.insert(position, Box::new(ExplosionShip)); // Replace with explosion.
        } else {
            self.game_board.insert(position, ship);
        }
        Ok(())
    }

    /// Removes a ship from the specified coordinates, if any.
    fn remove_ship(&mut self, x: u32, y: u32) -> Option<Box<dyn Ship>> {
        self.game_board.remove(&(x, y))
    }

    /// Logs a collision between two ships.
    fn collision(&self, ship1: Box<dyn Ship>, ship2: Box<dyn Ship>) {
        println!(
            "Collision between {} and {}",
            ship1.display_char(),
            ship2.display_char()
        );
    }

    ///  Moves a ship from one position to another.
    /// If a collision occurs, handles it appropriately.
    fn move_ship(&mut self, old_x: u32, old_y: u32, new_x: u32, new_y: u32) {
        if let Some(ship) = self.remove_ship(old_x, old_y) {
            self.add_ship(new_x, new_y, ship).ok(); // Ignore errors for simplicity.
        }
    }

    /// Automatically moves all ships according to their behavior.
    fn auto_move_ships(&mut self) {
        let mut to_move = Vec::new();
        let mut to_remove = Vec::new();

        for (&coords, mut ship) in self.game_board.iter_mut() {
            if let Some(new_coords) = ship.update_position(coords) {
                to_move.push((coords, new_coords));
            } else {
                to_remove.push(coords);
            }
        }

        for coords in to_remove {
            self.remove_ship(coords.0, coords.1);
        }

        for (old_coords, new_coords) in to_move {
            self.move_ship(old_coords.0, old_coords.1, new_coords.0, new_coords.1);
        }
    }

    /// The main game loop.
    async fn start_game(&mut self) -> Result<() {


        loop {
            thread::sleep(Duration::from_millis(10)); 
            self.tick_count += 1;
            self.auto_move_ships(); 
            self.display_board(); 

        }
    }
}
#[tokio::main]
async fn main() -> Result<(), String> {
    let mut game = GameState::new();

    game.add_ship(1, 1, Box::new(Player))?; 
    game.add_ship(2, 2, Box::new(Fly::new()))?; 
    game.add_ship(0, 5, Box::new(Bullet::new(BulletDirection::Down)))?; 
    game.add_ship(4, 7, Box::new(Bullet::new(BulletDirection::Up)))?;

    game.start_game().await; 
    Ok(())
}