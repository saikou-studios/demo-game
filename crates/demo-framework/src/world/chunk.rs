use crate::camera::MainCamera;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::core::Name;
use bevy::ecs::reflect::ReflectComponent;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::{IVec2, Vec3, Vec3Swizzles};
use bevy::prelude::{
    in_state, Camera, Commands, Component, Entity, Event, EventReader, EventWriter,
    IntoSystemConfigs, Query, Res, SpatialBundle, Transform, With,
};
use bevy::reflect::Reflect;
use bevy::render::view::RenderLayers;
use bevy::utils::HashSet;
use bevy_ecs_tilemap::map::{TilemapId, TilemapRenderSettings, TilemapTexture};
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TilemapType};
use bevy_ecs_tilemap::tiles::TileStorage;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_magic_light_2d::gi::render_layer::CAMERA_LAYER_FLOOR;

use crate::world::helpers::{camera_pos_to_chunk_pos, CHUNK_SIZE, TILE_SIZE};
use crate::world::tile::{
    determine_predominant_tile_type, get_tile_from_perlin_noise, tile_type_to_texture_index,
};

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnChunkEvent>()
            .add_systems(
                Update,
                (
                    spawn_chunks_around_camera,
                    handle_spawn_chunk_event.after(spawn_chunks_around_camera),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                despawn_chunks_out_of_range.run_if(in_state(GameState::Playing)),
            );
    }
}

const CHUNKS_AROUND_CAMERA: IVec2 = IVec2 { x: 4, y: 4 };

pub fn spawn_chunks_around_camera(
    camera_q: Query<&Transform, With<MainCamera>>,
    chunk_q: Query<&Chunk>,
    mut spawn_chunk_event: EventWriter<SpawnChunkEvent>,
) {
    if let Ok(transform) = camera_q.get_single() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        let existing_chunks: HashSet<IVec2> = chunk_q.iter().map(|chunk| chunk.pos).collect();
        for x in (camera_chunk_pos.x - CHUNKS_AROUND_CAMERA.x)
            ..=(camera_chunk_pos.x + CHUNKS_AROUND_CAMERA.x)
        {
            for y in (camera_chunk_pos.y - CHUNKS_AROUND_CAMERA.y)
                ..=(camera_chunk_pos.y + CHUNKS_AROUND_CAMERA.y)
            {
                let pos = IVec2 { x, y };
                if !existing_chunks.contains(&pos) {
                    spawn_chunk_event.send(SpawnChunkEvent { pos });
                }
            }
        }
    }
}

pub fn despawn_chunks_out_of_range(
    mut commands: Commands,
    camera_q: Query<&Transform, With<Camera>>,
    chunk_q: Query<(&Transform, Entity), With<Chunk>>,
) {
    for camera_transform in camera_q.iter() {
        let max_dist = f32::hypot(
            CHUNK_SIZE.x as f32 * TILE_SIZE.x,
            CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        );
        for (chunk_transform, entity) in chunk_q.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let dist = camera_transform.translation.xy().distance(chunk_pos);
            // let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
            // let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
            if dist > max_dist * 2. * CHUNKS_AROUND_CAMERA.x as f32
                && dist > max_dist * 2. * CHUNKS_AROUND_CAMERA.y as f32
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn handle_spawn_chunk_event(
    mut commands: Commands,
    mut cache_events: EventReader<SpawnChunkEvent>,
    chunk_q: Query<&Chunk>,
    texture_assets: Res<TextureAssets>,
) {
    for event in cache_events.read() {
        let chunk_pos = event.pos;
        let existing_chunks: HashSet<IVec2> = chunk_q.iter().map(|chunk| chunk.pos).collect();
        if existing_chunks.contains(&chunk_pos) {
            continue;
        }

        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let tile_pos = TilePos { x, y };
                let blocks = get_tile_from_perlin_noise(chunk_pos, tile_pos, 238432);

                // let predominant_tile_type = determine_predominant_tile_type(&blocks);
                let texture_index = tile_type_to_texture_index(blocks[0]);
                // println!(
                //     "Predominant tile type: {:?} {:?}",
                //     predominant_tile_type, texture_index
                // );

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index,
                        ..Default::default()
                    })
                    .insert(RenderLayers::from_layers(CAMERA_LAYER_FLOOR))
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

        commands
            .entity(tilemap_entity)
            .insert(TilemapBundle {
                grid_size: TILE_SIZE.into(),
                map_type: TilemapType::Square,
                size: CHUNK_SIZE.into(),
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_assets.grass_land.clone()),
                tile_size: TILE_SIZE,
                transform,
                render_settings: TilemapRenderSettings {
                    render_chunk_size: CHUNK_SIZE * 2,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RenderLayers::from_layers(CAMERA_LAYER_FLOOR))
            .insert(Chunk { pos: chunk_pos })
            .insert(Name::new(format!("Chunk {:?}", chunk_pos)));
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    pub pos: IVec2,
}

#[derive(Event)]
pub struct SpawnChunkEvent {
    pub pos: IVec2,
}
