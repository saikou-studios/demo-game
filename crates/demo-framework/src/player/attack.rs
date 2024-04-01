use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    math::{EulerRot, Quat, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
    time::Time,
    transform::components::Transform,
};
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{loading::TextureAssets, GameState};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    CostType, Player,
};

pub struct PlayerAttackPlugin;

impl Plugin for PlayerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnMissile>().add_systems(
            Update,
            (
                spawn_missile,
                despawn_missile,
                spell_casting,
                spell_cooldown.after(spell_casting),
                update_missile_path,
                update_animations,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Missile(pub f32);

#[derive(Event)]
pub struct SpawnMissile {
    pub range: f32,
    pub target_pos: Vec2,
}

#[derive(Component)]
pub struct Direction(pub Vec2);

pub fn spawn_missile(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    mut spawn_missile_events: EventReader<SpawnMissile>,
) {
    if let Ok((entity, transform)) = player_q.get_single() {
        let player_pos = transform.translation.truncate();

        for event in spawn_missile_events.read() {
            let mut animator = AnimationPlayer2D::default();
            animator
                .play(texture_assets.ice_spell_one_animations[0].clone())
                .repeat();

            let direction = (event.target_pos - player_pos).normalize_or_zero();

            let missile = commands
                .spawn((
                    Missile(event.range),
                    Direction(direction),
                    animator,
                    SpriteSheetBundle {
                        transform: Transform::from_translation(player_pos.extend(1.0))
                            .with_rotation(if direction == Vec2::ZERO {
                                Quat::IDENTITY
                            } else {
                                Quat::from_euler(
                                    EulerRot::XYZ,
                                    0.0,
                                    0.0,
                                    Vec2::X.angle_between(direction),
                                )
                            }),
                        texture: texture_assets.ice_spell_one.clone(),
                        atlas: TextureAtlas {
                            layout: texture_assets.ice_spell_one_layout.clone(),
                            index: 0,
                        },
                        ..Default::default()
                    },
                ))
                .id();

            commands.entity(entity).push_children(&[missile]);
        }
    }
}

pub fn despawn_missile(
    mut commands: Commands,
    missiles_q: Query<(&Transform, Entity, &Missile)>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        let player_pos = player_transform.translation.truncate();
        for (transform, entity, missile) in &missiles_q {
            let missile_pos = transform.translation.truncate();
            let dist = player_pos.distance(missile_pos);
            if dist >= missile.0 * 2.0 {
                println!("Despawning missile: {:?}, distance => {:?}", entity, dist);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spell_cooldown(mut player_q: Query<&mut Player>, time: Res<Time>) {
    for mut player in player_q.iter_mut() {
        player.attack_cooldown.0.tick(time.delta());
    }
}

fn spell_casting(
    mut player_q: Query<&mut Player>,
    player_input: Res<PlayerInput>,
    mouse_coords: Res<MouseWorldCoords>,
    mut spawn_missile_event: EventWriter<SpawnMissile>,
) {
    if let Ok(mut player) = player_q.get_single_mut() {
        if player.attack_cooldown.0.finished() {
            let skill_to_use = if player_input.is_left_attack {
                &player.selected_left_skill
            } else if player_input.is_right_attack {
                &player.selected_right_skill
            } else {
                return;
            };

            let range = skill_to_use.range().clone();

            let cost = skill_to_use.cost();
            let mut casted = false;
            match cost.cost_type {
                CostType::Mana => {
                    if player.stats.mana >= cost.value {
                        player.stats.mana -= cost.value;
                        casted = true;
                        spawn_missile_event.send(SpawnMissile {
                            range,
                            target_pos: mouse_coords.0,
                        });
                    }
                }
                CostType::None => {
                    casted = true;
                    spawn_missile_event.send(SpawnMissile {
                        range,
                        target_pos: mouse_coords.0,
                    });
                }
                _ => {
                    println!("unimplemented..");
                    casted = false;
                }
            }

            if casted {
                player.attack_cooldown.0.reset();
            }
        }
    }
}

pub fn update_missile_path(
    mut missile_q: Query<(&Direction, &mut Transform), With<Missile>>,
    time: Res<Time>,
) {
    let missile_speed = 200.0;
    for (direction, mut transform) in missile_q.iter_mut() {
        let displacement = direction.0 * missile_speed * time.delta_seconds();
        // println!(
        //     "Direction {:?}, Displacement {:?}",
        //     direction.0, displacement
        // );
        transform.translation += displacement.extend(0.0);
    }
}

fn update_animations(
    mut missile_q: Query<&mut AnimationPlayer2D, With<Missile>>,
    texture_assets: Res<TextureAssets>,
) {
    if let Ok(mut animator) = missile_q.get_single_mut() {
        if animator.is_finished() {
            let clip = texture_assets.ice_spell_one_animations[1].clone();
            animator.play(clip).repeat();
        }
    }
}
