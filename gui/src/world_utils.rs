use std::any::TypeId;

use robotics_lib::{interface::Tools, world::{World, worldgenerator::Generator}};
use endless_heights::height;
use crate::world_generator::WorldGenerator;

pub struct DumbTool {}
impl Tools for DumbTool {
    fn check(&self, world: &mut World) -> Result<(), robotics_lib::utils::LibError> {
        Ok(())
    }
    fn id(&self) -> TypeId {
        TypeId::of::<DumbTool>()
    }
}

fn initialize_world() -> World {
    let tools: Vec<DumbTool> = Vec::new();
    let (world, spawn, conditions, score) = WorldGenerator {}.gen();
    let mut world = World::new(world, conditions, tools, 10.0);

    let bumpiness = 100;
    let scale = 10.0;
    let interpolation = 1.0;
    let stretch = 3.0;
    let wideness = 2.0;

    // let height_map = height::create_height_map(world.dimension, bumpiness, scale, interpolation, stretch, wideness);
    // height::bump_world(&mut world, height_map);
    world
}