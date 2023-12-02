use ndshape::ConstShape3u32;

pub const WORLD_LENGTH:u32 = 100;
pub type WorldShape = ConstShape3u32<WORLD_LENGTH, WORLD_LENGTH, WORLD_LENGTH>;

pub mod generator;
pub mod materials;
pub mod meshing;
pub mod render;