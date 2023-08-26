pub(crate) mod player;
pub(crate) mod control;


use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Entity, Reflect, Resource, SpriteSheetBundle};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position};
use bevy_xpbd_2d::math::Vector2;
use bevy_xpbd_2d::prelude::{RigidBody, Rotation};
use player::Player;
use crate::{Layer, METERS_PER_PIXEL};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Water;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WaterBundle {
    water: Water,
}

#[derive(Debug, Component, Reflect, Clone)]
pub struct Health {
    pub health: i32,
    pub max: i32
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health: 100,
            max: 100,
        }
    }
}
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
        }
    }
}

#[derive(Component, Clone)]
pub struct Prey {}

#[derive(Component)]
pub struct AimLine {}

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


#[derive(Component)]
pub struct GameCam {}

#[derive(Component, Clone)]
pub struct CameraFollow {}

#[derive(Reflect)]
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct QuadCoord {
    pub x: i32,
    pub y: i32,
}

impl QuadCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn default() -> Self {
        Self { x: -15000, y: -15000 }
    }
}

#[derive(Resource)]
pub struct QuadStore(pub HashMap<QuadCoord, HashSet<Entity>>);


#[derive(Bundle, LdtkEntity)]
pub struct MapEntity {
    #[sprite_sheet_bundle]
    #[bundle()]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component)]
pub struct InWater {}

#[derive(Bundle, LdtkIntCell)]
pub struct IntCell {
    #[bundle()]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Debug, Reflect, Clone)]
pub struct AAName(pub String);


