use crate::utils::*;
use crate::World;
use std::fmt::Display;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rand_distr::{Distribution, Uniform};

/// Struct to link a position to a certain elevation.
#[allow(dead_code)]
struct ElevationTile {
    pos: Position,
    elevation: usize,
}

/// A grid of ElevationTiles.
pub struct HeightMap(Vec<Vec<ElevationTile>>);
#[macro_export]
macro_rules! height_map {
    ($elevation:expr; ($rows:expr, $cols:expr)) => {{
        let mut temp_height_map = Vec::new();
        for i in 0..$rows {
            let mut next_row = Vec::new();
            for j in 0..$cols {
                next_row.push(ElevationTile {
                    elevation: $elevation,
                    pos: Position { x: i, y: j },
                });
            }
            temp_height_map.push(next_row);
        }
        HeightMap(temp_height_map)
    }};
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("");
        for row in &self.0 {
            for tile in row {
                out = Vec::from([out.clone(), tile.elevation.to_string()]).join(" ");
            }
            out.push_str("\n");
        }
        write!(f, "{}", out)
    }
}

/// A gaussian function with given parameters.
struct Gaussian {
    angle: f32,
    sigma_y: f32,
    sigma_x: f32,
    mean_y: f32,
    mean_x: f32,
    scale: f32,
}

impl Gaussian {
    fn new(
        angle: f32,
        sigma_y: f32,
        sigma_x: f32,
        mean_y: f32,
        mean_x: f32,
        scale: f32,
    ) -> Gaussian {
        assert!(sigma_x > 0.0);
        assert!(sigma_y > 0.0);
        Gaussian {
            angle,
            sigma_y,
            sigma_x,
            mean_y,
            mean_x,
            scale,
        }
    }
    /// Returns the value for coordinate x, y for the gaussian.
    fn get_value_at(&self, x: f32, y: f32) -> f32 {
        let a = f32::powf(f32::cos(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_x, 2.0))
            + f32::powf(f32::sin(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_y, 2.0));
        let b = f32::sin(2.0 * self.angle) / (4.0 * f32::powf(self.sigma_x, 2.0))
            - f32::sin(2.0 * self.angle) / (4.0 * f32::powf(self.sigma_y, 2.0));
        let c = f32::powf(f32::sin(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_x, 2.0))
            + f32::powf(f32::cos(self.angle), 2.0) / (2.0 * f32::powf(self.sigma_y, 2.0));
        self.scale
            * f32::exp(
                -((a * f32::powf(x - self.mean_x, 2.0))
                    + 2.0 * b * (x - self.mean_x) * (y - self.mean_y)
                    + c * f32::powf(y - self.mean_y, 2.0)),
            )
    }
}
impl Default for Gaussian {
    fn default() -> Self {
        Self {
            angle: 0.0,
            sigma_y: 1.0,
            sigma_x: 1.0,
            mean_y: 0.0,
            mean_x: 0.0,
            scale: 1.0,
        }
    }
}

/// Creates an array of different gaussians randomly sampled given the parameters.
fn sample_gaussians(
    gaussians: &mut Vec<Gaussian>,
    bumpiness: usize,
    scale: f32,
    limit: usize,
    min_variance: f32,
    max_variance: f32,
) {
    let mut rng = StdRng::seed_from_u64(2);
    for _ in 0..bumpiness {
        let angle = std::f32::consts::PI * rng.gen_range(0.0..2.0);

        let uniform_sigma = Uniform::<f32>::from(min_variance..max_variance);
        let uniform_mean = Uniform::<f32>::from(0.0..limit as f32);
        let mean_x = uniform_mean.sample(&mut rng);
        let mean_y = uniform_mean.sample(&mut rng);
        let sigma_x = uniform_sigma.sample(&mut rng);
        let sigma_y = uniform_sigma.sample(&mut rng);
        let mut sampled_scale: f32 = 1.0;
        if scale > 1.0 {
            let uniform_scale = Uniform::<f32>::from(0.9..scale);
            sampled_scale = uniform_scale.sample(&mut rng);
        }

        gaussians.push(Gaussian::new(
            angle,
            sigma_y,
            sigma_x,
            mean_y,
            mean_x,
            sampled_scale,
        ));
    }
}

/// Creates a square map of elevation tiles.
/// First an array of gaussians is drawn from sample_gaussians(). Given the set of different functions, each position is given an elevation based on the highest value amongst gaussians, plus a fraction of the other gaussians given 'interpolation'.
#[allow(unused_assignments)]
pub fn create_height_map(
    size: usize,
    bumpiness: usize,
    scale: f32,
    interpolation: f32,
    min_variance: f32,
    max_variance: f32,
) -> HeightMap {
    let mut gaussians = Vec::<Gaussian>::new();
    sample_gaussians(
        &mut gaussians,
        bumpiness,
        scale,
        size,
        min_variance,
        max_variance,
    );

    let mut height_map = height_map!(0; (size, size));
    let mut elevations = Vec::<usize>::new();
    for i in 0..size {
        for j in 0..size {
            let mut elevation: usize = 0;
            let mut gaussian_values = Vec::<usize>::new();
            for gaussian in &gaussians {
                gaussian_values.push(gaussian.get_value_at(i as f32, j as f32) as usize);
            }
            // Taking max value for each position
            gaussian_values.sort();
            elevation = gaussian_values[gaussian_values.len() - 1];

            // Adding some value of each other gaussian
            for v in 0..gaussian_values.len() - 1 {
                elevation += (interpolation * v as f32) as usize;
            }
            elevations.push(elevation);
            let new_tile = ElevationTile {
                pos: Position { x: i, y: j },
                elevation,
            };
            height_map.0[i][j] = new_tile;
        }
    }
    elevations.sort();
    let min_elevation = elevations[0];
    for i in 0..size {
        for j in 0..size {
            height_map.0[i][j].elevation -= min_elevation;
        }
    }
    height_map
}

pub fn bump_world(world: &mut World, height_map: HeightMap) {
    for i in 0..height_map.0.len() {
        for j in 0..height_map.0.len() {
            world[i][j].elevation = height_map.0[i][j].elevation;
        }
    }
}
