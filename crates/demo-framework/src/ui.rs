use bevy::app::{App, Plugin};

mod diagnostics;
mod hud;
mod menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::MenuPlugin,
            diagnostics::DiagnosticsPlugin,
            hud::HudPlugin,
        ));
    }
}
