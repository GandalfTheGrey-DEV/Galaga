pub const SIZE: usize = 10;
pub const ROWS: usize = SIZE;
pub const COLUMNS: usize = SIZE * 2;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Cords(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct RelCords(pub i32, pub i32);

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

    pub fn get_level_status(&self) -> (u64, u8) {
        match self.current_level {
            Level::Easy => Self::easy(),
            Level::Medium => Self::medium(),
            Level::Hard => Self::hard(),
        }
    }

    fn easy() -> (u64, u8) {
        let speed = 500; 
        let lives = 5;
        (speed, lives)
    }

    fn medium() -> (u64, u8) {
        let speed = 300;
        let lives = 3;
        (speed, lives)
    }

    fn hard() -> (u64, u8) {
        let speed = 100; 
        let lives = 1;
        (speed, lives)
    }
}


impl RelCords {
    pub fn evaluate(&self, cords: Cords) -> (Cords, bool) {
        
        let new_cords = (
            (cords.0 as i32 + self.0),
            (cords.1 as i32 + self.1),
        );
        
        let mut wrapped = false;
        let new_cords = Cords(
            if new_cords.0 < 0 {
                wrapped = true;
                ROWS as usize - 1
            } else if new_cords.0 >= ROWS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.0 as usize
            },
            if new_cords.1 < 0 {
                wrapped = true;
                COLUMNS as usize - 1
            } else if new_cords.1 >= COLUMNS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.1 as usize
            },
        );

        (new_cords, wrapped)
    }
}

#[derive(Clone)]
pub enum ShipAction {
    Nothing,
    Remove,
    Shoot,
    Move(Cords, bool),
}

pub struct Timer {
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