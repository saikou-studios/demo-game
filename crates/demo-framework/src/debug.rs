use bevy::app::{App, Plugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            use bevy::diagnostic;

            app.add_plugins((
                diagnostic::FrameTimeDiagnosticsPlugin,
                diagnostic::LogDiagnosticsPlugin::default(),
                diagnostic::EntityCountDiagnosticsPlugin::default(),
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
            ));
        }
    }
}
