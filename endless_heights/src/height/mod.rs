use std::fmt::Display;
use crate::utils::*;
use crate::World;
use crate::MAP_SIZE;

use rand_distr::{Uniform, Distribution};
use rand::{Rng, thread_rng};

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

fn sample_gaussians(gaussians: &mut Vec<Gaussian>, bumpiness: u32, scale: f32, limit: usize, stretch: f32, mut wideness: f32){
    let mut rng = thread_rng();
    for _ in 0..bumpiness{
        let angle = std::f32::consts::PI * rng.gen_range(0.0..2.0);
        if stretch <= wideness{
            wideness = stretch - 0.5
        }
        let uniform_sigma = Uniform::<f32>::from(wideness..stretch);
        let uniform_mean = Uniform::<f32>::from(0.0..limit as f32);
        let mean_x = uniform_mean.sample(&mut rng);
        let mean_y = uniform_mean.sample(&mut rng);
        let sigma_x = uniform_sigma.sample(&mut rng);
        let sigma_y = uniform_sigma.sample(&mut rng);
        let mut sampled_scale: f32 = 1.0;
        if scale > 1.0{
            let uniform_scale = Uniform::<f32>::from(0.9..scale);
            sampled_scale = uniform_scale.sample(&mut rng);
        }

        gaussians.push(Gaussian::new(angle, sigma_y, sigma_x, mean_y, mean_x, sampled_scale));
    }
}
pub fn create_height_map(size: usize, bumpiness: u32, scale: f32, interpolation: f32, stretch: f32, wideness: f32) -> HeightMap{
    let mut gaussians = Vec::<Gaussian>::new();
    sample_gaussians(&mut gaussians, bumpiness, scale, size, stretch, wideness);

    let mut height_map = height_map!(0; (size, size));
    let mut elevations = Vec::<usize>::new();
    for i in 0..size{
        for j in 0..size{
            // let mut elevation: f32 = 0.0;
            // for gaussian in &gaussians{
            //     elevation = f32::max(elevation, gaussian.get_value_at(i as f32, j as f32));
            // }
            let mut elevation: usize = 0;
            let mut gaussian_values = Vec::<usize>::new();
            for gaussian in &gaussians{
                // elevation = f32::max(elevation, gaussian.get_value_at(i as f32, j as f32));
                gaussian_values.push(gaussian.get_value_at(i as f32, j as f32) as usize);
            }
            gaussian_values.sort();
            elevation = gaussian_values[gaussian_values.len() - 1];
            for v in 0..gaussian_values.len() - 1{
                elevation += (interpolation * v as f32) as usize;
            }
            elevations.push(elevation);
            let new_tile = ElevationTile{pos: Position { x: i, y: j }, elevation};
            height_map.0[i][j] = new_tile;
        }
    }
    elevations.sort();
    let min_elevation = elevations[0];
    for i in 0..size{
        for j in 0..size{
            height_map.0[i][j].elevation -= min_elevation;
        }
    }
    height_map
}

pub fn bump_world(world: &mut World, height_map: HeightMap){
    for i in 0..world.dimension{
        for j in 0..world.dimension{
            world.map[i][j].elevation = height_map.0[i][j].elevation;
        }
    }
}
// pub fn bump_world(world: &mut World, bumpiness: u32, scale: f32, interpolation: f32, stretch: f32, wideness: f32){
//     let mut gaussians = Vec::<Gaussian>::new();
//     sample_gaussians(&mut gaussians, bumpiness, scale, world.dimension, stretch, wideness);
//     for i in 0..world.dimension{
//         for j in 0..world.dimension{
//             let mut elevation: usize = 0;
//             let mut gaussian_values = Vec::<usize>::new();
//             for gaussian in &gaussians{
//                 // elevation = f32::max(elevation, gaussian.get_value_at(i as f32, j as f32));
//                 gaussian_values.push(gaussian.get_value_at(i as f32, j as f32) as usize);
//             }
//             gaussian_values.sort();
//             elevation = gaussian_values[gaussian_values.len() - 1];
//             for v in 0..gaussian_values.len() - 1{
//                 elevation += (interpolation * v as f32) as usize;
//             }
//             world.map[i][j].elevation = elevation;
//         }
//     }
// }

#[test]
fn test_height_map_plot(){
    let display_hm = create_height_map(DEFAULT_SIZE, 2, 100.0, 2.0, 10.0, 15.0);
    print!("{}", display_hm);
}


