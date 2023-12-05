use bevy::{ecs::component::Component, math::IVec3};
use ndshape::ConstShape3u32;

use super::Voxel;

pub mod chunks;
pub mod chunks_anim;
pub mod materials;
pub mod meshing;
pub mod sky;
pub mod terrain;

pub const CHUNK_LENGTH: u32 = 32;
pub const CHUNK_LENGTH_U: usize = CHUNK_LENGTH as usize;
pub type ChunkShape = ConstShape3u32<CHUNK_LENGTH, CHUNK_LENGTH, CHUNK_LENGTH>;

// A component tagging an entity as a chunk.
#[derive(Component)]
pub struct Chunk(pub IVec3);
