use console::{Key, Term};
use std::collections::HashMap;
use std::process::exit;
use std::process::Command;
use std::thread;
use std::time::Duration;

const SIZE: usize = 10;

const ROWS: usize = SIZE;
const COLUMNS: usize = SIZE*2;

pub type Cords = (usize, usize);

// 7. Enum ShipAction Can be Nothing, Remove, Move(Cords)
//    1. Nothing does nothing next ship
//    2. Remove remove the ship
//    3. Move(Cords) move the ship to the new cords provided

#[derive(Clone)]
enum ShipAction {
    Nothing,
    Remove,
    Move(Cords),
    Shoot,
    RelativeMove((i8, i8)),
}

enum Ship {
    Player,
    Fly(ShipAI, bool),
    Explosion(ShipAI, bool),
    Bullet(ShipAI, bool)
}

impl Ship {
    pub fn display_char(&self) -> char {
        match self {
            Ship::Player => '^',
            Ship::Fly(_, _) => 'F',
            Ship::Explosion(_, _) => 'X',
            Ship::Bullet(_, _) => '|',
        }
    }

    pub fn update(&mut self) -> ShipAction {
        match self {
            Ship::Player => ShipAction::Nothing,
            Ship::Fly(ai, _) => ai.get_action(),
            Ship::Explosion(ai, _) => ai.get_action(),
            Ship::Bullet(ai, _) => ai.get_action(),
        }
    }

    pub fn new_player() -> Self {
        Self::Player
    }

    pub fn new_fly() -> Self {
        Self::Fly(ShipAI::new(
            100, 
            vec![
                ShipAction::RelativeMove((1, -1)),
                ShipAction::Shoot,
                ShipAction::RelativeMove((-1, -1)),
                ShipAction::Shoot,
            ]
        ), true)
    }

    pub fn new_bullet(moving_down: bool) -> Self {
        let movement = if moving_down { (1, 0) } else { (-1, 0) };
        Self::Bullet(ShipAI::new(
            10, 
            vec![ShipAction::RelativeMove(movement)]
        ), false)
    }

    pub fn new_explosion() -> Self {
        Self::Explosion(ShipAI::new(
            10, 
            vec![ShipAction::Remove]
        ), false)
    }
}

// End
enum GameAction {
    AddShip(Cords, Ship),
    Remove(Cords),
    Move(Cords, Cords),
}

struct Timer {
    current_time: u64,
    interval: u64,
}

impl Timer {
    pub fn new(interval: u64) -> Self {
        Timer {
            current_time: 0,
            interval,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.current_time += 1;
        if self.current_time >= self.interval {
            self.current_time = 0; 
            true
        } else {
            false
        }
    }
}

struct ShipAI {
    timer: Timer,
    actions: Vec<ShipAction>,
    action_index: usize,
}

impl ShipAI {
    fn new(action_interval: u64, actions: Vec<ShipAction>) -> Self {
        ShipAI {
            timer: Timer::new(action_interval),
            actions,
            action_index: 0,
        }
    }

    pub fn get_action(&mut self) -> ShipAction {
        if self.actions.is_empty() {
            return ShipAction::Nothing;
        }

        if self.timer.tick() {
            let action = self.actions[self.action_index].clone();
            if self.action_index == self.actions.len() - 1 {
                self.action_index = 0;
            } else {
                self.action_index += 1;
            }
            action
        } else {
            ShipAction::Nothing
        }
    }
}

//End
struct Player {
    display_char: char,
    lives: u64,
    current_position: Option<Cords>,
    start_position: Cords,
    death_timer: Option<Timer>, 
}

impl Player {
    fn new() -> Self {
        let start_position = (ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^',
            lives: 4,
            current_position: Some(start_position),
            start_position,
            death_timer: None,
        }
    }

    fn move_to(&mut self, new_position: Option<Cords>) {
        self.current_position = new_position;
    }

    fn start_death_timer(&mut self, interval: u64) {
        self.death_timer = Some(Timer::new(interval));
    }

    fn tick_death_timer(&mut self) -> bool {
        if let Some(timer) = &mut self.death_timer {
            if timer.tick() {
                self.death_timer = None; 
                return true;
            }
        }
        false
    }
}

// KeyReader struct for asynchronous key reading
struct KeyReader {
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
// End

// GameState struct containing game logic and state

struct GameState {
    game_board: HashMap<Cords, Ship>,
    tick_count: u32,
    key_reader: KeyReader,
    player: Player,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            key_reader: KeyReader::new(),
            player: Player::new(),
        }
    }

    fn display_board(&self) {
        Command::new("clear").status().ok();
        print!("+");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+");
    
        for row in 0..ROWS {
            print!("|"); 
            for col in 0..COLUMNS {
                let position = (row, col);
    
                if row == ROWS - 1 && col < self.player.lives as usize {
                    print!("{}", self.player.display_char);
                } else if self.player.current_position == Some(position) {
                    print!("{}", self.player.display_char);
                } else if let Some(ship) = self.game_board.get(&position) {
                    print!("{}", ship.display_char());
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
    
        print!("+");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+");
    }
    
 
    

    async fn use_key(&mut self) {
        if let Some((x, y)) = self.player.current_position {
            let new_coords = match self.key_reader.read_key().await {
                Some(Key::ArrowLeft) => {
                    if y > 0 {
                        Some((x, y - 1))
                    } else {
                        None
                    }
                }
                Some(Key::ArrowRight) => {
                    if y < COLUMNS - 1 {
                        Some((x, y + 1))
                    } else {
                        None
                    }
                }
                Some(Key::ArrowUp) => {
                    if x > 0 {
                        self.add_ship((x - 1, y), Ship::new_bullet(false))
                            .ok();
                    }
                    None
                }
                Some(Key::CtrlC) => exit(0),
                _ => None,
            };

            if let Some(new_coords) = new_coords {
                self.player.move_to(Some(new_coords));
            }
        }
    }
    

    pub fn add_ship(&mut self, cords: Cords, ship: Ship) -> Result<(), String> {
        if cords.0 >= ROWS || cords.1 >= COLUMNS {
            return Err(format!("Coordinates are out of bounds."));
        } else {
            if let Some(existing_ship) = self.remove_ship(cords) {
                self.game_board.insert(cords, Ship::new_explosion());
            } else {
                self.game_board.insert(cords, ship);
            }
        }
        Ok(())
    }

    fn remove_ship(&mut self, cords: Cords) -> Option<Ship> {
        self.game_board.remove(&cords)
    }

    fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    fn ship_actions(&mut self) -> Vec<GameAction> {
        self.game_board
            .iter_mut()
            .flat_map(|(&cords, ship)| match ship.update() {
                ShipAction::Move(new_cords) => Some(GameAction::Move(cords, new_cords)),
                ShipAction::Remove => Some(GameAction::Remove(cords)),
                ShipAction::Shoot => {
                    let shoot_position = (cords.0+1, cords.1);
                    Some(GameAction::AddShip(shoot_position, Ship::new_bullet(true)))
                }
                ShipAction::RelativeMove((change_x, change_y)) => {
                    let ship_can_wrap = match ship {
                        Ship::Player => false,
                        Ship::Fly(_, wrap) => *wrap,
                        Ship::Explosion(_, wrap) => *wrap,
                        Ship::Bullet(_, wrap) => *wrap,
                    };
                    let mut new_x = cords.0 as i32 + change_x as i32;
                    let mut new_y = cords.1 as i32 + change_y as i32;
        
                    if ship_can_wrap {
                        new_x = new_x.rem_euclid(ROWS as i32);
                        new_y = new_y.rem_euclid(COLUMNS as i32);
                    } else {
                        if new_x < 0 || new_x >= ROWS as i32 || new_y < 0 || new_y >= COLUMNS as i32 {
                            return Some(GameAction::Remove(cords)); 
                        }
                        new_x = new_x.clamp(0, ROWS as i32 - 1);
                        new_y = new_y.clamp(0, COLUMNS as i32 - 1);
                    }
    
                    let wrapped_x = new_x as usize;
                    let wrapped_y = new_y as usize;
    
                    Some(GameAction::Move(cords, (wrapped_x, wrapped_y)))
                }
                ShipAction::Nothing => None,
            })
            .collect()
    }
    
    fn update(&mut self, actions: Vec<GameAction>) -> Result<(), String> {
        for action in actions {
            match action {
                GameAction::AddShip(cords, ship) => {
                    self.add_ship(cords, ship)?;
                }
                GameAction::Remove(cords) => {
                    self.remove_ship(cords);
                }
                GameAction::Move(old_cords, new_cords) => {
                    self.move_ship(old_cords, new_cords);
                }
            }
        }
        Ok(())
    }


    fn player_collision(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            if self.game_board.get(&player_pos).is_some() {
                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());
    
                self.player.lives -= 1;
    
                if self.player.lives == 0 {
                    println!("Game Over! You've lost all your lives.");
                    exit(0);
                } else {
                    println!(
                        "Collision detected! Lives remaining: {}",
                        self.player.lives
                    );
    
                    self.player.current_position = None;
                    self.player.start_death_timer(200); 
                }
            }
        }

        if self.player.current_position.is_none() && self.player.tick_death_timer() {
            if self.game_board.get(&self.player.start_position).is_none() {
                self.player.move_to(Some(self.player.start_position));
            } else {
                self.player.start_death_timer(100); 
            }
        }
    }
    
    

    async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10));
            self.tick_count += 1;

            let actions = self.ship_actions();
            self.update(actions)?;
            self.player_collision();

            self.display_board();
            self.use_key().await;
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut game = GameState::new();

    game.add_ship((2, 2), Ship::new_fly())?;

    game.start_game().await?;
    Ok(())
}
