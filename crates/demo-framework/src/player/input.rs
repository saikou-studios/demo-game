use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        event::EventReader,
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res, ResMut, Resource},
    },
    input::{
        keyboard::KeyCode,
        mouse::{MouseButton, MouseWheel},
        ButtonInput, InputSystem,
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

use crate::{camera::MainCamera, GameState};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .init_resource::<MouseWorldCoords>()
            .add_systems(
                PreUpdate,
                (
                    reset_player_input.before(InputSystem),
                    (
                        update_mouse_position_in_world,
                        update_scroll_event,
                        update_movement_direction,
                        update_is_running,
                        update_is_attacking,
                    )
                        .after(InputSystem),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct PlayerInput {
    pub movement_direction: Vec2,
    pub zoom: f32,
    pub is_running: bool,
    pub is_left_attack: bool,
    pub is_right_attack: bool,
}

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

fn update_mouse_position_in_world(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_coords: ResMut<MouseWorldCoords>,
) {
    if let Ok((camera, transform)) = camera_q.get_single() {
        if let Ok(window) = window_q.get_single() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                mouse_coords.0 = world_position;
            }
        }
    }
}

fn update_scroll_event(
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

fn update_is_attacking(
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    if mouse.pressed(MouseButton::Left) {
        player_input.is_left_attack = true;
        player_input.is_right_attack = false;
    } else if mouse.pressed(MouseButton::Right) {
        player_input.is_right_attack = true;
        player_input.is_left_attack = false;
    }
}
