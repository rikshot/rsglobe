use bevy::{
    core_pipeline::Skybox,
    pbr::{DirectionalLightShadowMap, ShadowFilteringMethod},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use smooth_bevy_cameras::{
    LookTransformPlugin,
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
};

use std::f32::consts::PI;

const DAY: bool = true;
const DRAW_CLOUDS: bool = true;
const DEBUG: bool = false;

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

#[derive(Resource, Clone)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth_day =
        asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/earth_day.jpg");
    let earth_night =
        asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/earth_night.jpg");
    let earth_normal =
        asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/earth_normal.png");
    let earth_spec =
        asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/earth_specular_inv.png");
    let earth_clouds =
        asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/earth_clouds.png");

    let skybox = asset_server.load("/Users/rikshot/Projects/rsglobe/assets/textures/skybox.png");
    let cubemap = Cubemap {
        is_loaded: false,
        image_handle: skybox.clone(),
    };
    commands.insert_resource(cubemap.clone());

    if DEBUG {
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
            Transform::from_rotation(
                Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_z(PI / 2.0),
            ),
            Mesh3d(meshes.add(Capsule3d::new(0.005, 5.0).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::LinearRgba(LinearRgba::BLUE),
                ..default()
            })),
        ));
    }

    let globe_mesh = Sphere::new(1.0).mesh().ico(64).unwrap();

    commands.spawn((
        Transform::from_rotation(Quat::from_rotation_z(PI)),
        Mesh3d(meshes.add(globe_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(if DAY { earth_day } else { earth_night }),
            normal_map_texture: Some(earth_normal),
            occlusion_texture: Some(earth_clouds.clone()),
            metallic: 1.0,
            perceptual_roughness: 1.0,
            metallic_roughness_texture: Some(earth_spec),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
    ));

    if DRAW_CLOUDS {
        commands.spawn((
            Transform::from_rotation(Quat::from_rotation_z(PI)),
            Mesh3d(meshes.add(Sphere::new(1.01).mesh().ico(64).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(earth_clouds.clone()),
                alpha_mode: AlphaMode::AlphaToCoverage,
                diffuse_transmission: 0.25,
                specular_transmission: 0.25,
                thickness: 0.01,
                ior: 1.02,
                ..default()
            })),
        ));
    }

    if DEBUG {
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
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands
        .spawn((
            Camera3d::default(),
            Msaa::Sample4,
            ShadowFilteringMethod::Gaussian,
        ))
        .insert(Skybox {
            image: cubemap.image_handle,
            brightness: 1000.0,
            ..default()
        })
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DirectionalLightShadowMap { size: 32768 })
        .add_plugins(DefaultPlugins)
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, asset_loaded)
        .run();
}
