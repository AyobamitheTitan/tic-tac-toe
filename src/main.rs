mod animation;
mod components;
mod constants;
mod event;
mod observers;
mod plugin;
mod resources;

use bevy::prelude::*;

use crate::plugin::TicTacToePlugin;

fn main() {
    // TODO - Is this something to worry about
    // Skipped event Destroyed for unknown winit Window Id WindowId(140295003827920)
    App::new().add_plugins(TicTacToePlugin).run();
}
