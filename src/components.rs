use bevy::ecs::component::Component;

use crate::resources::Turn;

#[derive(Component, Debug, Eq, Hash, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CellPosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    NotCell,
}

#[derive(Component, Debug)]
pub struct Cell {
    pub position: CellPosition,
    pub val: Option<Turn>,
    pub locked: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            position: CellPosition::NotCell,
            val: None,
            locked: false,
        }
    }
}

impl Cell {
    pub fn from_position(position: CellPosition) -> Self {
        let mut default_cell = Self::default();
        default_cell.position = position;
        default_cell
    }
}

impl CellPosition {
    pub fn from_u8(u8_representation: u8) -> Self {
        match u8_representation {
            0 => Self::TopLeft,
            1 => Self::TopCenter,
            2 => Self::TopRight,
            3 => Self::MiddleLeft,
            4 => Self::MiddleCenter,
            5 => Self::MiddleRight,
            6 => Self::BottomLeft,
            7 => Self::BottomCenter,
            8 => Self::BottomRight,
            _ => Self::NotCell,
        }
    }

    pub fn winning_positions() -> [[Self; 3]; 8] {
        return [
            [Self::TopLeft, Self::TopCenter, Self::TopRight],
            [Self::TopLeft, Self::MiddleLeft, Self::BottomLeft],
            [Self::TopLeft, Self::MiddleCenter, Self::BottomRight],
            [Self::MiddleLeft, Self::MiddleCenter, Self::MiddleRight],
            [Self::BottomLeft, Self::BottomCenter, Self::BottomRight],
            [Self::TopCenter, Self::MiddleCenter, Self::BottomCenter],
            [Self::TopRight, Self::MiddleRight, Self::BottomRight],
            [Self::TopRight, Self::MiddleCenter, Self::BottomLeft],
        ];
    }

    pub fn to_str<'a>(self) -> &'a str {
        match self {
            CellPosition::TopLeft => "TopLeft",
            CellPosition::TopCenter => "TopCenter",
            CellPosition::TopRight => "TopRight",
            CellPosition::MiddleLeft => "MiddleLeft",
            CellPosition::MiddleCenter => "MiddleCenter",
            CellPosition::MiddleRight => "MiddleRight",
            CellPosition::BottomLeft => "BottomLeft",
            CellPosition::BottomCenter => "BottomCenter",
            CellPosition::BottomRight => "BottomRight",
            CellPosition::NotCell => "NotCell",
        }
    }
}
