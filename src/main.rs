#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::enum_glob_use
)]

use bevy::{prelude::*, window::PresentMode};
use debug::Debug;

mod debug;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: true,
            ..Default::default()
        })
        // External plugins
        .add_plugins(DefaultPlugins)
        // Internal plugins
        .add_plugin(Debug)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_map)
        .add_startup_system(spawn_player)
        // .add_system(face_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = PerspectiveCameraBundle::new_3d();

    camera.transform.translation = Vec3::new(10.0, 13.0, 0.0);
    camera.transform.look_at(Vec3::new(1.0, 0.0, 0.0), Vec3::Y);

    commands.spawn_bundle(camera);
}

const MAP_UNIT_SIZE: f32 = 15.0;

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
            shape::Plane {
                size: MAP_UNIT_SIZE - 1.0,
            }
            .into(),
        ),
        material: materials.add(Color::GRAY.into()),
        ..default()
    });

    let mut wall_color = StandardMaterial::from(Color::DARK_GRAY);
    wall_color.reflectance = 0.0;
    let wall_color = materials.add(wall_color);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Box::new(MAP_UNIT_SIZE, 2.0, 0.5).into()),
        transform: Transform::from_xyz(0.0, 1.0, -MAP_UNIT_SIZE / 2.0 + 0.25),
        material: wall_color.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Box::new(MAP_UNIT_SIZE, 2.0, 0.5).into()),
        transform: Transform::from_xyz(0.0, 1.0, MAP_UNIT_SIZE / 2.0 - 0.25),
        material: wall_color.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Box::new(0.5, 2.0, MAP_UNIT_SIZE).into()),
        transform: Transform::from_xyz(-MAP_UNIT_SIZE / 2.0 + 0.25, 1.0, 0.0),
        material: wall_color.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Box::new(0.5, 2.0, MAP_UNIT_SIZE).into()),
        transform: Transform::from_xyz(MAP_UNIT_SIZE / 2.0 - 0.25, 1.0, 0.0),
        material: wall_color,
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::ANTIQUE_WHITE,
            intensity: 10_000.0,
            range: MAP_UNIT_SIZE - 1.0,
            radius: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 11.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FaceCamera;

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(shape::Plane { size: 1.0 }.into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert_bundle((Player, FaceCamera));
}

// fn face_camera(
//     mut transforms: Query<&mut Transform, (With<FaceCamera>, Changed<GlobalTransform>)>,
//     camera: Query<&GlobalTransform, With<Camera3d>>,
// ) {
//     let camera = camera.single();
//     for mut transform in transforms.iter_mut() {
//         let forward = Vec3::normalize(transform.translation - camera.translation);
//         transform.rotation = Quat::from_axis_angle(Vec3::X, angle)
//     }
// }
