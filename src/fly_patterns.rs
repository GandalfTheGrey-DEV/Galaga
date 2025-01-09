use crate::ship::{AIAction, Condition};
use crate::structs::RelCords;

#[derive(Debug)]
pub enum Fly_Pattern {
    Pattern1,
    Pattern2,
    Pattern3,
}

impl Fly_Pattern {
    pub fn fly_pattern(&self) -> Vec<(Option<Condition>, AIAction)> {
        match self {
            Fly_Pattern::Pattern1 => vec![
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot),
            ],
            Fly_Pattern::Pattern2 => vec![
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot),
            ],
            Fly_Pattern::Pattern3 => vec![
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot),
            ],
        }
    }
}
