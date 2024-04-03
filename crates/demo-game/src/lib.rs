use bevy::{
    app::{App, Plugin},
    ecs::{schedule::OnEnter, system::ResMut},
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_magic_light_2d::gi::{resource::LightPassParams, BevyMagicLight2DPlugin};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_trickfilm::Animation2DPlugin;

use demo_framework::{
    camera::CameraPlugin, loading::LoadingPlugin, player::PlayerPlugin, ui::UiPlugin,
    world::WorldPlugin, GameState,
};
use discord::{ActivityState, DiscordClient};

pub mod version;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let app_id = std::env::var("APPLICATION_ID")
            .expect("APPLICATION_ID not set")
            .parse()
            .expect("APPLICATION_ID is not a valid u64");

        app.init_state::<GameState>()
            .add_plugins((
                TilemapPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                Animation2DPlugin,
                BevyMagicLight2DPlugin,
                LoadingPlugin,
                discord::DiscordPlugin::new(app_id, true),
                UiPlugin,
                CameraPlugin,
                WorldPlugin,
                PlayerPlugin,
            ))
            .add_systems(OnEnter(GameState::Playing), update_presence);

        #[cfg(debug_assertions)]
        {
            app.add_plugins(demo_framework::debug::DebugPlugin);
        }
    }
}

fn update_presence(mut client: ResMut<DiscordClient>) {
    client.update_activity(&ActivityState {
        details: Some("In Game".to_string()),
        ..Default::default()
    });
}
