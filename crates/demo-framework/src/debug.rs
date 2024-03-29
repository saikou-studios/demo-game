pub struct DebugPlugin;

impl bevy::app::Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
                bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
            ));
        }
    }
}
