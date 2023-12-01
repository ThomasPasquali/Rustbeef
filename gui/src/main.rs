use macroquad::prelude as mq;
use macroquad::texture::Texture2D;
use nla_compass::NLACompass;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::{worldgenerator::Generator, World};
use world_generator::WorldGenerator;
use endless_heights::height;

mod nla_compass;
mod world_generator;

const TILE_SIZE: u32 = 32;
const MOVE_SPEED: f32 = 0.3;
const LOOK_SPEED: f32 = 0.1;

#[test]
fn test_main(){
    main();
}

#[macroquad::main("Rustbeef")]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = mq::vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = mq::vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = mq::vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: mq::Vec2 = mq::mouse_position().into();

    let mut grabbed = true;
    mq::set_cursor_grab(grabbed);
    mq::show_mouse(false);

    let tools: Vec<NLACompass> = Vec::new();
    let (world, spawn, conditions, score) = WorldGenerator {}.gen();
    let mut world = World::new(world, conditions, tools, 10.0);

    let bumpiness = 100;
    let scale = 10.0;
    let interpolation = 1.0;
    let stretch = 3.0;
    let wideness = 2.0;

    let height_map = height::create_height_map(world.dimension, bumpiness, scale, interpolation, stretch, wideness);
    height::bump_world(&mut world, height_map);
    
    let textures = vec![
        mq::Texture2D::from_file_with_format(include_bytes!("../assets/grass.png"), None),
        mq::Texture2D::from_file_with_format(include_bytes!("../assets/sand.png"), None),
        mq::Texture2D::from_file_with_format(include_bytes!("../assets/water.png"), None),
    ];

    loop {
        let delta = mq::get_frame_time();

        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            grabbed = true;
            mq::set_cursor_grab(grabbed);
            mq::show_mouse(!grabbed);
        }
        if mq::is_key_pressed(mq::KeyCode::Tab) || mq::is_key_pressed(mq::KeyCode::Escape) {
            grabbed = !grabbed;
            mq::set_cursor_grab(grabbed);
            mq::show_mouse(!grabbed);
        }
        if grabbed {
            if mq::is_key_down(mq::KeyCode::W) {
                position += front * MOVE_SPEED;
            }
            if mq::is_key_down(mq::KeyCode::S) {
                position -= front * MOVE_SPEED;
            }
            if mq::is_key_down(mq::KeyCode::A) {
                position -= right * MOVE_SPEED;
            }
            if mq::is_key_down(mq::KeyCode::D) {
                position += right * MOVE_SPEED;
            }
            if mq::is_key_down(mq::KeyCode::Space) {
                position.y += MOVE_SPEED;
            }
            if mq::is_key_down(mq::KeyCode::LeftShift) {
                position.y -= MOVE_SPEED;
            }

            let mouse_position: mq::Vec2 = mq::mouse_position().into();
            let mouse_delta = mouse_position - last_mouse_position;
            last_mouse_position = mouse_position;

            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = mq::vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize();
        }
        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        mq::set_camera(&mq::Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });
        mq::clear_background(mq::LIGHTGRAY);

        render_world(&world, &textures);
        mq::set_default_camera();

        mq::next_frame().await
    }
}

fn render_world(world: &World, textures: &Vec<mq::Texture2D>) {
    for (row, row_v) in world.map.iter().enumerate() {
        for (col, tile) in row_v.iter().enumerate() {
            for elevation in 0..=tile.elevation{
                match tile.tile_type {
                    ShallowWater => {
                        // render_cube(row, col, tile.elevation, &textures[2]);
                        render_cube(row, elevation, col, &textures[2]);
                    }
                    Sand => {
                        render_cube(row, elevation, col, &textures[1]);
                    },
                    Grass => {
                        render_cube(row,elevation, col, &textures[0]);
                    },
                    _ => {
                        render_cube(row, elevation, col, &textures[0]);
                    }
                }
            }
        }
    }
}

fn render_cube(x: usize, y: usize, z: usize, texture: &Texture2D) {
    mq::draw_cube(
        mq::vec3(x as f32, y as f32, z as f32),
        mq::vec3(1., 1., 1.),
        Some(texture),
        mq::WHITE,
    );
}