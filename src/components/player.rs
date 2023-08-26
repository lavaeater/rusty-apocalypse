use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkIntCell;
use bevy::core::Name;
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody};
use bevy::math::Vec2;
use crate::components::{CameraFollow, Health, Prey, QuadCoord};
use crate::components::control::PlayerControl;
use crate::{Layer, METERS_PER_PIXEL};
use crate::components::weapon::Weapon;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStart;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
}

#[derive(Component, Clone)]
pub struct Player {}

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    name: Name,
    camera_follow: CameraFollow,
    direction_control: PlayerControl,
    player: Player,
    health: Health,
    prey: Prey,
    rigid_body: RigidBody,
    quad_coord: QuadCoord,
    position: Position,
    collider: Collider,
    collision_layers: CollisionLayers,
    weapon: Weapon,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name::from("Player".to_string()),
            camera_follow: CameraFollow {},
            direction_control: PlayerControl::default(),
            player: Player {},
            health: Health::default(),
            prey: Prey {},
            rigid_body: RigidBody::Kinematic,
            quad_coord: QuadCoord::default(),
            position: Position::from(Vec2 {
                x: 0.0,
                y: 0.0,
            }),
            collider: Collider::cuboid(16.0 * METERS_PER_PIXEL, 8.0 * METERS_PER_PIXEL),
            collision_layers: CollisionLayers::new([Layer::Player], [Layer::Walls, Layer::Water]),
            weapon: Weapon::default(),
        }
    }
}
