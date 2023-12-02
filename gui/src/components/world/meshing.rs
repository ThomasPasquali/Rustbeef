use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

use crate::components::{storage::VoxelBuffer, voxel::Voxel, render::{MeshBuffers, mesh_buffer, ChunkMaterialSingleton}};

use super::{WorldShape, generator::TerrainGenSet};

fn process_mesh_tasks(mut commands: Commands, buffer: ResMut<VoxelBuffer<Voxel, WorldShape>>, mut meshes: ResMut<Assets<Mesh>>, material: Res<ChunkMaterialSingleton>) {
    let mut mesh_buffers = MeshBuffers::<Voxel, WorldShape>::new(WorldShape {});

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh_buffer(&buffer, &mut mesh_buffers, &mut mesh, 1.0);
    commands.spawn((MaterialMeshBundle {
        material: (**material).clone(),
        mesh: meshes.add(mesh),
        transform: Transform::from_translation(Vec3::from_array([0.0, 0.0, 0.0])),
        visibility: Visibility::Visible,
        ..Default::default()
    })
    // },
    // Aabb::from_min_max(Vec3::ZERO, Vec3::splat(WORLD_LENGTH as f32)))
);
}

/// The set of systems which asynchronusly mesh the chunks.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct WorldMeshingSet;

/// Handles the meshing of the chunks.
pub struct VoxelWorldMeshingPlugin;

impl Plugin for VoxelWorldMeshingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Startup,
            WorldMeshingSet.after(TerrainGenSet)
        );
        app.add_systems(
            Startup,
            process_mesh_tasks.in_set(WorldMeshingSet)
        );
    }
}
