use bevy::prelude::ResMut;
use demo_framework::{debug, loading, player, ui, world, GameState};
use discord::{ActivityState, DiscordClient};

pub mod version;

pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let app_id = std::env::var("APPLICATION_ID")
            .expect("APPLICATION_ID not set")
            .parse()
            .expect("APPLICATION_ID is not a valid u64");

        app.init_state::<GameState>()
            .add_plugins((
                loading::LoadingPlugin,
                discord::DiscordPlugin::new(app_id, true),
                ui::UiPlugin,
                bevy_ecs_tilemap::TilemapPlugin,
                bevy_rapier2d::plugin::RapierPhysicsPlugin::<bevy_rapier2d::plugin::NoUserData>::pixels_per_meter(100.0),
                world::WorldPlugin,
                player::PlayerPlugin,
            ))
            .add_systems(
                bevy::ecs::schedule::OnEnter(GameState::Playing),
                update_presence,
            );

        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}

fn update_presence(mut client: ResMut<DiscordClient>) {
    client.update_activity(&ActivityState {
        details: Some("In Game".to_string()),
        ..Default::default()
    });
}
