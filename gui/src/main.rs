use bevy::{prelude as bv, hierarchy::BuildChildren, pbr::{self, AmbientLight}};
use components::{camera::{camera_controller, CameraController}, cube::create_cube_mesh};

mod world_generator;
mod world_utils;
mod components;
#[test]
fn test_main(){
    main();
}

fn main() {
    bv::App::new()
        .add_plugins(bv::DefaultPlugins)
        .add_systems(bv::Startup, setup)
        .add_systems(
            bv::Update,
            camera_controller
        )
        .run();
}


fn setup(
    mut commands: bv::Commands,
    asset_server: bv::ResMut<bv::AssetServer>,
    mut materials: bv::ResMut<bv::Assets<bv::StandardMaterial>>,
    mut meshes: bv::ResMut<bv::Assets<bv::Mesh>>,
) {
    // camera
    commands.spawn((
        bv::Camera3dBundle {
            transform: bv::Transform::from_xyz(-3.0, 3.0, -3.0)
                .looking_at(bv::Vec3::new(-3.0, 3.0, 0.0), bv::Vec3::Y),
            ..bv::default()
        },
        CameraController {
            pitch: 0.0,
            yaw: std::f32::consts::PI, // -bv::Vec3::new(-3.0, 3.0, -3.0).angle_between(bv::Vec3::X),
            ..bv::default()
        },
        pbr::ShadowFilteringMethod::Hardware2x2,
    ));

    let style = bv::TextStyle {
        font_size: 20.,
        ..bv::default()
    };
    commands.insert_resource(AmbientLight{color: bv::Color::WHITE, brightness: 0.5});
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
            c.spawn(bv::TextBundle::from_sections([
                bv::TextSection::new("test\n", style.clone()),
            ]));
        });
    
    // Import the custom texture.
    let custom_texture_handle: bv::Handle<bv::Image> = asset_server.load("array_texture.png");
    // Create and save a handle to the mesh.
    let cube_mesh_handle: bv::Handle<bv::Mesh> = meshes.add(create_cube_mesh());
    let mut blocks = vec![];
    for x in 0..10 {
        for y in 0..10 {
            blocks.push(
                bv::PbrBundle {
                    mesh: cube_mesh_handle.clone(),
                    material: materials.add(bv::StandardMaterial {
                        base_color_texture: Some(custom_texture_handle.clone()),
                        ..bv::default()
                    }),
                    transform: bv::Transform {
                        translation: bv::Vec3::from([x as f32,0.0, y as f32]),
                        ..bv::default()
                    },
                    ..bv::default()
                }
            )
        }
    }
    commands.spawn_batch(blocks);
}