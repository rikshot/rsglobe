use bevy::{pbr::DirectionalLightShadowMap, prelude::*};

use bevy_panorbit_camera::PanOrbitCameraPlugin;
use rsglobe::{asset_loaded, setup};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DirectionalLightShadowMap { size: 1024 * 128 })
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.1,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, asset_loaded)
        .run();
}
