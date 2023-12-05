use bevy::app::Plugin;

use self::{storage::ChunkMap, voxel::Voxel};

/// Storage primitives for storing voxel data
pub mod storage;
/// Utils for managing a voxel world.
mod world;
pub use world::*;
pub mod camera;
/// Systems for defining voxel materials with physical properties.
pub mod material;
/// Systems and utilities for rendering voxels.
pub mod render;
pub mod terraingen;
mod voxel;
pub use voxel::*;
mod robot;

/// Registers all resources and systems for simulating and rendering an editable and interactive voxel world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ChunkMap::<Voxel, ChunkShape>::new(ChunkShape {}))
            .add_plugins(robot::RobotPlugin)
            .add_plugins(chunks::VoxelWorldChunkingPlugin)
            .add_plugins(meshing::VoxelWorldMeshingPlugin)
            // ordering of plugin insertion matters here.
            .add_plugins(terrain::VoxelWorldTerrainGenPlugin)
            .add_plugins(material::VoxelMaterialPlugin)
            .add_plugins(render::ChunkMaterialPlugin)
            .add_plugins(materials::VoxelWorldBaseMaterialsPlugin)
            .add_plugins(chunks_anim::ChunkAppearanceAnimatorPlugin)
            .add_plugins(bevy_atmosphere::plugin::AtmospherePlugin)
            .add_plugins(camera::CameraControllerPlugin)
            .add_plugins(sky::InteractiveSkyboxPlugin);
    }
}
