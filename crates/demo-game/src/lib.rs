use bevy::app::Update;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};
use demo_framework::{debug, loading, menu, player, ui, GameState};

pub mod version;

pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), player::spawn_player)
            .add_systems(
                Update,
                player::animate_sprite_system.run_if(in_state(GameState::Playing)),
            )
            .add_plugins((loading::LoadingPlugin, menu::MenuPlugin, ui::UiPlugin));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}
