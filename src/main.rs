// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::{ClearColor, Color, Entity, Msaa, NonSend, Query, With};
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use demo_game::GamePlugin;
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Game".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            GamePlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("build/icon_256x256.png")
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon.clone()));
}
