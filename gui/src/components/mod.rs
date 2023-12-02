use bevy::app::Plugin;

use self::{voxel::Voxel, world::WorldShape, storage::VoxelBuffer};

pub mod camera;
// pub mod cube;
pub mod world;
pub mod robot;
pub mod storage;
pub mod voxel;
pub mod material;
pub mod render;

/// Registers all resources and systems for simulating and rendering an editable and interactive voxel world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(VoxelBuffer::<Voxel, WorldShape>::new(WorldShape {}, Voxel::default()))
            .add_plugins(world::meshing::VoxelWorldMeshingPlugin)
            // ordering of plugin insertion matters here.
            // .add_plugins(terraingen::TerrainGeneratorPlugin)
            .add_plugins(world::generator::VoxelWorldTerrainGenPlugin)
            // ordering of plugin insertion matters here.
            .add_plugins(material::VoxelMaterialPlugin)
            .add_plugins(render::ChunkMaterialPlugin)
            .add_plugins(world::materials::VoxelWorldBaseMaterialsPlugin)
            // .add_plugins(world::render::VoxelWorldRenderPlugin)
            .add_plugins(bevy_atmosphere::plugin::AtmospherePlugin)
            .add_plugins(camera::CameraControllerPlugin);
            //.add_plugins(sky::InteractiveSkyboxPlugin);
    }
}