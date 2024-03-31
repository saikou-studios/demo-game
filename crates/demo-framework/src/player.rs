mod input;
mod movement;
mod state;

use crate::{loading::TextureAssets, GameState};

use bevy::{
    app::{App, Plugin},
    ecs::entity::Entity,
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Name, OnEnter, Res, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas},
    transform::TransformBundle,
};
use bevy_rapier2d::{
    dynamics::{Ccd, LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, CollisionGroups, Group},
};
use bevy_trickfilm::prelude::AnimationPlayer2D;

use self::state::MovementState;

const PLAYER_SPAWN_POS: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_plugins((
                input::PlayerInputPlugin,
                state::PlayerStatePlugin,
                movement::PlayerMovementPlugin,
            ));
        // .add_systems(
        //     Update,
        //     (
        //         animate_sprite_system,
        //         (player_movement, camera_follow_player).chain(),
        //     )
        //         .run_if(in_state(GameState::Playing)),
        // );
    }
}

#[derive(Component, Debug)]
pub(crate) struct Player {
    pub(crate) movement_state: MovementState,
    pub(crate) current_direction: Vec2,
    pub(crate) collider_entity: Entity,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            movement_state: MovementState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
        }
    }
}

pub fn spawn_player(mut commands: Commands, my_assets: Res<TextureAssets>) {
    let collider = commands
        .spawn((
            Collider::capsule_y(15.0, 9.0),
            ActiveEvents::COLLISION_EVENTS,
            PLAYER_COLLISION_GROUPS,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -32.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(my_assets.female_adventurer_animations[0].clone_weak());

    commands
        .spawn((
            Player::new(collider),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
                texture: my_assets.female_adventurer.clone(),
                atlas: TextureAtlas {
                    layout: my_assets.female_adventurer_layout.clone(),
                    index: 0,
                },
                ..Default::default()
            },
            Name::new("Player"),
        ))
        .push_children(&[collider]);
}
