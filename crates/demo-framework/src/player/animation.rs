use crate::player::{MovementDirection, MovementState};
use bevy::prelude::{Component, Query, Res, TextureAtlas, Time, Timer};
use bevy::sprite::Sprite;

#[derive(Component)]
pub(crate) struct AnimationTimer(pub(crate) Timer);

pub(crate) fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(
        &MovementDirection,
        &MovementState,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut Sprite,
    )>,
) {
    for (movement_direction, movement_state, mut timer, mut atlas, mut sprite) in
        &mut sprites_to_animate
    {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let calc_index = |base: usize, range: usize| (atlas.index + 1) % range + base;
            let (base_index, range_size) = match (movement_direction, movement_state) {
                (MovementDirection::Up, MovementState::Walking)
                | (MovementDirection::Up, MovementState::Running) => (20, 4),
                (MovementDirection::Up, MovementState::Idle) => (16, 4),

                (MovementDirection::Down, MovementState::Walking)
                | (MovementDirection::Down, MovementState::Running) => (12, 4),
                (MovementDirection::Down, MovementState::Idle) => (8, 4),

                (MovementDirection::Left, MovementState::Walking)
                | (MovementDirection::Left, MovementState::Running)
                | (MovementDirection::Right, MovementState::Walking)
                | (MovementDirection::Right, MovementState::Running) => (4, 4),
                (MovementDirection::Left, MovementState::Idle)
                | (MovementDirection::Right, MovementState::Idle) => (0, 4),
            };

            atlas.index = calc_index(base_index, range_size);
            sprite.flip_x = matches!(movement_direction, MovementDirection::Right);
        }
    }
}
