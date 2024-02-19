pub mod height;
pub mod utils;

use std::collections::HashMap;

use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::world_generator::Generator;
use strum::IntoEnumIterator;

/// # World Generator
/// 
/// The Endless Heights Generator generates a map with tiles of diverse elevations.
/// 
///  Elevations are sampled from a set of gaussians in the following way:
/// The elevation is primarily taken to be the highest value amongst all gaussians. All other gaussians are then added to the elevation with a multiplicative of an interpolation between 0 and 1.
/// ## Parameters
/// - map_size: The size of the square map
/// - amount_mountains: The amount of different gaussians to be spawned.
/// - scale: Scale of the gaussians.
/// - interpolation: The impact of gaussians behind the highest on the elevation.
/// - max_variance: The maximum variance in each direction to draw gaussians from.
/// - min_variance: The minimum variance in each direction to draw gaussians from.
pub struct WorldGenerator {
    map_size: usize,
    amount_mountains: usize,
    scale: f32,
    interpolation: f32,
    max_variance: f32,
    min_variance: f32,
}
impl WorldGenerator{
    pub fn new(map_size: usize, amount_mountains: usize, scale: f32, interpolation: f32, max_variance: f32, min_variance: f32) -> WorldGenerator {
        WorldGenerator{map_size, amount_mountains, scale, interpolation, max_variance, min_variance}
    }
}

type World = Vec<Vec<Tile>>;

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (World, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let mut rng = rand::thread_rng();
        let mut world = Vec::new();
        for _ in 0..self.map_size {
            let mut row = Vec::new();
            for _ in 0..self.map_size {
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

        let height_map =
            height::create_height_map(self.map_size, self.amount_mountains, self.scale, self.interpolation, self.min_variance, self.max_variance);
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
