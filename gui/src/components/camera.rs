use std::f32::consts::PI;

use bevy::{prelude as bv, input::mouse::MouseMotion};

#[derive(bv::Component)]
pub struct Lights;

#[derive(bv::Component)]
pub struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub key_forward: bv::KeyCode,
    pub key_back: bv::KeyCode,
    pub key_left: bv::KeyCode,
    pub key_right: bv::KeyCode,
    pub key_up: bv::KeyCode,
    pub key_down: bv::KeyCode,
    pub key_run: bv::KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: bv::Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.2,
            key_forward: bv::KeyCode::W,
            key_back: bv::KeyCode::S,
            key_left: bv::KeyCode::A,
            key_right: bv::KeyCode::D,
            key_up: bv::KeyCode::Space,
            key_down: bv::KeyCode::ShiftLeft,
            key_run: bv::KeyCode::ControlLeft,
            walk_speed: 10.0,
            run_speed: 30.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: bv::Vec3::ZERO,
        }
    }
}

pub fn camera_controller(
    time: bv::Res<bv::Time>,
    mut mouse_events: bv::EventReader<MouseMotion>,
    key_input: bv::Res<bv::Input<bv::KeyCode>>,
    mut query: bv::Query<(&mut bv::Transform, &mut CameraController), bv::With<bv::Camera>>,
) {
    let dt = time.delta_seconds();

    // Handle mouse input
    let mut mouse_delta = bv::Vec2::ZERO;
    for mouse_event in mouse_events.read() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in &mut query {
        if !options.enabled {
            continue;
        }

        // Handle key input
        let mut axis_input = bv::Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        // Apply movement update
        if axis_input != bv::Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = bv::Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * bv::Vec3::Y
            + options.velocity.z * dt * forward;

        if mouse_delta != bv::Vec2::ZERO {
            // Apply look update
            options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * options.sensitivity * dt;
            transform.rotation = bv::Quat::from_euler(bv::EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    }
}