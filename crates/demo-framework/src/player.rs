use crate::loading::TextureAssets;
use bevy::prelude::{Commands, Component, Query, Res, Transform};
use bevy::sprite::{SpriteBundle, TextureAtlas};
use bevy::time::{Time, Timer, TimerMode};

#[derive(Component)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(mut commands: Commands, my_assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: my_assets.female_adventurer.clone(),
            transform: Transform::from_xyz(0., -150., 0.),
            ..Default::default()
        },
        TextureAtlas::from(my_assets.female_adventurer_layout.clone()),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut sprite) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}
