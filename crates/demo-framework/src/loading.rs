use crate::GameState;
use bevy::{
    app::{App, Plugin},
    asset::{AssetServer, Assets, Handle},
    ecs::system::Resource,
    math::Vec2,
    prelude::TextureAtlasLayout,
    render::texture::Image,
    text::Font,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
    prelude::ConfigureLoadingState,
};
use bevy_trickfilm::asset::AnimationClip2D;
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
    // UI (main-menu)
    #[asset(path = "textures/backgrounds/desert_mountains/background1.png")]
    pub(crate) background_one: Handle<Image>,
    #[asset(path = "textures/backgrounds/desert_mountains/background2.png")]
    pub(crate) background_two: Handle<Image>,
    #[asset(path = "textures/backgrounds/desert_mountains/background3.png")]
    pub(crate) background_three: Handle<Image>,
    #[asset(path = "fonts/dungeon_font.ttf")]
    pub(crate) dungeon_font: Handle<Font>,

    // player
    #[asset(texture_atlas_layout(tile_size_x = 32.0, tile_size_y = 48.0, columns = 8, rows = 3))]
    pub(crate) female_adventurer_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/npc_characters/female_2.png")]
    pub(crate) female_adventurer: Handle<Image>,
    #[asset(
        paths(
            "textures/npc_characters/female_2.trickfilm#idle-down",
            "textures/npc_characters/female_2.trickfilm#idle-left",
            "textures/npc_characters/female_2.trickfilm#idle-right",
            "textures/npc_characters/female_2.trickfilm#idle-top",
            "textures/npc_characters/female_2.trickfilm#walking-down",
            "textures/npc_characters/female_2.trickfilm#walking-left",
            "textures/npc_characters/female_2.trickfilm#walking-right",
            "textures/npc_characters/female_2.trickfilm#walking-top",
            "textures/npc_characters/female_2.trickfilm#sprinting-down",
            "textures/npc_characters/female_2.trickfilm#sprinting-left",
            "textures/npc_characters/female_2.trickfilm#sprinting-right",
            "textures/npc_characters/female_2.trickfilm#sprinting-top",
        ),
        collection(typed)
    )]
    pub(crate) female_adventurer_animations: Vec<Handle<AnimationClip2D>>,

    // world
    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/grass_land/main_autotiling.png")]
    pub(crate) grass_land: Handle<Image>,

    #[asset(path = "textures/grass_land/decorative.png")]
    pub grass_land_decorative: Handle<Image>,
}
