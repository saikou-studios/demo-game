use bevy::prelude::States;

pub mod debug;
pub mod loading;
pub mod menu;
pub mod player;
pub mod ui;
pub mod world;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}
