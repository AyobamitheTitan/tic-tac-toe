use crate::animation::*;
use crate::components::*;
use crate::constants::*;
use crate::observers::*;
use crate::resources::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resize_constraints: WindowResizeConstraints {
                    min_width: 500.,
                    min_height: 500.,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        // NOTE - X to start by default
        // Potential improvement can be giving the user the option to select who starts
        .insert_resource(PlayerTurn(Turn::X))
        .insert_resource(GameOver(false))
        .add_systems(Startup, Self::spawn_game)
        .add_observer(select_cell)
        .add_observer(hover_cell)
        .add_observer(hover_out_cell)
        .add_observer(game_won)
        .add_observer(detect_win);
        return;
    }
}

impl TicTacToePlugin {
    fn spawn_game(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
        let mut entity_commands = Self::draw_board(&mut commands, window_query);
        Self::draw_cells(window_query, &mut entity_commands);
    }

    fn draw_cells<'b>(
        window_query: Query<&Window, With<PrimaryWindow>>,

        board_command: &mut EntityCommands<'b>,
    ) {
        if let Ok(window) = window_query.single() {
            let window_width = window.width() / 2.7;
            let window_height = window.height() / 2.0;

            board_command.with_children(|parent| {
                for i in 0..9 {
                    let _cell = parent.spawn((
                        Node {
                            width: Val::Px(window_width / CELL_SIZE_DIVISOR),
                            height: Val::Px(window_height / CELL_SIZE_DIVISOR),
                            // align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            border_radius: BorderRadius::all(Val::Px(BOARD_BORDER_RADIUS)),
                            ..Default::default()
                        },
                        // BoxShadow(vec![ShadowStyle {
                        //     color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                        //     x_offset: Val::Px(4.0),
                        //     y_offset: Val::Px(4.0),
                        //     spread_radius: Val::Px(2.0),
                        //     blur_radius: Val::Px(4.0),
                        // }]),
                        Transform::from_xyz(0., 0., 0.),
                        BackgroundColor::from(Color::from(
                            Srgba::hex(CELL_COLOR).unwrap_or(Srgba::WHITE),
                        )),
                        Cell::from_position(CellPosition::from_u8(i)),
                    ));
                }
            });
        }
    }

    fn draw_board<'b>(
        commands: &'b mut Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) -> EntityCommands<'b> {
        match window_query.single() {
            Ok(window) => {
                let window_width = window.width() / 2.7;
                let window_height = window.height() / 2.0;

                commands.spawn(Camera2d);
                return commands.spawn((
                    Node {
                        width: Val::Px(window_width),
                        height: Val::Px(window_height),
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        display: Display::Grid,
                        row_gap: Val::Px(CELL_GAP),
                        column_gap: Val::Px(CELL_GAP),
                        grid_template_columns: RepeatedGridTrack::auto(3),
                        grid_template_rows: RepeatedGridTrack::auto(3),
                        border_radius: BorderRadius::all(Val::Px(BOARD_BORDER_RADIUS)),
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 0.),
                    BackgroundColor::from(Color::from(
                        Srgba::hex(BOARD_COLOR).unwrap_or(Srgba::WHITE),
                    )),
                ));
            }
            Err(_) => todo!(),
        }
    }
}
