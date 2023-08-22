use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_xpbd_2d::prelude::*;
use systems::*;
use crate::components::{BoidDirection, BoidStuff, DirectionControl, QuadCoord, QuadStore};

mod components;
mod systems;

const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;
const CAMERA_SCALE: f32 = 1.0;
const FIXED_TIME_STEP: f32 = 1.0/10.0;
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(ShapePlugin)
        .insert_resource(QuadStore(HashMap::new()))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .register_type::<DirectionControl>()
        .register_type::<BoidDirection>()
        .register_type::<BoidStuff>()
        .register_type::<QuadCoord>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, load_background)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_boids)
        .add_systems(Startup, add_mouse_aim_line)
        .insert_resource(GizmoConfig {
            depth_bias: -1.0,
            ..default()
        })
        .add_systems(Update, camera_follow)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, mouse_position)
        .add_systems(Update, draw_mouse_aim)
        .add_systems(Update, mouse_look)
        .add_systems(Update, linear_velocity_control_player)
        .add_systems(Update, linear_velocity_control_boid)
        .add_systems(Update, boid_steering)
        .add_systems(FixedUpdate, quad_boid_flocking)
        .add_systems(FixedUpdate, naive_quad_system)
        .run();
}

#[derive(PhysicsLayer)]
enum Layer {
    Player,
    Boid,
    Walls,
    Water,
}
