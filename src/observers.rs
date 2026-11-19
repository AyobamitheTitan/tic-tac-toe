use bevy::prelude::*;

use crate::{components::Cell, constants::*, resources::*};
pub fn hover_cell(
    hover: On<Pointer<Over>>,
    player_turn: ResMut<PlayerTurn>,
    game_over_res: Res<GameOver>,
    mut query: Query<(&Cell, &mut BackgroundColor)>,
) {
    if game_over_res.0 {
        return;
    }
    let hovered_cell = hover.entity;

    if let Ok((cell, mut bg_color)) = query.get_mut(hovered_cell) {
        match cell.val {
            // NOTE - Don't "hover" if it already has a value
            Some(_) => {}
            None => match player_turn.0 {
                Turn::X => {
                    bg_color.0 =
                        Color::from(Srgba::hex(X_TURN_HOVER_COLOR).unwrap_or(Srgba::WHITE));
                }
                Turn::O => {
                    bg_color.0 =
                        Color::from(Srgba::hex(O_TURN_HOVER_COLOR).unwrap_or(Srgba::WHITE));
                }
            },
        }
    }
}

pub fn hover_out_cell(hover: On<Pointer<Out>>, mut query: Query<(&Cell, &mut BackgroundColor)>) {
    if let Ok((_, mut bg_color)) = query.get_mut(hover.entity) {
        bg_color.0 = Color::from(Srgba::hex(CELL_COLOR).unwrap_or(Srgba::WHITE));
    }
}

pub fn select_cell(
    click: On<Pointer<Click>>,
    mut query: Query<(&Node, &mut Cell)>,
    mut player_turn: ResMut<PlayerTurn>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_res: Res<GameOver>,
) {
    if game_over_res.0 {
        return;
    }
    let clicked_cell = click.entity;

    // NOTE - Else case not needed, because the event is triggered only on a cell
    if let Ok((_, mut cell)) = query.get_mut(clicked_cell) {
        if cell.locked {
            return;
        }
        match player_turn.0 {
            Turn::X => {
                if let Ok(mut e) = commands.get_entity(clicked_cell) {
                    e.with_children(|parent| {
                        parent.spawn((ImageNode::new(asset_server.load("textures/x_vector.png")),));
                    });
                    cell.val = Some(Turn::X);
                    player_turn.0 = Turn::O;
                }
            }
            Turn::O => {
                if let Ok(mut e) = commands.get_entity(clicked_cell) {
                    e.with_children(|parent| {
                        parent.spawn((ImageNode::new(asset_server.load("textures/o_vector.png")),));
                    });
                    cell.val = Some(Turn::O);
                    player_turn.0 = Turn::X;
                }
            }
        }
        cell.locked = true;
    }
}
