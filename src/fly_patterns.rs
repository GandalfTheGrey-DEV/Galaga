use crate::ship::{AIAction, Condition};
use crate::structs::RelCords;

#[derive(Debug)]
pub enum Fly_Pattern {
    Pattern1,
    Pattern2,
    Pattern3,
    Pattern4,
    Pattern5,
    Pattern6,
    Pattern7,
    Pattern8,
    Pattern9,
    Pattern10,
}

impl Fly_Pattern {
    pub fn fly_pattern(&self) -> Vec<(Option<Condition>, AIAction)> {
        match self {
            Fly_Pattern::Pattern1 => vec![
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
            ],
            Fly_Pattern::Pattern2 => vec![
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
            ],
            Fly_Pattern::Pattern3 => vec![
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
            ],
            Fly_Pattern::Pattern4 => vec![
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
            ],
            Fly_Pattern::Pattern5 => vec![
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (None, AIAction::MoveOrNothing(RelCords(0, -1))),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
            ],
            Fly_Pattern::Pattern6 => vec![
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(1, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(-1, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(-1, 1))),
            ],
            Fly_Pattern::Pattern7 => vec![
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 1))), AIAction::Shoot),
            ],
            Fly_Pattern::Pattern8 => vec![
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(1, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(-1, 0))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(-1, 0))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(0, 1))),
            ],
            Fly_Pattern::Pattern9 => vec![
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(1, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(-1, 1))), AIAction::Shoot),
            ],
            Fly_Pattern::Pattern10 => vec![
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 1))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(1, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 2))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(-1, 1))),
                (Some(Condition::DontShootIfShipsAreBelow(RelCords(0, 2))), AIAction::Shoot),
                (None, AIAction::MoveOrNothing(RelCords(1, 1))),
            ],
        }
    }
}
