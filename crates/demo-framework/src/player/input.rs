use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        event::EventReader,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, mouse::MouseWheel, ButtonInput, InputSystem},
    math::Vec2,
};

use crate::GameState;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(
                PreUpdate,
                reset_player_input
                    .before(InputSystem)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PreUpdate,
                (
                    on_scroll_event,
                    update_movement_direction,
                    update_is_running,
                )
                    .after(InputSystem)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct PlayerInput {
    pub movement_direction: Vec2,
    pub zoom: f32,
    pub is_running: bool,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

fn on_scroll_event(
    mut scroll_events: EventReader<MouseWheel>,
    mut player_input: ResMut<PlayerInput>,
) {
    for event in scroll_events.read() {
        player_input.zoom = -event.y;
    }
}

fn update_movement_direction(
    key: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut dir = Vec2::default();
    if key.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        dir += Vec2::new(0.0, 1.0);
    }
    if key.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        dir += Vec2::new(-1.0, 0.0);
    }
    if key.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        dir += Vec2::new(0.0, -1.0);
    }
    if key.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        dir += Vec2::new(1.0, 0.0);
    }
    player_input.movement_direction = dir.normalize_or_zero();
}

fn update_is_running(key: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.is_running = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
}
