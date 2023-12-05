use std::sync::RwLock;

use endless_heights::MAP_SIZE;
use ilattice::{extent::Extent};

use bevy::math::{IVec3, UVec3};
use once_cell::sync::Lazy;
use robotics_lib::world::tile::{Tile, TileType};

use crate::components::materials::{Sand, Water};

use super::{
    material::VoxelMaterial, materials::Grass, storage::VoxelBuffer, ChunkShape, Voxel,
    CHUNK_LENGTH_U,
};

// Terrain generator singleton.
pub static DISCOVERED_WORLD: Lazy<RwLock<TerrainGenerator>> = Lazy::new(Default::default);

#[derive(Default)]
pub struct TerrainGenerator {
    pub world: Vec<Vec<Tile>>,
}

impl TerrainGenerator {
    pub fn generate(&self, chunk_key: IVec3, buffer: &mut VoxelBuffer<Voxel, ChunkShape>) {
        if chunk_key.x < MAP_SIZE as i32 && chunk_key.x >= 0 && chunk_key.y == 0 {
            for (x, row) in self.world
                [chunk_key.x as usize..(chunk_key.x as usize + CHUNK_LENGTH_U).clamp(0, MAP_SIZE)]
                .iter()
                .enumerate()
            {
                if chunk_key.z <= MAP_SIZE as i32 && chunk_key.z >= 0 {
                    for (z, tile) in row[chunk_key.z as usize
                        ..(chunk_key.z as usize + CHUNK_LENGTH_U).clamp(0, MAP_SIZE)]
                        .iter()
                        .enumerate()
                    {
                        buffer.fill_extent(
                            Extent {
                                minimum: UVec3 {
                                    x: x as u32,
                                    y: 0,
                                    z: z as u32,
                                },
                                shape: UVec3::new(1, tile.elevation as u32 + 1, 1),
                            },
                            match tile.tile_type {
                                TileType::Grass => Grass::into_voxel(),
                                TileType::Sand => Sand::into_voxel(),
                                TileType::ShallowWater => Water::into_voxel(),
                                TileType::DeepWater => Water::into_voxel(),
                                _ => Grass::into_voxel(),
                            },
                        );
                    }
                }
            }
        }
    }
}
