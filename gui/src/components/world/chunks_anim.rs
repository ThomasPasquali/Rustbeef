use bevy::{
    prelude::{
        Commands, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PostUpdate,
        Query, RemovedComponents, Res, SystemSet, Transform, Update, Visibility,
    },
    time::Time,
};

use super::{
    meshing::{ChunkMeshingSet, ChunkMeshingTask},
    Chunk,
};

fn attach_chunk_animation(
    mut ready_chunks: Query<(&mut Transform, &mut Visibility, &Chunk)>,
    mut removed_chunk_meshes: RemovedComponents<ChunkMeshingTask>,
    _time: Res<Time>,
    _commands: Commands,
) {
    removed_chunk_meshes.read().for_each(|entity| {
        if ready_chunks.contains(entity) {
            if let Ok((mut transform, mut visibility, chunk)) = ready_chunks.get_mut(entity) {
                *visibility = Visibility::Visible;
                transform.translation.y = chunk.0.y as f32;
            };
        }
    });
}

/// Animates the spawning of chunk entities that come into sight.
pub struct ChunkAppearanceAnimatorPlugin;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, SystemSet)]
pub struct ChunkAppearanceAnimatorSet;

impl Plugin for ChunkAppearanceAnimatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            PostUpdate,
            ChunkAppearanceAnimatorSet.after(ChunkMeshingSet),
        )
        .add_systems(
            Update,
            (attach_chunk_animation).in_set(ChunkAppearanceAnimatorSet),
        );
    }
}
