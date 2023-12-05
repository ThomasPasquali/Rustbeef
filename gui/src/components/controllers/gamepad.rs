use std::f32::consts::FRAC_PI_2;

use bevy::{
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res, Resource},
    },
    input::{
        gamepad::{
            Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, GamepadEvent,
        },
        Axis, Input,
    },
    math::{Quat, Vec2, Vec3},
    transform::components::Transform,
};

use crate::components::camera::{CameraController, DEFAULT_CAMERA_SENS};

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        // the ID of the gamepad
        match &ev {
            GamepadEvent::Connection(info) => {
                match &info.connection {
                    bevy::input::gamepad::GamepadConnection::Connected(gamepad) => {
                        println!(
                            "New gamepad connected with ID: {:?}, name: {}",
                            info.gamepad.id, gamepad.name
                        );

                        // if we don't have any gamepad yet, use this one
                        if my_gamepad.is_none() {
                            commands.insert_resource(MyGamepad(info.gamepad));
                        }
                    }
                    bevy::input::gamepad::GamepadConnection::Disconnected => {
                        println!("Lost gamepad connection with ID: {:?}", info.gamepad.id);

                        // if it's the one we previously associated with the player,
                        // disassociate it:
                        if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                            if *old_id == info.gamepad {
                                commands.remove_resource::<MyGamepad>();
                            }
                        }
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

pub fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };
    let (mut controller, mut transform) = query.single_mut();

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        let mut direction = Vec3::ZERO;
        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize() * Vec3::new(1.0, 0., 1.0);
        let right = transform.rotation.mul_vec3(Vec3::X).normalize();

        let mut acceleration = 0.5f32;

        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        if left_stick_pos.x.abs() > 0.1 {
            direction.x += left_stick_pos.x;
        }
        if left_stick_pos.y.abs() > 0.1 {
            direction.z += -left_stick_pos.y;
        }

        let up = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::LeftTrigger2,
        };
        let down = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger2,
        };

        if buttons.pressed(up) {
            direction.y += 1.0;
        }
        if buttons.pressed(down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            // hardcoding 0.10 as a factor for now to not go zoomin across the world.
            transform.translation += direction.x * right * acceleration
                + direction.z * forward * acceleration
                + direction.y * Vec3::Y * acceleration;
        }
    }

    let mut delta = Vec2::ZERO;

    let south = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };
    let east = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };
    let north = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::North,
    };
    let west = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::West,
    };

    if buttons.pressed(south) {
        delta.y -= 5.0;
    }
    if buttons.pressed(north) {
        delta.y += 5.0;
    }
    if buttons.pressed(east) {
        delta.x += 5.0;
    }
    if buttons.pressed(west) {
        delta.x -= 5.0; }

    let mut new_pitch = delta.y.mul_add(DEFAULT_CAMERA_SENS, controller.pitch);
    let new_yaw = delta.x.mul_add(-DEFAULT_CAMERA_SENS, controller.yaw);

    new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    controller.yaw = new_yaw;
    controller.pitch = new_pitch;

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, new_yaw) * Quat::from_axis_angle(-Vec3::X, new_pitch);
}
