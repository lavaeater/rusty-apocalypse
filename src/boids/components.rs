use bevy::prelude::{Bundle, Component, default, Reflect};
use bevy_xpbd_2d::math::Vector2;
use std::ops::Range;
use bevy::core::Name;
use bevy::math::{Vec2};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody};
use crate::components::{Health, QuadCoord};
use crate::{Layer, METERS_PER_PIXEL};

#[derive(Component, Clone)]
pub struct Boid {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct BoidStuff {
    pub cohesion_boids: i32,
    pub separation_boids: i32,
    pub desired_direction: Vector2,
    pub flock_center: Vector2,
    pub separation_vector: Vector2,
    pub alignment_direction: Vector2,
    pub separation_distance: f32,
    pub cohesion_distance: f32,
    pub desired_factor: f32,
    pub separation_factor: f32,
    pub cohesion_factor: f32,
    pub alignment_distance: f32,
    pub alignment_factor: f32,
    pub alignment_boids: i32,
    pub turn_speed: f32,
}

impl Default for BoidStuff {
    fn default() -> Self {
        Self {
            cohesion_boids: 0,
            separation_boids: 0,
            alignment_boids: 0,
            flock_center: Vector2::ZERO,
            separation_vector: Vector2::ZERO,
            alignment_direction: Vector2::ZERO,
            desired_direction: Vector2::ZERO,
            separation_distance: 25.0,
            cohesion_distance: 100.0,
            alignment_distance: 75.0,
            desired_factor: 1.0,
            separation_factor: 0.5,
            cohesion_factor: 0.5,
            alignment_factor: 0.7,
            turn_speed: 0.05,
        }
    }
}

#[derive(Reflect)]
#[derive(Clone, Debug, Component)]
pub struct BoidAttack {
    pub max_damage: Range<i32>,
    pub cool_down: f32,
    pub cool_down_default: f32,
    pub skill_level: i32
}


#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct BoidDirection {
    pub direction: Vector2,
    pub up: Vector2,
    pub force_scale: f32,
}

impl Default for BoidDirection {
    fn default() -> Self {
        Self {
            direction: Vector2::ZERO,
            up: Vector2::Y,
            force_scale: 10.0,
        }
    }
}

#[derive(Bundle)]
pub struct BoidBundle {
    pub name: Name,
    pub direction_control: BoidDirection,
    pub boid: Boid,
    pub health: Health,
    pub rigid_body: RigidBody,
    pub quad_coord: QuadCoord,
    pub position: Position,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub boid_attack: BoidAttack,
    pub boid_stuff: BoidStuff
}
impl BoidBundle {
    pub fn new(
        name: String,
        position: Vec2,
        direction: Vec2,
        max_damage: Range<i32>,
        cool_down_default: f32,
        skill_level: i32,
        boid_stuff: BoidStuff,
    ) -> Self {
        Self {
            name: Name::from(name),
            direction_control: BoidDirection {
                force_scale: 5.0,
                direction: direction.clone(),
                ..default()
            },
            boid: Boid {},
            boid_stuff,
            health: Health::default(),
             boid_attack: BoidAttack {
                 max_damage,
                 cool_down: 0.0,
                 cool_down_default,
                 skill_level,
             },
             rigid_body: RigidBody::Kinematic,
            quad_coord: QuadCoord::default(),
            position: Position::from(position),
            collider: Collider::cuboid(16.0 * METERS_PER_PIXEL, 8.0 * METERS_PER_PIXEL),
            collision_layers: CollisionLayers::new([Layer::Boid], [Layer::Player, Layer::Bullet]),
        }
    }
}