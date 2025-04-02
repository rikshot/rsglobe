use bevy::{pbr::DirectionalLightShadowMap, prelude::*};

use rsglobe::{asset_loaded, setup};
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::OrbitCameraPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DirectionalLightShadowMap { size: 1024 * 128 })
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.1,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, asset_loaded)
        .run();
}
