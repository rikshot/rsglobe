use bevy::prelude::*;

use smooth_bevy_cameras::{
    LookTransformPlugin,
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
};

use std::f32::consts::PI;

const FULL_TURN: f32 = 2.0 * PI;

const DAY: bool = true;
const DRAW_CLOUDS: bool = true;

#[derive(Component)]
struct Rotatable {
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
}

fn convert_lat_lon_to_vec3(lat: f32, lon: f32, height: f32) -> Vec3 {
    let cos_lat = f32::cos(lat * PI / 180.0);
    let sin_lat = f32::sin(lat * PI / 180.0);
    let cos_lon = f32::cos(lon * PI / 180.0);
    let sin_lon = f32::sin(lon * PI / 180.0);
    Vec3::new(
        height * cos_lat * cos_lon,
        height * cos_lat * sin_lon,
        height * sin_lat,
    )
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth_day = asset_server.load("textures/earth_day.jpg");
    let earth_night = asset_server.load("textures/earth_night.jpg");
    let earth_normal = asset_server.load("textures/earth_normal.png");
    let earth_spec = asset_server.load("textures/earth_specular.png");
    let earth_clouds = asset_server.load("textures/earth_clouds.png");

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.005, 5.0).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::RED),
            ..default()
        })),
    ));

    commands.spawn((
        Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        Mesh3d(meshes.add(Capsule3d::new(0.005, 5.0).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::GREEN),
            ..default()
        })),
    ));

    commands.spawn((
        Transform::from_rotation(Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_z(PI / 2.0)),
        Mesh3d(meshes.add(Capsule3d::new(0.005, 5.0).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::BLUE),
            ..default()
        })),
    ));

    let globe_mesh = Sphere::new(2.0).mesh().uv(64, 32);

    commands.spawn((
        Transform::from_rotation(Quat::from_rotation_z(PI)),
        Mesh3d(meshes.add(globe_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(if DAY { earth_day } else { earth_night }),
            normal_map_texture: Some(earth_normal),
            occlusion_texture: Some(earth_spec),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
    ));

    if DRAW_CLOUDS {
        commands
            .spawn((
                Transform::from_rotation(Quat::from_rotation_z(PI)),
                Mesh3d(
                    meshes.add(
                        Sphere::new(2.02)
                            .mesh()
                            .build()
                            .with_generated_tangents()
                            .unwrap(),
                    ),
                ),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(earth_clouds),
                    alpha_mode: AlphaMode::AlphaToCoverage,
                    ..default()
                })),
            ))
            .insert(Rotatable {
                speed_x: 0.005,
                speed_y: 0.0,
                speed_z: 0.0,
            });
    }

    let lat = 60.192_06;
    let lon = 24.945831;
    let pos = convert_lat_lon_to_vec3(lat, lon, 2.0);

    commands.spawn((
        Transform::from_translation(pos),
        Mesh3d(meshes.add(Sphere::new(0.01).mesh().ico(10).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::RED),
            ..default()
        })),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands
        .spawn((Camera3d::default(), Msaa::Sample4))
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

fn rotate(mut rotatables: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, rotatable) in rotatables.iter_mut() {
        let rotation_change =
            Quat::from_rotation_x(FULL_TURN * rotatable.speed_x * timer.delta_secs());
        transform.rotate(rotation_change);
        let rotation_change =
            Quat::from_rotation_y(FULL_TURN * rotatable.speed_y * timer.delta_secs());
        transform.rotate(rotation_change);
        let rotation_change =
            Quat::from_rotation_z(FULL_TURN * rotatable.speed_z * timer.delta_secs());
        transform.rotate(rotation_change);
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}
