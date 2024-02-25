// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::prelude::{ClearColor, Color, Entity, Msaa, NonSend, Query, With};
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use demo_game::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Demo Game {}", demo_game::version::version()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    file_path: "../../assets".to_string(),
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
    let icon_buf = Cursor::new(include_bytes!("../../build/icon_256x256.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
