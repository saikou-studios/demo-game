use bevy::math::{IVec2, UVec2, Vec2};
use bevy_ecs_tilemap::map::TilemapTileSize;
use noise::{NoiseFn, Perlin};

pub(crate) const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
pub(crate) const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };

pub(crate) fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = IVec2::new(
        (camera_pos.x + (TILE_SIZE.x / 2.)) as i32,
        (camera_pos.y + (TILE_SIZE.y / 2.)) as i32,
    );
    IVec2::new(
        (camera_pos.x as f32 / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32,
        (camera_pos.y as f32 / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32,
    )
}

pub(crate) fn get_perlin_noise_for_pos(pos: (f64, f64), seed: u32) -> f64 {
    let n1 = Perlin::new(1 + seed);
    let n2 = Perlin::new(2 + seed);
    let n3 = Perlin::new(3 + seed);
    let base_octave = 1. / 200.;
    let e1 = (n1.get([pos.0 * base_octave, pos.1 * base_octave]) + 1.) / 2.;
    let e2 = (n2.get([pos.0 * base_octave * 8., pos.1 * base_octave * 8.]) + 1.) / 2.;
    let e3 = (n3.get([pos.0 * base_octave * 32., pos.1 * base_octave * 32.]) + 1.) / 2.;
    f64::min(e1, f64::min(e2, e3) + 0.1).clamp(0., 1.)
}
