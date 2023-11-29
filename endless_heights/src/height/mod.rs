use std::fmt::Display;
use crate::utils::*;

use rand_distr::{Uniform, Distribution};
use rand::{Rng, thread_rng};
use robotics_lib::world::World;


const DEFAULT_SIZE: usize = 10;
const WORLD_DIMENSION: Dimension = Dimension{ width: DEFAULT_SIZE, height: DEFAULT_SIZE };
// const MIN_MOUNTAIN_SIZE: Dimension = Dimension{width: 30, height: 30};
// const MIN_VALLEY_SIZE: Dimension = Dimension{width: 30, height: 30};

struct ElevationTile {
    pos: Position,
    elevation: usize,
    // expanded: bool
}

pub struct HeightMap(Vec<Vec<ElevationTile>>);
#[macro_export]
macro_rules! height_map {
    ($elevation:expr; ($rows:expr, $cols:expr)) => {{
        let mut temp_height_map = Vec::new();
        for i in 0..$rows{
            let mut next_row = Vec::new();
            for j in 0..$cols{
                next_row.push(ElevationTile{elevation: $elevation, pos: Position{x: i, y: j}});
            }
            temp_height_map.push(next_row);
        }
        HeightMap(temp_height_map)
    }};
}

impl Display for HeightMap{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("");
        for row in &self.0{
            for tile in row{
                out = Vec::from([out.clone(), tile.elevation.to_string()]).join(" ");
            }
            out.push_str("\n");

        }
        write!(f, "{}", out)
    }
}
struct Gaussian{
    angle: f32,
    sigma_y: f32,
    sigma_x: f32,
    mean_y: f32,
    mean_x: f32,
    scale: f32
}

impl Gaussian{
    fn new(angle: f32, sigma_y: f32, sigma_x: f32, mean_y: f32, mean_x: f32, scale: f32) -> Gaussian{
        assert!(sigma_x > 0.0);
        assert!(sigma_y > 0.0);
        Gaussian{angle, sigma_y, sigma_x, mean_y, mean_x, scale}
    }
    fn get_value_at(&self, x: f32, y: f32) -> f32{
        let a = f32::powf(f32::cos(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_x, 2.0)) + f32::powf(f32::sin(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_y, 2.0));
        let b = f32::sin(2.0 * self.angle) / (4.0 * f32::powf(self.sigma_x, 2.0)) - f32::sin(2.0 * self.angle) / (4.0 * f32::powf(self.sigma_y, 2.0));
        let c = f32::powf(f32::sin(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_x, 2.0)) + f32::powf(f32::cos(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_y, 2.0));
        self.scale * f32::exp(- ((a * f32::powf(x - self.mean_x, 2.0)) + 2.0 * b * (x - self.mean_x) * (y - self.mean_y) + c * f32::powf(y - self.mean_y, 2.0)))
    }
}
impl Default for Gaussian {
    fn default() -> Self {
        Self { angle: 0.0, sigma_y: 1.0, sigma_x: 1.0, mean_y: 0.0, mean_x: 0.0, scale: 1.0 }
    }
}

fn sample_gaussians(gaussians: &mut Vec<Gaussian>, bumpiness:u32, scale: f32){
    let mut rng = thread_rng();
    for _ in [0..bumpiness]{
        let angle = std::f32::consts::PI * rng.gen_range(0.0..2.0);
        let uniform_sigma = Uniform::<f32>::from(1.0..10.0);
        let uniform_mean = Uniform::<f32>::from(0.0..DEFAULT_SIZE as f32);
        let uniform_scale = Uniform::<f32>::from(0.0..scale);
        let sigma_x = uniform_sigma.sample(&mut rng);
        let sigma_y = uniform_sigma.sample(&mut rng);
        let mean_x = uniform_mean.sample(&mut rng);
        let mean_y = uniform_mean.sample(&mut rng);
        let sampled_scale = uniform_scale.sample(&mut rng);

        gaussians.push(Gaussian::new(angle, sigma_y, sigma_x, mean_y, mean_x, sampled_scale));
    }
}
pub fn create_height_map(bumpiness: u32, scale: f32) -> HeightMap{
    let mut gaussians = Vec::<Gaussian>::new();
    sample_gaussians(&mut gaussians, bumpiness, scale);

    let mut height_map = height_map!(0; (WORLD_DIMENSION.width, WORLD_DIMENSION.height));
    for i in 0..DEFAULT_SIZE{
        for j in 0..DEFAULT_SIZE{
            let mut elevation: f32 = 0.0;
            for gaussian in &gaussians{
                elevation = f32::max(elevation, gaussian.get_value_at(i as f32, j as f32));
            }
            let new_tile = ElevationTile{pos: Position { x: i, y: j }, elevation: elevation.floor() as usize};
            height_map.0[i][j] = new_tile;
        }
    }
    height_map
}

pub fn bump_world(world: &mut World, bumpiness: u32, scale: f32){
    let mut gaussians = Vec::<Gaussian>::new();
    sample_gaussians(&mut gaussians, bumpiness, scale);
    for i in 0..world.dimension{
        for j in 0..world.dimension{
            let mut elevation: f32 = 0.0;
            for gaussian in &gaussians{
                elevation = f32::max(elevation, gaussian.get_value_at(i as f32, j as f32));
            }
            world.map[i][j].elevation = elevation.floor() as usize;
        }
    }
}

#[test]
fn test_height_map_plot(){
    let display_hm = create_height_map(2, 100.0);
    print!("{}", display_hm);
}

