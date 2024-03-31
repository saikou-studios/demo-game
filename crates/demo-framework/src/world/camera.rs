use bevy::{
    app::{App, Plugin, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        event::EventReader,
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query},
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::Component,
    render::camera::{OrthographicProjection, ScalingMode},
};

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_main_camera)
            .add_systems(Update, zoom_camera.run_if(in_state(GameState::Playing)));
    }
}

const PROJECTION_SCALE: f32 = 200.0;

#[derive(Component)]
pub struct MainCamera;

fn setup_main_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(PROJECTION_SCALE);
    commands.spawn((camera, MainCamera));
}

fn zoom_camera(
    mut projection_q: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if let Ok(mut projection) = projection_q.get_single_mut() {
        for event in scroll_events.read() {
            if event.unit == MouseScrollUnit::Line {
                projection.scale = (projection.scale - event.y).clamp(1.0, 10.0);
            }
        }
    }
}
