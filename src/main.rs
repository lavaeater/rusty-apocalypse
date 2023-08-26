
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rand::plugin::EntropyPlugin;
use bevy_xpbd_2d::prelude::*;
use big_brain::{BigBrainPlugin, BigBrainSet};
use systems::*;
use crate::components::{Health, QuadCoord, QuadStore};
use rand_chacha::ChaCha8Rng;
use boids::ai::{attack_and_eat_action_system, find_prey_action_system, Hunger, hunger_scorer_system, hunger_system, hunt_prey_action_system, HuntTarget};
use boids::components::{BoidDirection, BoidStuff};
use boids::systems::{boid_steering, quad_boid_flocking, spawn_boids};
use components::control::PlayerControl;
use systems::camera::camera_follow;
use systems::input::{add_mouse_aim_line, draw_mouse_aim, keyboard_input, mouse_look, mouse_position};
use systems::movement::{linear_velocity_control_boid, linear_velocity_control_player};
use systems::player::spawn_player;
use systems::startup::{load_background, spawn_camera};

mod components;
mod systems;
mod boids;

const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;
const CAMERA_SCALE: f32 = 1.0;
const FIXED_TIME_STEP: f32 = 1.0 / 10.0;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(ShapePlugin)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .insert_resource(QuadStore(HashMap::new()))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .register_type::<PlayerControl>()
        .register_type::<BoidDirection>()
        .register_type::<BoidStuff>()
        .register_type::<QuadCoord>()
        .register_type::<HuntTarget>()
        .register_type::<Hunger>()
        .register_type::<Health>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(Startup,
                     load_background)
        .add_systems(Startup,
                     spawn_camera)
        .add_systems(Startup,
                     spawn_player)
        .add_systems(Startup,
                     spawn_boids)
        .add_systems(Startup,
                     add_mouse_aim_line)
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
        .add_systems(Update, hunger_system)
        .add_systems(FixedUpdate, quad_boid_flocking)
        .add_systems(FixedUpdate, naive_quad_system)
        .add_systems(
            PreUpdate,
            (
                (find_prey_action_system, hunt_prey_action_system, attack_and_eat_action_system).in_set(BigBrainSet::Actions),
                hunger_scorer_system.in_set(BigBrainSet::Scorers),
            ),
        ).run();
}

#[derive(PhysicsLayer)]
enum Layer {
    Player,
    Boid,
    Walls,
    Water,
}
