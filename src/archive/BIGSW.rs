use std::collections::HashMap;
use crate::structs::{ROWS, COLUMNS, Cords, Timer, ShipAction};


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
