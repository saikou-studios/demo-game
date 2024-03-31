use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        event::EventReader,
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::{Quat, Vec2, Vec3},
    prelude::Component,
    reflect::Reflect,
    render::camera::{OrthographicProjection, ScalingMode},
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::dynamics::Velocity;
use noise::{core::simplex::simplex_2d, permutationtable::PermutationTable, Vector2};

use crate::{player::Player, GameState};

const PROJECTION_SCALE: f32 = 200.0;
const TIMESTEP: f32 = 60.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Shake>()
            .add_systems(OnEnter(GameState::Playing), setup_main_camera)
            .add_systems(
                Update,
                (zoom_camera, decay_shake_trauma).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                (update_camera_to_target.before(update_camera), update_camera)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Reflect, Resource)]
pub struct Shake {
    trauma: f32,
    seed: f32,
    target: Vec2,
    noise_strength: f32,
    translation_shake_strength: f32,
    rotation_shake_strength: f32,
}

impl Default for Shake {
    fn default() -> Self {
        Self {
            trauma: f32::default(),
            seed: f32::default(),
            target: Vec2::default(),
            noise_strength: 10.0,
            translation_shake_strength: 15.0,
            rotation_shake_strength: 2.5,
        }
    }
}

impl Shake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    pub fn update_target(&mut self, target: Vec2) {
        self.target = target;
    }

    /// Exponentially decays the trauma value over time to create a smooth decay effect
    /// The decay rate is controlled by the `delta` time between frames.
    pub fn decay_trauma(&mut self, delta: f32) {
        const DECAY_RATE: f32 = 1.5;
        self.trauma *= f32::exp(-DECAY_RATE * delta);
    }

    fn noise(&self, stack: u32) -> f32 {
        let hasher = PermutationTable::new(self.seed as u32 + stack);

        simplex_2d(
            Vector2::new(self.trauma as f64 * self.noise_strength as f64, 0.0),
            &hasher,
        )
        .0 as f32
    }
}

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

fn decay_shake_trauma(time: Res<Time>, mut shake: ResMut<Shake>) {
    shake.decay_trauma(time.delta_seconds());
}

/// Updates the camera's position and rotation based on the current shake
/// effect. This includes applying a translation offset and a rotation offset
/// to simulate camera shake. The magnitude of these effects is determined by
/// the current level of "trauma" within the `Shake` resource, simulating an
/// impact or shake effect in response to in-game events.
fn update_camera(mut camera_q: Query<&mut Transform, With<MainCamera>>, shake: Res<Shake>) {
    if let Ok(mut transform) = camera_q.get_single_mut() {
        let translation_offset = Vec3::new(shake.noise(0), shake.noise(1), 0.0)
            * shake.trauma.powi(2)
            * shake.translation_shake_strength;
        let rotation_offset = Quat::from_rotation_z(
            (shake.noise(2) * shake.trauma.powi(2) * shake.rotation_shake_strength).to_radians(),
        );

        transform.translation = shake.target.extend(transform.translation.z) + translation_offset;
        transform.rotation = rotation_offset;
    }
}

/// Updates the target position for the camera shake effect based on the player's current
/// position and velocity. This anticipates the player's future position, providing a
/// smoother camera movement.
fn update_camera_to_target(
    mut shake: ResMut<Shake>,
    player_q: Query<(&Transform, &Velocity), With<Player>>,
) {
    if let Ok((transform, velocity)) = player_q.get_single() {
        shake.update_target(transform.translation.truncate() + velocity.linvel / TIMESTEP);
    }
}
