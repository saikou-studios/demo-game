use crate::world::helpers::{get_perlin_noise_for_pos, CHUNK_SIZE};
use bevy::app::{App, Plugin};
use bevy::math::IVec2;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
}

// Grass, Dirt, Water
const FREQUENCIES: [f64; 3] = [0.0, 0.32, 0.15];

fn sample_tile_type(x: f64, y: f64, seed: u32) -> TileType {
    let e = get_perlin_noise_for_pos((x, y), seed);
    if e <= FREQUENCIES[2] {
        TileType::Water
    } else if e <= FREQUENCIES[1] {
        TileType::Dirt
    } else {
        TileType::Grass
    }
}

pub(crate) fn get_tile_from_perlin_noise(
    chunk_pos: IVec2,
    tile_pos: TilePos,
    seed: u32,
) -> [TileType; 4] {
    let nx = tile_pos.x as f64 + chunk_pos.x as f64 * CHUNK_SIZE.x as f64;
    let ny = tile_pos.y as f64 + chunk_pos.y as f64 * CHUNK_SIZE.y as f64;
    // Define sampling points relative to the current position
    let offsets = [(-0.5, 0.5), (0.5, 0.5), (-0.5, -0.5), (0.5, -0.5)];
    offsets.map(|(dx, dy)| sample_tile_type(nx + dx, ny + dy, seed))
}

pub(crate) fn tile_type_to_texture_index(tile_type: TileType) -> TileTextureIndex {
    match tile_type {
        // light grass => 53, grass => 63, dark grass => 73?
        TileType::Grass => TileTextureIndex(63),
        TileType::Dirt => TileTextureIndex(83),
        TileType::Water => TileTextureIndex(247),
    }
}

pub(crate) fn determine_predominant_tile_type(blocks: &[TileType; 4]) -> TileType {
    use std::collections::HashMap;
    let mut type_counts = HashMap::new();
    for &block_type in blocks {
        *type_counts.entry(block_type).or_insert(0) += 1;
    }
    // Correctly find and return the TileType with the highest count
    type_counts.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(t, _)| t) // Directly use t without dereferencing
        .unwrap_or(TileType::Grass) // Provide a default value in case blocks is empty or another error occurs
}
