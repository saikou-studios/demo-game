use bevy::{
    input::ButtonInput,
    math::Vec2,
    prelude::{Camera, KeyCode, Query, Res, Transform, With, Without},
    time::Time,
};
use bevy_rapier2d::control::KinematicCharacterController;

use crate::player::{MovementDirection, MovementState, Player};

pub(crate) fn camera_follow_player(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_q.get_single(), camera_q.get_single_mut())
    {
        camera_transform.translation = player_transform.translation.truncate().extend(999.0);
    }
}

pub(crate) fn player_movement(
    mut player_q: Query<(
        &Player,
        &mut MovementState,
        &mut MovementDirection,
        &mut KinematicCharacterController,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut movement_state, mut movement_direction, mut controller)) =
        player_q.get_single_mut()
    {
        let is_running = keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        let base_speed = player.speed;
        let speed = if is_running {
            base_speed * 1.5
        } else {
            base_speed
        };

        let mut translation = Vec2::default();
        let mut is_moving = false;
        let delta_time = time.delta_seconds();

        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            translation.y += speed * delta_time;
            *movement_direction = MovementDirection::Up;
            is_moving = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            translation.y -= speed * time.delta_seconds();
            *movement_direction = MovementDirection::Down;
            is_moving = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            translation.x -= speed * time.delta_seconds();
            *movement_direction = MovementDirection::Left;
            is_moving = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            translation.x += speed * time.delta_seconds();
            *movement_direction = MovementDirection::Right;
            is_moving = true;
        }

        *movement_state = match (is_moving, is_running) {
            (true, true) => MovementState::Running,
            (true, false) => MovementState::Walking,
            _ => MovementState::Idle,
        };

        controller.translation = Some(translation);
    }
}
