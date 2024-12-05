use console::{Key, Term};
use std::collections::HashMap;
use std::process::exit;
use std::process::Command;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use crossterm::execute;

const SIZE: usize = 10;

const ROWS: usize = SIZE;
const COLUMNS: usize = SIZE*2;

pub type Cords = (usize, usize);

enum GameAction {
    AddShip(Cords, Ship),
    Remove(Cords),
    Move(Cords, Cords),
}

#[derive(Clone)]
enum ShipAction {
    Nothing,
    Remove,
    Shoot,
    RelativeMove((i8, i8)),
}

impl ShipAction {
    pub fn to_game_action(self, ship: &Ship, cords: Cords) -> (Option<Cords>, Option<GameAction>) {
        match self {
            ShipAction::Remove => (None, None),
            ShipAction::Shoot => {
                let shoot_position = (cords.0 + 1, cords.1);
                (Some(cords), Some(GameAction::AddShip(shoot_position, Ship::new_bullet(true))))
            }
            ShipAction::RelativeMove((change_x, change_y)) => {
                let ship_can_wrap = ship.wrap();
                let mut nx = cords.0 as i32 + change_x as i32;
                let mut ny = cords.1 as i32 + change_y as i32;
                let cords = if ship_can_wrap {
                    Some((nx.rem_euclid(ROWS as i32) as usize, ny.rem_euclid(COLUMNS as i32) as usize))
                } else {
                    if nx < 0 || nx >= ROWS as i32 || ny < 0 || ny >= COLUMNS as i32 {
                        None
                    } else {
                        Some((nx.clamp(0, ROWS as i32 - 1) as usize, ny.clamp(0, COLUMNS as i32 - 1) as usize)  )
                    }
                };
                (cords, None)
            }
            ShipAction::Nothing => (Some(cords), None),
        }
    }
}

#[derive(Clone)]
enum AIAction {
    Nothing,
    Remove,
    Shoot,
    RelativeMove((i8, i8)),
    MoveOrNothing((i8, i8)),
    ShootOrNothing,
}
impl AIAction {
    pub fn to_ship_action(self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        match self {
            AIAction::Nothing => ShipAction::Nothing,
            AIAction::Remove => ShipAction::Remove,
            AIAction::Shoot => ShipAction::Shoot,
            AIAction::RelativeMove(rel_cords) => ShipAction::RelativeMove(rel_cords),
            AIAction::MoveOrNothing((dx, dy)) => {
                let new_cords = (cords.0 as i32 + dx as i32, cords.1 as i32 + dy as i32);
                let new_cords = (
                    new_cords.0.clamp(0, ROWS as i32 - 1) as usize,
                    new_cords.1.clamp(0, COLUMNS as i32 - 1) as usize,
                );
                if game_board.get(&new_cords).is_none() {
                    ShipAction::RelativeMove((dx, dy))
                } else {
                    ShipAction::Nothing 
                }
            }
            AIAction::ShootOrNothing => {
                let (dx, dy) = (1, 0); 
                let mut shoot_position = (cords.0 + dx, cords.1 + dy);

                if game_board.get(&shoot_position).is_none() {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }
        }
    }
}

enum Ship {
    Fly(ShipAI, bool, Uuid),
    Explosion(ShipAI, bool, Uuid),
    Bullet(ShipAI, bool, Uuid),
}

impl Ship {
    pub fn display_char(&self) -> char {
        match self {
            Ship::Fly(_, _, _) => 'F',
            Ship::Explosion(_, _, _) => 'X',
            Ship::Bullet(_, _, _) => '|',
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Ship::Fly(_, _, id) => *id,
            Ship::Explosion(_, _, id) => *id,
            Ship::Bullet(_, _, id) => *id,
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &mut HashMap<Cords, Ship>) -> ShipAction {
        match self {
            Ship::Fly(ai, _, _) => ai.get_action(cords, game_board),
            Ship::Explosion(ai, _, _) => ai.get_action(cords, game_board),
            Ship::Bullet(ai, _, _) => ai.get_action(cords, game_board),
        }
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly(_, wrap, _) => *wrap,
            Ship::Explosion(_, wrap, _) => *wrap,
            Ship::Bullet(_, wrap, _) => *wrap,
        }
    }

    pub fn new_fly() -> Self {
            Self::Fly(
             ShipAI::new(
                100, 
                vec![
                    AIAction::MoveOrNothing((1, -1)),
                    AIAction::ShootOrNothing,
                    AIAction::MoveOrNothing((-1, -1)),
                    AIAction::ShootOrNothing,
                ]
            ), 
            true, 
            Uuid::new_v4(),
        )
    }
    
    pub fn new_bullet(moving_down: bool) -> Self {
        let movement = if moving_down { (1, 0) } else { (-1, 0) };
        Self::Bullet(
            ShipAI::new(
                10, 
                vec![AIAction::RelativeMove(movement)]
            ), 
            false, 
            Uuid::new_v4(),
        )
    }

    pub fn new_explosion() -> Self {
        Self::Explosion(
            ShipAI::new(
                10, 
                vec![AIAction::Remove]
            ), 
            false, 
            Uuid::new_v4(),
        )
    }
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
    actions: Vec<AIAction>,
    action_index: usize,
}
/////todo add new ai action move or nothing, that checks if there is somthing where its going to move do nothing other wise move,
/////todo: change fly to use move or nothing instead of reltive move
impl ShipAI {
    fn new(action_interval: u64, actions: Vec<AIAction>) -> Self {
        ShipAI {
            timer: Timer::new(action_interval),
            actions,
            action_index: 0,
        }
    }

    fn get_ai_action(&mut self, _cords: Cords, _game_board: &HashMap<Cords, Ship>) -> AIAction {
        if self.actions.is_empty() {
            return AIAction::Nothing;
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
            AIAction::Nothing
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &mut HashMap<Cords, Ship>,) -> ShipAction {
        self.get_ai_action(cords, game_board).to_ship_action(cords, game_board)
    }
}


struct Player {
    display_char: char,
    lives: u64,
    current_position: Option<Cords>,
    start_position: Cords,
    death_timer: Timer, 
}

impl Player {
    fn new() -> Self {
        let start_position = (ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^',
            lives: 4,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(0), 
        }
    }

    fn move_to(&mut self, new_position: Option<Cords>) {
        self.current_position = new_position;
    }

    fn start_death_timer(&mut self, interval: u64) {
        self.death_timer = Timer::new(interval); 
    }

    fn tick(&mut self) -> bool {
        self.death_timer.tick()
    }

    fn handle_collision(&mut self) {
        self.lives -= 1;

        if self.lives == 0 {
            println!("You lost");
            exit(0);
        } else {
            println!("You got hit, Lives remaining: {}", self.lives);

            self.current_position = None;
            self.start_death_timer(200); 
        }
    }

    fn respawn(&mut self, game_board: &HashMap<Cords, Ship>) {
        if self.current_position.is_none() && self.tick() {
            if game_board.get(&self.start_position).is_none() {
                self.move_to(Some(self.start_position));
            } else {
                self.start_death_timer(100); 
            }
        }
    }
}

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
        //Command::new("clear").status().ok();

        execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0));
        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");
    
        for row in 0..ROWS {
            print!("           |"); 
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
            println!("|           ");
        }
    
        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");
    }

    async fn use_key(&mut self) {
        if let Some((x, y)) = self.player.current_position {
            let new_cords = match self.key_reader.read_key().await {
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

            if let Some(new_cords) = new_cords {
                self.player.move_to(Some(new_cords));
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

    fn ship_actions(&mut self) -> Result<(), String> {
        let to_update = self.game_board.iter().map(|(cords, ship)|
            (*cords, ship.get_id())
        ).collect::<Vec<(Cords, Uuid)>>();
    
        for (cords, shipid) in to_update {
            if let Some(mut current_ship) = self.game_board.remove(&cords) {
                if current_ship.get_id() != shipid {
                    continue;
                }
    
                let action = current_ship.get_action(cords, &mut self.game_board);
                let wrap = current_ship.wrap();
                let (opt_current_pos, opt_game_action): (Option<Cords>, Option<GameAction>) = action.to_game_action(&current_ship, cords);

                if let Some(current_pos) = opt_current_pos {
                    self.add_ship(current_pos, current_ship);
                }
                
                if let Some(game_action) = opt_game_action {
                    self.update(game_action)?;
                }
            }
        }
        Ok(())
    }

    fn update(&mut self, action: GameAction) -> Result<(), String> {
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
        Ok(())
    }

    fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            if self.game_board.get(&player_pos).is_some() {
                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());
                self.player.handle_collision();
            }
        }

        self.player.respawn(&self.game_board);
    }

    async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10));
            self.tick_count += 1;

            self.ship_actions();
            self.player_actions();

            self.display_board();
            self.use_key().await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut game = GameState::new();
    game.add_ship((2, 2), Ship::new_fly())?;
    game.add_ship((3, 3), Ship::new_fly())?;
    game.add_ship((4, 3), Ship::new_fly())?;
    game.start_game().await?;
    Ok(())
}

