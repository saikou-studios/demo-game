use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use iyes_progress::ProgressPlugin;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
            .add_loading_state(
                LoadingState::new(GameState::Loading).load_collection::<TextureAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/grass_land/main.png")]
    pub grass_land: Handle<Image>,
    #[asset(path = "textures/grass_land/decorative.png")]
    pub grass_land_decorative: Handle<Image>,
}
