use uuid::Uuid;
use std::collections::HashMap;
use crate::structs::{ROWS, COLUMNS, Cords, Timer};
use crate::structs::ShipAction;

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

    pub fn get_ai_action(
        &mut self,
        _cords: Cords,
        _game_board: &HashMap<Cords, Ship>,
    ) -> AIAction {
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
        self.get_ai_action(cords, game_board)
            .to_ship_action(cords, game_board)
    }
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
#[derive(Clone, Debug)]
pub struct RelCords(i8, i8);

impl RelCords {
    pub fn evaluate(&self, cords: Cords) -> (Cords, bool) {
        let new_cords = (
            (cords.0 as i32 + self.0 as i32),
            (cords.1 as i32 + self.1 as i32),
        );
        
        let mut wrapped = false;

        let new_cords = (
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

impl AIAction {
    pub fn to_ship_action(self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        match self {
            AIAction::Nothing => ShipAction::Nothing,
            AIAction::Remove => ShipAction::Remove,
            AIAction::Shoot => ShipAction::Shoot,
            AIAction::RelativeMove(rel_cords) => {
                let rel_cords = RelCords(rel_cords.0, rel_cords.1);
                let (new_cords, wrapped) = rel_cords.evaluate(cords);

                if wrapped || game_board.get(&new_cords).is_some() {
                    ShipAction::Nothing
                } else {
                    ShipAction::Move(new_cords)
                }
            }
            AIAction::MoveOrNothing((dx, dy)) => {
                let rel_cords = RelCords(dx, dy);
                let (new_cords, wrapped) = rel_cords.evaluate(cords);

                if !wrapped && game_board.get(&new_cords).is_none() {
                    ShipAction::Move(new_cords)
                } else {
                    ShipAction::Nothing
                }
            }
            AIAction::ShootOrNothing => {
                let (dx, dy) = (1, 0); 
                let rel_cords = RelCords(dx, dy);
                let (shoot_position, _) = rel_cords.evaluate(cords);

                if game_board.get(&shoot_position).is_none() {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }
        }
    }
}


//TODO: create a new structure called RelCords has to i8 i8s and a method called evaluate. Provide evaluate with Cords and it responds with the relitive position based on the cords and a bool if the cords had to wrap

//pub enum Condition {
//    ShipExists(Cords),
//}

//TODO: Move the logic from MoveOrNothing into an impl for Condition where condition has an evaluate method that accepts cords and gamestate and that returns a bool if condition true