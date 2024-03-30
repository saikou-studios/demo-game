mod animation;
mod movement;

use crate::{
    loading::TextureAssets,
    player::animation::{animate_sprite_system, AnimationTimer},
    player::movement::player_movement,
    GameState,
};

use crate::player::movement::camera_follow_player;
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, Commands, Component, IntoSystemConfigs, Name, OnEnter, Res, Transform},
    sprite::{SpriteBundle, TextureAtlas},
    time::{Timer, TimerMode},
};
use bevy_rapier2d::{
    control::KinematicCharacterController, dynamics::RigidBody, geometry::Collider,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    animate_sprite_system,
                    (player_movement, camera_follow_player).chain(),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 100. }
    }
}

#[derive(Component, Debug, Default)]
pub enum MovementDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Running,
}

pub fn spawn_player(mut commands: Commands, my_assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: my_assets.female_adventurer.clone(),
            transform: Transform::from_xyz(0., -150., 1.),
            ..Default::default()
        },
        TextureAtlas::from(my_assets.female_adventurer_layout.clone()),
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        KinematicCharacterController::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(15.0, 9.0),
        Player::default(),
        MovementState::default(),
        MovementDirection::default(),
        Name::new("Player"),
    ));
}
