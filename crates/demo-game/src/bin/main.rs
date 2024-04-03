// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    app::{App, PluginGroup, Startup},
    asset::{AssetMetaCheck, AssetPlugin},
    ecs::{
        entity::Entity,
        query::With,
        system::{NonSend, Query},
    },
    render::{
        camera::ClearColor,
        color::Color,
        texture::{ImageFilterMode, ImagePlugin, ImageSamplerDescriptor},
        view::Msaa,
    },
    window::{PrimaryWindow, Window, WindowPlugin},
    winit::WinitWindows,
    DefaultPlugins,
};
use demo_game::{version::version, GamePlugin};
use std::io::Cursor;
use winit::window::Icon;

const SCREEN_SIZE: (f32, f32) = (1280.0, 720.0);

fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgba_u8(0, 0, 0, 0)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: SCREEN_SIZE.into(),
                        title: format!("Demo Game {}", version()),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    file_path: "../../assets".to_string(),
                    ..Default::default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        mag_filter: ImageFilterMode::Nearest,
                        min_filter: ImageFilterMode::Nearest,
                        ..Default::default()
                    },
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
