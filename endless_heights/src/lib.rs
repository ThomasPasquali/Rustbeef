pub mod height;
pub mod utils;

use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::worldgenerator::Generator;
use strum::IntoEnumIterator;

pub struct WorldGenerator {}

const MAP_SIZE: usize = 50;

type World = Vec<Vec<Tile>>;

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (World, (usize, usize), EnvironmentalConditions, f32) {
        let mut rng = rand::thread_rng();
        let mut world = Vec::new();
        for _ in 0..MAP_SIZE {
            let mut row = Vec::new();
            for _ in 0..MAP_SIZE {
                row.push(Tile {
                    tile_type: match rng.gen_range(0..TileType::iter().len()) {
                        1 => TileType::Sand,
                        2 => TileType::ShallowWater,
                        _ => TileType::Grass,
                    },
                    content: Rock(0),
                    elevation: 0,
                });
            }
            world.push(row);
        }
        let bumpiness = 100;
        let scale = 10.0;
        let interpolation = 1.0;
        let stretch = 3.0;
        let wideness = 2.0;

        let height_map = height::create_height_map(world.dimension, bumpiness, scale, interpolation, stretch, wideness);
        height::bump_world(&mut world, height_map);
        (
            world,
            (0, 0),
            EnvironmentalConditions::new(&[WeatherType::Sunny], 1, 1).unwrap(),
            10.0
        )
    }
}
