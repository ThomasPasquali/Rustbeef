use std::any::TypeId;

use macroquad::prelude as mq;
use robotics_lib::interface::Tools;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::{worldgenerator::Generator, World};
use world_generator::WorldGenerator;

mod world_generator;

const TILE_SIZE: u32 = 32;
const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

struct DumbTool {}
impl Tools for DumbTool {
    fn check(&self, world: &mut World) -> Result<(), robotics_lib::utils::LibError> {
        Ok(())
    }
    fn id(&self) -> TypeId {
        TypeId::of::<DumbTool>()
    }
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

    let tools: Vec<DumbTool> = Vec::new();
    let (world, spawn, conditions, score) = WorldGenerator {}.gen();
    let world = World::new(world, conditions, tools, 10.0);
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
        for (col, col_v) in row_v.iter().enumerate() {
            match col_v.tile_type {
                Grass => {
                    mq::draw_cube(
                        mq::vec3(col as f32, col_v.elevation as f32, row as f32),
                        mq::vec3(1., 1., 1.),
                        Some(&textures[0]),
                        mq::WHITE,
                    );
                }
                Sand => {
                    mq::draw_cube(
                        mq::vec3(col as f32, col_v.elevation as f32, row as f32),
                        mq::vec3(1., 1., 1.),
                        Some(&textures[1]),
                        mq::WHITE,
                    );
                }
                _ => {
                    mq::draw_cube(
                        mq::vec3(col as f32, col_v.elevation as f32, row as f32),
                        mq::vec3(1., 1., 1.),
                        Some(&textures[2]),
                        mq::WHITE,
                    );
                }
            }
        }
    }
}
