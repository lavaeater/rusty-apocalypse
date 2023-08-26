use bevy::prelude::{Component, Reflect};
use bevy_xpbd_2d::math::Vector2;
use std::ops::Range;

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
