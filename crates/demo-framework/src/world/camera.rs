use bevy::{
    app::{App, Plugin},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{schedule::OnEnter, system::Commands},
    prelude::Component,
    render::camera::ScalingMode,
};

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_camera);
    }
}

const PROJECTION_SCALE: f32 = 200.0;

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(PROJECTION_SCALE);
    commands.spawn((camera, MainCamera));
}
