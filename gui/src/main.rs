use crate::components::WorldPlugin;
use bevy::{core_pipeline::fxaa::Fxaa, hierarchy::BuildChildren, prelude as bv, asset::{Handle, AssetServer}, text::Font, ecs::system::Res};

use std::f32::consts::PI;

mod components;
#[test]
fn test_main() {
    main();
}

const LEFT_ARROW:char = '🢀';
const RIGHT_ARROW:char = '🢂';
const UP_ARROW:char = '🢁';
const DOWN_ARROW:char = '🢃';

fn main() {
    bv::App::new()
        .add_plugins(bv::DefaultPlugins)
        .add_systems(bv::Startup, setup)
        .add_plugins(WorldPlugin)
        .run();
}

fn setup(mut commands: bv::Commands, server: Res<AssetServer>) {
    // camera
    commands
        .spawn(bv::Camera3dBundle {
            projection: bv::Projection::Perspective(bv::PerspectiveProjection {
                fov: PI / 2.,
                far: 2048.0,
                ..Default::default()
            }),
            transform: bv::Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(bv::Vec3::ZERO, bv::Vec3::Y),
            ..Default::default()
        })
        .insert(components::camera::CameraController::default())
        .insert(Fxaa::default())
        .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());

    commands.insert_resource(bv::AmbientLight {
        color: bv::Color::WHITE,
        brightness: 1.0,
    });
    let handle: Handle<Font> = server.load("NotoSansSymbols2-Regular.ttf");

    let style = bv::TextStyle {
        font_size: 60.,
        font: handle,
        ..bv::default()
    };
    commands
        .spawn(bv::NodeBundle {
            style: bv::Style {
                position_type: bv::PositionType::Absolute,
                padding: bv::UiRect::all(bv::Val::Px(5.0)),
                ..bv::default()
            },
            z_index: bv::ZIndex::Global(i32::MAX),
            background_color: bv::Color::BLACK.with_a(0.75).into(),
            ..bv::default()
        })
        .with_children(|c| {
            c.spawn(bv::TextBundle::from_sections([bv::TextSection::new(
                format!("{} {} {} {}", LEFT_ARROW, RIGHT_ARROW, UP_ARROW, DOWN_ARROW),
                style.clone(),
            )]));
        });
}
