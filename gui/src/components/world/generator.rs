use bevy::app::Startup;
use bevy::prelude::IntoSystemConfigs;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::SystemSet,
        system::{Res, ResMut},
    },
};
use ilattice::{extent::Extent, glam::UVec3};

use crate::components::{material::VoxelMaterial, storage::VoxelBuffer, voxel::Voxel};

use super::{materials::Bedrock, WorldShape, WORLD_LENGTH};

/// Generate the world bottom border for a chunk.
pub fn generate_world(mut buffer: ResMut<VoxelBuffer<Voxel, WorldShape>>) {
    buffer.fill_extent(
        Extent::from_min_and_shape(UVec3::ZERO, UVec3::new(WORLD_LENGTH, 2, WORLD_LENGTH)),
        Bedrock::into_voxel(),
    );
}

/// Handles terrain generation.
pub struct VoxelWorldTerrainGenPlugin;

/// The set of systems which asynchronusly mesh the chunks.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct TerrainGenSet;

impl Plugin for VoxelWorldTerrainGenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, 
            (generate_world).in_set(TerrainGenSet)
        );
    }
}
