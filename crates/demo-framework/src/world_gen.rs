use crate::ui::diagnostics::DiagnosticsRoot;
use crate::{loading::TextureAssets, GameState};
use bevy::prelude::Without;
use bevy::text::Text;
use bevy::{
    app::{App, Plugin, Update},
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    math::{IVec2, UVec2, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        in_state, Camera, Commands, Component, Entity, IntoSystemConfigs, Query, Res, ResMut,
        Resource, Transform, With,
    },
    utils::HashSet,
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapRenderSettings, TilemapTexture, TilemapTileSize},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default()).add_systems(
            Update,
            (spawn_chunks_around_camera)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Component)]
pub struct Chunk;

pub fn spawn_chunk(commands: &mut Commands, texture_assets: &TextureAssets, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(53),
                        ..Default::default()
                    },
                    Chunk,
                ))
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));
    let texture_handle = texture_assets.grass_land.clone();
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform,
        render_settings: TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    camera_q: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for transform in camera_q.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &texture_assets, IVec2::new(x, y));
                }
            }
        }
    }
}

pub(crate) fn despawn_chunks_out_of_range(
    mut commands: Commands,
    camera_q: Query<&Transform, With<Camera>>,
    chunks_q: Query<(Entity, &Transform)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_q.iter() {
        for (entity, chunk_transform) in chunks_q.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > 480.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
