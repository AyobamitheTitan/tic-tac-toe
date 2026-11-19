use bevy::ecs::resource::Resource;

#[derive(Resource, Debug)]
pub struct PlayerTurn(pub Turn);

#[derive(Resource)]
pub struct GameOver(pub bool);

#[derive(Debug, PartialEq, Eq)]
pub enum Turn {
    X,
    O,
}
