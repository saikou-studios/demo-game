use bevy::prelude::States;

pub mod debug;
pub mod loading;
pub mod menu;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}
