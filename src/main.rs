use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use systems::*;

mod components;
mod systems;

const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .insert_resource(GizmoConfig {
            depth_bias: -1.0,
            ..default()
        })
        .add_systems(Update, camera_follow)
        .add_systems(Update, keyboard_input)
        .run();
}

#[derive(PhysicsLayer)]
enum Layer {
    Player,
    Walls,
    Water,
}
