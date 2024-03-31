mod chunk;
mod helpers;
mod tile;

use bevy::app::{App, Plugin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(chunk::ChunkPlugin);
    }
}
