
.##....##.########.##......##....................######..########..######..########.####..#######..##....##
.###...##.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.###...##
.####..##.##.......##..##..##...................##.......##.......##..........##.....##..##.....##.####..##
.##.##.##.######...##..##..##....................######..######...##..........##.....##..##.....##.##.##.##
.##..####.##.......##..##..##.........................##.##.......##..........##.....##..##.....##.##..####
.##...###.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.##...###
.##....##.########..###..###.....................######..########..######.....##....####..#######..##....##


//pub enum Condition {
//    ShipExists(Cords),
//}

//TODO: Move the logic from MoveOrNothing into an impl for Condition where condition has an evaluate method that accepts cords and gamestate and that returns a bool if condition true


enum Condition {
    Postiion_free: bool; //if postion is free then do somthing
    ShipExists(Cords); //if ship exists do somthing
    Shoot_clear: bool; //if no bullet is beneth ship bool true
}

//impl Condition(self, cords: Cords, game_board: &HashMap<Cords, Ship>) {
    //TODO: Check if next move for Ship is Free.
    //TODO: Check if Ship exists.
    //TODO: Check if Ship can fire bullet with out hitting another ship.
//}

impl Condition(self, cords: Cords, game_board: &HashMap<Cords, Ship>) {
    //TODO: break down, Postion_free
    //TODO: get ship postion and next move: CORDS NEXTMOVE 
        //TODO: get cords of next move for that ship
        //TODO: IF next move CORDS have Ship return NONE/FALSE
        
    //TODO: break down, shoot_clear
    //TODO: get ship and check if any ships are beneth it
      //TODO: if ship is beneth ship then instead of returing optinal bullet cords return none/nothing/FALSE
}

pub enum Condition {

}

impl Condition(self, cords: Cords, game_board: &HashMap<Cords, Ship>) {

}

#[derive(Clone)]
pub enum AIAction {
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
            AIAction::Remove => {
                ShipAction::Remove
            }

            AIAction::Shoot => {
                let shoot_position = (cords.0 + 1, cords.1);
                ShipAction::Shoot
            }

            AIAction::RelativeMove(rel_cords) => {
                let rel_cords = RelCords(rel_cords.0, rel_cords.1);
                let (new_cords, wrapped) = rel_cords.evaluate(cords);
                ShipAction::RelativeMove((rel_cords.0, rel_cords.1))
            }

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
                let shoot_position = (cords.0 + dx, cords.1 + dy);

                if game_board.get(&shoot_position).is_none() {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::Nothing => {
                ShipAction::Nothing
            }
        }
    }
}

.##....##.########.##......##....................######..########..######..########.####..#######..##....##
.###...##.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.###...##
.####..##.##.......##..##..##...................##.......##.......##..........##.....##..##.....##.####..##
.##.##.##.######...##..##..##....................######..######...##..........##.....##..##.....##.##.##.##
.##..####.##.......##..##..##.........................##.##.......##..........##.....##..##.....##.##..####
.##...###.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.##...###
.##....##.########..###..###.....................######..########..######.....##....####..#######..##....##



🚧
//TODO a way to set a level and recive data about that level?
//! thinking enum is gonna work for this?
#[derive(Clone, Debug)]
pub enum Level {
    Easy,
    Medium,
    Hard,
}
//TODO creating a struct to hold current level 
#[derive(Debug)]
pub struct GameLevel {
    current_level: Level,
}
//TODO create irl for gamelevel that takes the enum?
🚧
impl GameLevel {
    //TODO set default level
    pub fn new(level: Level) -> Self {
        GameLevel { current_level: level }
    }
    //TODO a way to update level!!!???
    pub fn set_level(&mut self, level: Level) {
        self.current_level = level;
    }
    //TODO create a way to get level status
    pub fn get_level_status(&self) -> (i32, i32) {
        match self.current_level {
            Level::Easy => Self::easy(),
            Level::Medium => Self::medium(),
            Level::Hard => Self::hard(),
        }
    }
    //TODO: implment those levels
   // ✅
    fn easy() -> (i32, i32) {
        let speed = 500; 
        let lives = 5;
        (speed, lives)
    }
   // ✅
    fn medium() -> (i32, i32) {
        let speed = 300;
        let lives = 3;
        (speed, lives)
    }
   // ✅
    fn hard() -> (i32, i32) {
        let speed = 100; 
        let lives = 1;
        (speed, lives)
    }
}

//!! Exampls first one set the level

    let mut game_level = GameLevel::new(Level::Medium); 
    println!("level default: {:?}", game_level.get_level_status());
//!! update level
    game_level.set_level(Level::Hard);
    println!("updated level: {:?}", game_level.get_level_status());


    //TODO add method what is the game level set to. might have to refactor to have it a bool so if easy is set to true then i can call it in another file or another spot ask it hey whats the level and it returns the speed and lives
    #[derive(Clone, Debug)]
    pub enum Level {
        Easy,
        Medium,
        Hard,
    }

    #[derive(Debug)]
    pub struct GameLevel {
        current_level: Level,
    }

    impl GameLevel {
        pub fn new(level: Level) -> Self {
            GameLevel { current_level: level }
        }

        pub fn set_level(&mut self, level: Level) {
            self.current_level = level;
        }

        pub fn get_level_status(&self) -> (i32, i32) {
            match self.current_level {
                Level::Easy => Self::easy(),
                Level::Medium => Self::medium(),
                Level::Hard => Self::hard(),
            }
        }

        fn easy() -> (i32, i32) {
            let speed = 500; 
            let lives = 5;
            (speed, lives)
        }

        fn medium() -> (i32, i32) {
            let speed = 300;
            let lives = 3;
            (speed, lives)
        }

        fn hard() -> (i32, i32) {
            let speed = 100; 
            let lives = 1;
            (speed, lives)
        }
    }
    

    .##....##.########.##......##....................######..########..######..########.####..#######..##....##
    .###...##.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.###...##
    .####..##.##.......##..##..##...................##.......##.......##..........##.....##..##.....##.####..##
    .##.##.##.######...##..##..##....................######..######...##..........##.....##..##.....##.##.##.##
    .##..####.##.......##..##..##.........................##.##.......##..........##.....##..##.....##.##..####
    .##...###.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.##...###
    .##....##.########..###..###.....................######..########..######.....##....####..#######..##....##
    
    

    impl Ship {
        
        pub fn get_status(&self) -> String {
            match self {
                Ship::Fly(ai, wrap, id) => format!(
                    "Type: Fly, Wrap: {}, ID: {}, AI Info: {:?}",
                    wrap, id, ai
                ),
                Ship::Explosion(ai, wrap, id) => format!(
                    "Type: Explosion, Wrap: {}, ID: {}, AI Info: {:?}",
                    wrap, id, ai
                ),
                Ship::Bullet(ai, wrap, id) => format!(
                    "Type: Bullet, Wrap: {}, ID: {}, AI Info: {:?}",
                    wrap, id, ai
                ),
            }
        }
    }
    
    //let fly = Ship::new_fly();
    let bullet = Ship::new_bullet(true);
    let explosion = Ship::new_explosion();
    
    println!("{}", fly.get_status());
    println!("{}", bullet.get_status());
    println!("{}", explosion.get_status());
    

pub enum Ship {
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

    pub fn new_fly(game_level: &GameLevel) -> Self {
        let (speed, _lives) = game_level.get_level_status(); 
        Self::Fly(
            ShipAI::new(
                speed,
                vec![
                    AIAction::MoveOrNothing(RelCords(1, -1)),
                    AIAction::ShootOrNothing,
                    AIAction::MoveOrNothing(RelCords(-1, -1)),
                    AIAction::ShootOrNothing,
                ]
            ),
            true,
            Uuid::new_v4(),
        )
    }
    
    pub fn new_bullet(moving_down: bool, game_level: &GameLevel) -> Self {
        let (speed, _) = game_level.get_level_status();
        let movement = if moving_down { RelCords(1, 0) } else { RelCords(-1, 0) };
        Self::Bullet(
            ShipAI::new(
                speed, 
                vec![AIAction::RelativeMove(movement)],
            ),
            false,
            Uuid::new_v4(),
        )
    }

    pub fn new_explosion(game_level: &GameLevel) -> Self {
        let (_speed, lives) = game_level.get_level_status();
    
        Self::Explosion(
            ShipAI::new(
                10, 
                vec![AIAction::Remove],
            ), 
            false, 
            Uuid::new_v4(),
        )
    }
}

.##....##.########.##......##....................######..########..######..########.####..#######..##....##
.###...##.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.###...##
.####..##.##.......##..##..##...................##.......##.......##..........##.....##..##.....##.####..##
.##.##.##.######...##..##..##....................######..######...##..........##.....##..##.....##.##.##.##
.##..####.##.......##..##..##.........................##.##.......##..........##.....##..##.....##.##..####
.##...###.##.......##..##..##...................##....##.##.......##....##....##.....##..##.....##.##...###
.##....##.########..###..###.....................######..########..######.....##....####..#######..##....##





Ai command description: actions:
[
    [(Option<Condition>, AIAction), ...] = [(Some(Condition::NoFriendlyShipsBelowMe(cords)), AIAction::shoot), (None, AIAction::move_up)] //If the end of the array was reached and no action found do Action::Nothing
]

//[(Option<Condition>, AIAction), ...] = 




use uuid::Uuid;
use std::collections::HashMap;
use crate::structs::{Cords, Timer, ShipAction, RelCords, GameLevel};
       // pramitor game_level: &GameLevel
       // let (speed, _lives) = game_level.get_level_status(); 
pub enum Ship {
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
                    //TODO this how we want to redo theses actions: a touple optinal conditon
                    [(Some(Condition::NoFriendlyShipsBelowMe(cords)), AIAction::shoot), (None, AIAction::move_up)] //If the end of the array was reached and no action found do Action::Nothing
              //THESE are the old actions keep the same movement pattern but we need to use the example above 
              //OLD      AIAction::MoveOrNothing(RelCords(1, -1)),
              //OLD      AIAction::ShootOrNothing,
              //OLD      AIAction::MoveOrNothing(RelCords(-1, -1)),
              //OLD      AIAction::ShootOrNothing,
                ]
            ),
            true,
            Uuid::new_v4(),
        )
    }

    pub fn new_bullet(moving_down: bool) -> Self {
        let movement = if moving_down { RelCords(1, 0) } else { RelCords(-1, 0) };
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


pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<AIAction>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<AIAction>) -> Self {
        ShipAI {
            timer: Timer::new(action_interval),
            actions,
            action_index: 0,
        }
    }

    pub fn get_ai_action(&mut self) -> AIAction {
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

    pub fn get_action(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Ship>,
    ) -> ShipAction {
        self.get_ai_action().to_ship_action(cords, game_board)
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    NoFriendlyShipsBelowMe(RelCords),
}

impl Condition {
    pub fn evaluate(&self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> bool {
        match self {
            Condition::ShipExists(ref target_cords) => {
                game_board.contains_key(target_cords) 
            }
            Condition::PositionAvailable(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none() 
            }
            Condition::NoFriendlyShipsBelowMe(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none() 
            }
        }
    }
}

#[derive(Clone)]
pub enum AIAction {
    Nothing,
    Remove,
    Shoot,
    Move(Cords),
    MoveOrNothing(RelCords),
    ShootOrNothing,
    RelativeMove(RelCords),
}

impl AIAction {
    pub fn to_ship_action(self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        match self {
            AIAction::Remove => {
                ShipAction::Remove
            }

            AIAction::Shoot => {
                ShipAction::Shoot
            }

            AIAction::Move(cords) => {
                ShipAction::Move(cords, false)
            }

            AIAction::MoveOrNothing(rel_cords) => {
                let condition = Condition::PositionAvailable(rel_cords.clone());
                if condition.evaluate(cords, game_board) {
                    let (new_cords, wrap) = rel_cords.evaluate(cords);
                    ShipAction::Move(new_cords, wrap)
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrapped) = rel_cords.evaluate(cords);
                ShipAction::Move(new_cords, wrapped)
            }

            AIAction::ShootOrNothing => {
                let condition = Condition::NoFriendlyShipsBelowMe(RelCords(1, 0)); 
                if condition.evaluate(cords, game_board) {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::Nothing => {
                ShipAction::Nothing
            }
        }
    }
}

use uuid::Uuid;
use std::collections::HashMap;
use crate::structs::{Cords, Timer, ShipAction, RelCords, GameLevel};

pub enum Ship {
    Fly((ShipAI, bool, Uuid)),
    Explosion((ShipAI, bool, Uuid)),
    Bullet((ShipAI, bool, Uuid)),
}

impl Ship {
    pub fn display_char(&self) -> char {
        match self {
            Ship::Fly(_) => 'F',
            Ship::Explosion(_) => 'X',
            Ship::Bullet(_) => '|',
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Ship::Fly((_, _, id)) => *id,
            Ship::Explosion((_, _, id)) => *id,
            Ship::Bullet((_, _, id)) => *id,
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &mut HashMap<Cords, Ship>) -> ShipAction {
        match self {
            Ship::Fly((ai, _, _)) => ai.get_action(cords, game_board),
            Ship::Explosion((ai, _, _)) => ai.get_action(cords, game_board),
            Ship::Bullet((ai, _, _)) => ai.get_action(cords, game_board),
        }
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly((_, wrap, _)) => *wrap,
            Ship::Explosion((_, wrap, _)) => *wrap,
            Ship::Bullet((_, wrap, _)) => *wrap,
        }
    }

    pub fn new_fly() -> Self {
        Self::Fly((
            ShipAI::new(
                100,
                vec![
                    (
                        Some(Condition::NoFriendlyShipsBelowMe(RelCords(0, -1))),
                        AIAction::Shoot,
                    ),
                    (None, AIAction::MoveOrNothing(RelCords(-1, 1))),
                    (None, AIAction::MoveOrNothing(RelCords(-1, -1))),
                    (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                ],
            ),
            true,
            Uuid::new_v4(),
        ))
    }

    pub fn new_bullet(moving_down: bool) -> Self {
        let movement = if moving_down { RelCords(1, 0) } else { RelCords(-1, 0) };
        Self::Bullet((
            ShipAI::new(
                10,
                vec![(None, AIAction::RelativeMove(movement))],
            ),
            false,
            Uuid::new_v4(),
        ))
    }

    pub fn new_explosion() -> Self {
        Self::Explosion((
            ShipAI::new(
                10,
                vec![(None, AIAction::Remove)],
            ),
            false,
            Uuid::new_v4(),
        ))
    }
}

pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<(Option<Condition>, AIAction)>) -> Self {
        ShipAI {
            timer: Timer::new(action_interval),
            actions,
            action_index: 0,
        }
    }

    pub fn get_ai_action(&mut self) -> AIAction {
        if self.actions.is_empty() {
            return AIAction::Nothing;
        }

        if self.timer.tick() {
            let action = self.actions[self.action_index].1.clone(); 
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

    pub fn get_action(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Ship>,
    ) -> ShipAction {
        self.get_ai_action().to_ship_action(cords, game_board)
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    NoFriendlyShipsBelowMe(RelCords),
}

impl Condition {
    pub fn evaluate(&self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> bool {
        match self {
            Condition::ShipExists(ref target_cords) => game_board.contains_key(target_cords),
            Condition::PositionAvailable(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none()
            }
            Condition::NoFriendlyShipsBelowMe(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none()
            }
        }
    }
}

#[derive(Clone)]
pub enum AIAction {
    Nothing,
    Remove,
    Shoot,
    Move(Cords),
    MoveOrNothing(RelCords),
    ShootOrNothing,
    RelativeMove(RelCords),
}

impl AIAction {
    pub fn to_ship_action(self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        match self {
            AIAction::Remove => ShipAction::Remove,
            AIAction::Shoot => ShipAction::Shoot,
            AIAction::Move(cords) => ShipAction::Move(cords, false),
            AIAction::MoveOrNothing(rel_cords) => {
                let condition = Condition::PositionAvailable(rel_cords.clone());
                if condition.evaluate(cords, game_board) {
                    let (new_cords, wrap) = rel_cords.evaluate(cords);
                    ShipAction::Move(new_cords, wrap)
                } else {
                    ShipAction::Nothing
                }
            }
            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrapped) = rel_cords.evaluate(cords);
                ShipAction::Move(new_cords, wrapped)
            }
            AIAction::ShootOrNothing => {
                let condition = Condition::NoFriendlyShipsBelowMe(RelCords(1, 0));
                if condition.evaluate(cords, game_board) {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }
            AIAction::Nothing => ShipAction::Nothing,
        }
    }
}
