use uuid::Uuid;
use std::collections::HashMap;
use rand::Rng;
use crate::fly_patterns::Fly_Pattern;
use crate::settings::GameSettings;
use crate::structs::{Cords, Timer, ShipAction, RelCords};

trait ship {
    fn display_info(&self) -> String;
    fn get_id(&self) -> Uuid;
    fn get_action(&mut self, cords: Cords, game_board: &mut HashMap<Cords, Ship>) -> ShipAction;
    fn wrap(&self) -> bool;
}

pub enum Ship {
    Fly(ShipAI, bool, Uuid, u32, bool, bool), // Add `allow_shoot` as the last field
    Explosion(ShipAI, bool, Uuid),
    Laser(ShipAI, bool, Uuid, bool, u32), // Add speed as the fifth field for bullets
}

impl Ship {
    pub fn is_fly(&self) -> bool {
        matches!(self, Ship::Fly(_, _, _, _, _, _))
    }

    pub fn display_info(&self) -> String {
        match self {
            Ship::Fly(_, _, _, _, _, _) => "assets/fly.png".to_string(),
            Ship::Explosion(_, _, _) => "assets/explosion.png".to_string(),
            Ship::Laser(_, _, _, moving_down, _) => {
                if *moving_down {
                    "assets/bullet_downward.png".to_string()
                } else {
                    "assets/bullet_upward.png".to_string()
                }
            }
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Ship::Fly(_, _, id, _, _, _) => *id,
            Ship::Explosion(_, _, id) => *id,
            Ship::Laser(_, _, id, _, _) => *id,
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &mut HashMap<Cords, Ship>) -> ShipAction {
        match self {
            Ship::Fly(ai, _, _, _, _, _) => ai.get_action(cords, game_board),
            Ship::Explosion(ai, _, _) => ai.get_action(cords, game_board),
            Ship::Laser(ai, _, _, _, _) => ai.get_action(cords, game_board),
        }
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly(_, wrap, _, _, _, _) => *wrap,
            Ship::Explosion(_, wrap, _) => *wrap,
            Ship::Laser(_, wrap, _, _, _) => *wrap,
        }
    }

pub fn new_fly(speed: u32, wrap: bool, allow_shoot: bool) -> Self {
        let mut rng = rand::thread_rng();
        let random_pattern = match rng.gen_range(0..3) {
            0 => Fly_Pattern::Pattern1,
            1 => Fly_Pattern::Pattern2,
            _ => Fly_Pattern::Pattern3,
        };

        let mut actions = random_pattern.fly_pattern();
        if !wrap {
            actions = actions
                .into_iter()
                .map(|(cond, action)| match action {
                    AIAction::MoveOrNothing(_) => (cond, AIAction::Nothing),
                    other => (cond, other),
                })
                .collect();
        }
        if !allow_shoot {
            actions = actions
                .into_iter()
                .map(|(cond, action)| match action {
                    AIAction::ShootOrNothing => (cond, AIAction::Nothing),
                    other => (cond, other),
                })
                .collect();
        }

        Self::Fly(ShipAI::new(speed as u64, actions), wrap, Uuid::new_v4(), 0, allow_shoot, true)
    }

    pub fn new_bullet(moving_down: bool, speed: u32) -> Self {
        let movement = if moving_down { RelCords(1, 0) } else { RelCords(-1, 0) };

        Self::Laser(
            ShipAI::new(
                speed as u64,
                vec![(None, AIAction::RelativeMove(movement))],
            ),
            false,
            Uuid::new_v4(),
            moving_down,
            speed, // Store speed
        )
    }

    pub fn new_explosion() -> Self {
        Self::Explosion(
            ShipAI::new(
                20,
                vec![(None, AIAction::Remove)],
            ),
            false,
            Uuid::new_v4(),
        )
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

    pub fn get_ai_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> AIAction {
        if self.actions.is_empty() {
            return AIAction::Nothing;
        }

        if self.timer.tick() {
            let (condition, action) = &self.actions[self.action_index];

            if let Some(condition) = condition {
                if !condition.evaluate(cords, game_board) {
                    self.next_action();
                    return self.get_ai_action(cords, game_board);
                }
            }

            if self.action_index == self.actions.len() - 1 {
                self.action_index = 0;
            } else {
                self.action_index += 1;
            }

            return action.clone();
        }

        AIAction::Nothing
    }

    fn next_action(&mut self) {
        if self.action_index == self.actions.len() - 1 {
            self.action_index = 0;
        } else {
            self.action_index += 1;
        }
    }

    pub fn get_action(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Ship>,
    ) -> ShipAction {
        self.get_ai_action(cords, game_board).to_ship_action(cords, game_board)
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    DontShootIfShipsAreBelow(RelCords),
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
            Condition::DontShootIfShipsAreBelow(_) => {
                let mut below_cords = cords;
                loop {
                    if !game_board.contains_key(&below_cords) {
                        break;
                    }

                    if let Some(ship) = game_board.get(&below_cords) {
                        if let Ship::Fly(_, _, _, _, _, _) = ship {
                            return false;
                        }
                    }
                    below_cords.0 += 1;
                }
                true
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
                let condition = Condition::DontShootIfShipsAreBelow(RelCords(1, 0));
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