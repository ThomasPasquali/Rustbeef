pub mod height;
pub mod utils;

use std::collections::HashMap;
use std::iter::MapWhile;

use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::world_generator::Generator;
use strum::IntoEnumIterator;

pub struct WorldGenerator {}

pub const MAP_SIZE: usize = 50;

type World = Vec<Vec<Tile>>;

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (World, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let mut rng = rand::thread_rng();
        let mut world = Vec::new();
        for _ in 0..MAP_SIZE {
            let mut row = Vec::new();
            for _ in 0..MAP_SIZE {
                row.push(Tile {
                    tile_type: match rng.gen_range(0..TileType::iter().len()) {
                        1 => TileType::Sand,
                        _ => TileType::Grass,
                    },
                    content: Content::None,
                    elevation: 0,
                });
            }
            world.push(row);
        }

        // Create different elevations for each tile
        // PARAMETERS:
        let amount_mountains = (MAP_SIZE/5) as u32;
        let scale = MAP_SIZE as f32;
        let interpolation = 0.5;
        let max_variance = 20 as f32;
        let min_variance = 2 as f32;

        let height_map =
            height::create_height_map(MAP_SIZE, amount_mountains, scale, interpolation, min_variance, max_variance);
        height::bump_world(&mut world, height_map);

        (
            world,
            (0, 0),
            EnvironmentalConditions::new(&[WeatherType::Sunny], 1, 1).unwrap(),
            10.0,
            None
        )
    }
}
