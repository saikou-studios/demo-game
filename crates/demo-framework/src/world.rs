mod chunk;
mod helpers;
mod tile;

use bevy::{
    app::{App, Plugin},
    ecs::{schedule::OnExit, system::ResMut},
    math::Vec2,
};
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(chunk::ChunkPlugin)
            .add_systems(OnExit(GameState::Loading), configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
