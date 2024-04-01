use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    math::Vec2,
    prelude::{Query, Res},
};
use bevy_rapier2d::dynamics::Velocity;

use crate::GameState;

use super::{attack::SpawnMissile, input::PlayerInput, MovementState, Player};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnMissile>()
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

fn player_movement(
    mut player_q: Query<(&mut Velocity, &mut Player)>,
    player_input: Res<PlayerInput>,
) {
    if let Ok((mut velocity, mut player)) = player_q.get_single_mut() {
        let dir = player_input.movement_direction;
        if dir == Vec2::default() {
            velocity.linvel = Vec2::ZERO;
            return;
        }

        let speed = if player.movement_state == MovementState::Sprinting {
            100.0
        } else {
            75.0
        };

        player.current_direction = dir;
        velocity.linvel = dir * speed;
    }
}
