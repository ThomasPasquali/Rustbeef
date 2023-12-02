use bevy::{ecs::{system::{Commands, ResMut, Res}, schedule::SystemSet}, render::{mesh::Mesh, primitives::Aabb, render_resource::PrimitiveTopology, view::Visibility}, app::{Startup, Plugin}, pbr::MaterialMeshBundle, math::Vec3, asset::Assets, transform::components::Transform};
use bevy::prelude::IntoSystemSetConfigs;
use bevy::prelude::IntoSystemConfigs;
use crate::components::render::ChunkMaterialSingleton;

use super::{meshing::WorldMeshingSet, WORLD_LENGTH};

fn render_world(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<ChunkMaterialSingleton>
) {
    commands.spawn((MaterialMeshBundle {
            material: (**material).clone(),
            mesh: meshes.add(Mesh::new(PrimitiveTopology::TriangleList)),
            transform: Transform::from_translation(Vec3::from_array([0.0, 0.0, 0.0])),
            visibility: Visibility::Visible,
            ..Default::default()
        })
        // },
        // Aabb::from_min_max(Vec3::ZERO, Vec3::splat(WORLD_LENGTH as f32)))
    );
    println!("{:?}", meshes.add(Mesh::new(PrimitiveTopology::TriangleList)));
}

/// The set of systems which asynchronusly mesh the chunks.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct RenderSet;

/// Handles the meshing of the chunks.
pub struct VoxelWorldRenderPlugin;

impl Plugin for VoxelWorldRenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Startup,
            RenderSet.after(WorldMeshingSet)
        );
        app.add_systems(
            Startup,
            render_world.in_set(RenderSet)
        );
    }
}
