mod attack;
pub(crate) mod input;
mod movement;
mod state;

use std::{collections::HashMap, time::Duration};

use crate::{loading::TextureAssets, GameState};

use bevy::{
    app::{App, Plugin},
    ecs::entity::Entity,
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Name, OnEnter, Res, SpatialBundle, Transform},
    render::view::RenderLayers,
    sprite::{SpriteSheetBundle, TextureAtlas},
    time::{Timer, TimerMode},
    transform::TransformBundle,
};
use bevy_magic_light_2d::gi::{render_layer::CAMERA_LAYER_OBJECTS, types::LightOccluder2D};
use bevy_rapier2d::{
    dynamics::{Ccd, LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, CollisionGroups, Group},
};
use bevy_trickfilm::prelude::AnimationPlayer2D;

use self::state::{CooldownTimer, MovementState};

const PLAYER_SPAWN_POS: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_plugins((
                attack::PlayerAttackPlugin,
                input::PlayerInputPlugin,
                state::PlayerStatePlugin,
                movement::PlayerMovementPlugin,
            ));
    }
}

#[derive(Component, Debug)]
pub(crate) struct Player {
    pub(crate) movement_state: MovementState,
    pub(crate) current_direction: Vec2,
    #[allow(dead_code)]
    pub(crate) collider_entity: Entity,
    pub attributes: Attributes,
    pub stats: Stats,
    pub resistances: Resistances,
    pub experience: Experience,
    pub skills: Skills,
    pub(crate) attack_cooldown: CooldownTimer,
    pub selected_left_skill: Skill,
    pub selected_right_skill: Skill,
}

#[derive(Component, Debug)]
pub struct Attributes {
    pub strength: i32,
    pub vitality: i32,
    pub energy: i32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: 10,
            vitality: 10,
            energy: 15,
        }
    }
}

#[derive(Component, Debug)]
pub struct Stats {
    pub life: f32,
    pub mana: f32,
    pub stamina: f32,
    pub defense: f32,
    pub cast_rate: f32,
    pub attack_rate: f32,
    pub resistances: Resistances,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            life: 15.0,                          // vitality * 1.5
            mana: 30.0,                          // energy * 2
            stamina: 20.0,                       // vitality * 2
            defense: 12.0,                       // strength * 1.2
            cast_rate: 0.0,                      // influenced by items
            attack_rate: 0.0,                    // influenced by items
            resistances: Resistances::default(), // influenced by items
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Resistances {
    pub light: f32,
    pub fire: f32,
    pub cold: f32,
}

#[derive(Component, Debug)]
pub struct Experience {
    pub current: u32,
    pub remaining: u32,
    pub level: u16,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            current: 0,
            remaining: 100, // base_exp => 100, growth => 1.5, base_exp * growth.powi(level - 1)).round()
            level: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CostType {
    #[default]
    None,
    Life,
    Mana,
    Stamina,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cost {
    pub cost_type: CostType,
    pub value: f32,
}

impl Cost {
    pub fn new(cost_type: CostType, value: f32) -> Self {
        Self { cost_type, value }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Skill {
    // Melee
    Attack {
        base_damage: f32,
        range: f32,
    },
    // Cold spells
    IceBolt {
        base_damage: f32,
        mana_cost: f32,
        level: u16,
        range: f32,
    },
    IceBlast {
        base_damage: f32,
        mana_cost: f32,
        level: u16,
        range: f32,
    },
    IceNova {
        base_damage: f32,
        mana_cost: f32,
        level: u16,
        range: f32,
    },
}

impl Skill {
    pub fn cost(&self) -> Cost {
        match self {
            Skill::Attack { .. } => Cost::default(),
            Skill::IceBolt { mana_cost, .. } => Cost::new(CostType::Mana, *mana_cost),
            Skill::IceBlast { mana_cost, .. } => Cost::new(CostType::Mana, *mana_cost),
            Skill::IceNova { mana_cost, .. } => Cost::new(CostType::Mana, *mana_cost),
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            Skill::Attack { range, .. } => range.clone(),
            Skill::IceBolt { range, .. } => range.clone(),
            Skill::IceBlast { range, .. } => range.clone(),
            Skill::IceNova { range, .. } => range.clone(),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Skills {
    pub skills: HashMap<String, Skill>,
    pub available_points: u16, // available to spend on upgrading
}

impl Default for Skills {
    fn default() -> Self {
        let mut skills = HashMap::new();

        skills.insert(
            "Attack".to_string(),
            Skill::Attack {
                base_damage: 3.0,
                range: 100.0,
            },
        );

        Self {
            skills,
            available_points: 0,
        }
    }
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        let skills = Skills::default();

        let selected_left_skill = *skills.skills.get("Attack").unwrap();
        let selected_right_skill = *skills.skills.get("Attack").unwrap();

        Self {
            movement_state: MovementState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
            attributes: Attributes::default(),
            stats: Stats::default(),
            resistances: Resistances::default(),
            experience: Experience::default(),
            skills: skills.clone(),
            attack_cooldown: CooldownTimer(Timer::new(
                Duration::from_millis(400),
                TimerMode::Repeating,
            )),
            selected_left_skill,
            selected_right_skill,
        }
    }
}

pub fn spawn_player(mut commands: Commands, my_assets: Res<TextureAssets>) {
    let collider = commands
        .spawn((
            Collider::capsule_y(15.0, 9.0),
            ActiveEvents::COLLISION_EVENTS,
            PLAYER_COLLISION_GROUPS,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(my_assets.female_adventurer_animations[0].clone_weak());

    commands
        .spawn((
            Name::new("Player"),
            Player::new(collider),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS),
                texture: my_assets.female_adventurer.clone(),
                atlas: TextureAtlas {
                    layout: my_assets.female_adventurer_layout.clone(),
                    index: 0,
                },
                ..Default::default()
            },
            RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
            LightOccluder2D {
                h_size: Vec2::new(2.0, 2.0),
            },
        ))
        .push_children(&[collider]);
}
