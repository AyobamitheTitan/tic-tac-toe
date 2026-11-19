use bevy::ecs::event::Event;

use crate::{components::CellPosition, resources::Turn};

#[derive(Event)]
pub struct GameWon {
    pub turn: Turn,
    pub positions: [CellPosition; 3],
}
