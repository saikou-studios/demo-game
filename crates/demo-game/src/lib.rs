use bevy::app::Update;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, ResMut};
use demo_framework::{debug, loading, menu, player, ui, GameState};
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
            .add_systems(
                OnEnter(GameState::Playing),
                (player::spawn_player, update_presence),
            )
            .add_systems(
                Update,
                player::animate_sprite_system.run_if(in_state(GameState::Playing)),
            )
            .add_plugins((
                loading::LoadingPlugin,
                menu::MenuPlugin,
                ui::UiPlugin,
                discord::DiscordPlugin::new(app_id, true),
            ));

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
