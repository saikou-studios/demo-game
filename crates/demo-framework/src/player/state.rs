use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, PostUpdate},
    ecs::{
        component::Component,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res},
    },
    math::Vec2,
    sprite::Sprite,
    time::Timer,
};
use bevy_rapier2d::dynamics::Velocity;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{loading::TextureAssets, GameState};

use super::{input::PlayerInput, Player};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_player_movement_state,
                update_animations.after(update_player_movement_state),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Debug, Default)]
pub struct CooldownTimer(pub Timer);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum MovementState {
    #[default]
    Idle,
    Walking,
    Sprinting,
}

fn update_player_movement_state(
    mut player_q: Query<(&Velocity, &mut Player)>,
    player_input: Res<PlayerInput>,
) {
    if let Ok((velocity, mut player)) = player_q.get_single_mut() {
        let state = if velocity.linvel == Vec2::ZERO {
            MovementState::Idle
        } else if player_input.is_running {
            MovementState::Sprinting
        } else {
            MovementState::Walking
        };
        player.movement_state = state;
    }
}

fn update_animations(
    mut player_q: Query<(&Velocity, &mut AnimationPlayer2D, &mut Sprite, &Player)>,
    texture_assets: Res<TextureAssets>,
) {
    if let Ok((velocity, mut animator, mut sprite, player)) = player_q.get_single_mut() {
        let dir = if velocity.linvel == Vec2::ZERO {
            if player.current_direction == Vec2::ZERO {
                Vec2::NEG_X
            } else {
                player.current_direction
            }
        } else {
            velocity.linvel
        };

        let angle = Vec2::X.angle_between(dir);
        let dir_index = match angle {
            a if (-3.0 / 4.0 * PI..=-1.0 / 4.0 * PI).contains(&a) => 0, // Down
            a if (1.0 / 4.0 * PI..=3.0 / 4.0 * PI).contains(&a) => 3,   // Up
            a if !(-3.0 / 4.0 * PI..=3.0 / 4.0 * PI).contains(&a) => 1, // Left
            _ => 2,                                                     // Right
        };

        sprite.flip_x = dir_index == 2;

        let (animation_index, repeat) = match player.movement_state {
            MovementState::Idle => (dir_index, true),
            MovementState::Walking => (4 + dir_index, true),
            MovementState::Sprinting => (8 + dir_index, true),
            // Attack => no repeat probably??
        };

        let clip = texture_assets.female_adventurer_animations[animation_index].clone();
        if repeat {
            animator.play(clip).repeat();
        } else {
            animator.play(clip);
        }
    }
}
