use bevy::prelude::{Component, Reflect};
use bevy_xpbd_2d::math::Vector2;
use bevy_xpbd_2d::components::Rotation;

#[derive(Component, Clone)]
pub struct TriggerPulled {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct PlayerControl {
    pub direction: Vector2,
    pub aim_direction: Vector2,
    pub up: Vector2,
    pub aim_rotation: Rotation,
    pub aim_degrees: f32,
    pub mouse_position: Vector2,
    pub force_scale: f32,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            direction: Vector2::ZERO,
            aim_direction: Vector2::Y,
            up: Vector2::Y,
            aim_rotation: Rotation::default(),
            aim_degrees: 0.0,
            mouse_position: Vector2::ZERO,
            force_scale: 10.0
        }
    }
}
