use bevy::{
    animation::{AnimatedBy, AnimationEvaluationError, AnimationTargetId},
    prelude::*,
};
use std::{any::TypeId, collections::HashMap};

use crate::{components::*, constants::*, event::*, resources::*};

pub struct AnimationInfo {
    clip: AnimationClip,
    target_id: AnimationTargetId,
}

pub fn generate_win_cell_animation(animation_target_name: &Name) -> AnimationInfo {
    // Create an ID that identifies the text node we're going to animate.
    let animation_target_id = AnimationTargetId::from_name(animation_target_name);

    // Allocate an animation clip.
    let mut animation_clip = AnimationClip::default();

    // Create a curve that animates `Background color`.
    animation_clip.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            BackgroundColorProperty,
            AnimatableKeyframeCurve::new([0.0, 0.8].into_iter().zip([
                Srgba::hex(BOARD_COLOR).unwrap(),
                Srgba::hex(WINNER_COLOR).unwrap(),
            ]))
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    AnimationInfo {
        clip: animation_clip,
        target_id: animation_target_id,
    }
}

#[derive(Clone)]
struct BackgroundColorProperty;

impl AnimatableProperty for BackgroundColorProperty {
    type Property = Srgba;

    fn get_mut<'a>(
        &self,
        entity: &'a mut bevy::animation::AnimationEntityMut,
    ) -> std::result::Result<&'a mut Self::Property, AnimationEvaluationError> {
        let background_color = entity
            .get_mut::<BackgroundColor>()
            .ok_or(AnimationEvaluationError::ComponentNotPresent(TypeId::of::<
                BackgroundColor,
            >(
            )))?
            .into_inner();
        match background_color.0 {
            Color::Srgba(ref mut color) => Ok(color),
            _ => Err(AnimationEvaluationError::PropertyNotPresent(TypeId::of::<
                Srgba,
            >(
            ))),
        }
    }

    fn evaluator_id(&self) -> EvaluatorId<'_> {
        EvaluatorId::Type(TypeId::of::<Self>())
    }
}

pub fn game_won(
    winner: On<GameWon>,
    mut query: Query<(Entity, &Node, &mut Cell)>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut game_over_res: ResMut<GameOver>,
    mut commands: Commands,
) {
    game_over_res.0 = true;
    for (entity, _nodee, cell) in query
        .iter_mut()
        .filter(|(_e, _n, c)| winner.positions.contains(&c.position))
    {
        let animation_info = generate_win_cell_animation(&Name(cell.position.to_str().into()));

        // Create the animation graph
        let (graph, animation_index) =
            AnimationGraph::from_clip(animations.add(animation_info.clip));

        // // Create the animation player, and set it to repeat
        let mut player = AnimationPlayer::default();
        player.play(animation_index);

        commands.entity(entity).despawn_children();
        commands.entity(entity).insert((
            animation_info.target_id,
            AnimatedBy(entity),
            player,
            AnimationGraphHandle(graphs.add(graph)),
        ));
    }
}

pub fn detect_win(_: On<Pointer<Click>>, query: Query<&Cell>, mut commands: Commands) {
    let mut value_map: HashMap<&CellPosition, &Option<Turn>> = Default::default();
    for cell in query {
        value_map.insert(&cell.position, &cell.val);
    }

    for position in CellPosition::winning_positions() {
        if let Some(first_valid_turn) = value_map.get(&position[0]) {
            if let Some(second_valid_turn) = value_map.get(&position[1]) {
                if let Some(third_valid_turn) = value_map.get(&position[2]) {
                    if first_valid_turn == second_valid_turn
                        && second_valid_turn == third_valid_turn
                        && first_valid_turn == third_valid_turn
                    {
                        if let Some(turn) = first_valid_turn {
                            commands.trigger(GameWon {
                                turn: if *turn == Turn::O { Turn::O } else { Turn::X },
                                positions: [position[0], position[1], position[2]],
                            });
                        }
                    }
                }
            }
        }
    }
}
