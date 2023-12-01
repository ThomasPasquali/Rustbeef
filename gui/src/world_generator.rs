use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::worldgenerator::Generator;

pub struct WorldGenerator {}

const MAP_SIZE: usize = 50;

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32) {
        let mut rng = rand::thread_rng();
        let mut world = Vec::new();
        for _ in 0..MAP_SIZE {
            let mut row = Vec::new();
            for _ in 0..MAP_SIZE {
                row.push(Tile {
                    tile_type: match rng.gen_range(1..8) {
                        1 => TileType::Sand,
                        2 => TileType::ShallowWater,
                        _ => TileType::Grass
                    },
                    content: Rock(0),
                    elevation: 0,
                });
            }
            world.push(row);
        }
        (
            world,
            (0, 0),
            EnvironmentalConditions::new(&[WeatherType::Sunny], 1, 1).unwrap(),
            10.0
        )
    }
}
