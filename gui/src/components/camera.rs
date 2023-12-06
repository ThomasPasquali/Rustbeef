use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use std::f32::consts::FRAC_PI_2;

use super::controllers::{mouse::{handle_mouse_input, handle_mouse_move}, gamepad::{gamepad_input, gamepad_connections}};

// Reusing the player controller impl for now.

pub const DEFAULT_CAMERA_SENS: f32 = 0.005;

#[derive(Default, Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub cursor_locked: bool,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, SystemSet)]
/// Systems related to player controls.
pub struct PlayerControllerSet;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse_input, handle_mouse_move, gamepad_connections, gamepad_input).chain());
    }
}