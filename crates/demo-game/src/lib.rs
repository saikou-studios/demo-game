use demo_framework::{debug, loading, menu, GameState};

pub mod version;

pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<GameState>()
            .add_plugins((loading::LoadingPlugin, menu::MenuPlugin));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}
