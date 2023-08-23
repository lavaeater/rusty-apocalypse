use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Entity, Reflect, Resource, SpriteSheetBundle};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position};
use bevy_xpbd_2d::math::Vector2;
use bevy_xpbd_2d::prelude::{RigidBody, Rotation};
use big_brain::prelude::{ActionBuilder, ScorerBuilder};
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStart;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
}

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    name: Name,
    camera_follow: CameraFollow,
    direction_control: DirectionControl,
    player: Player,
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
            direction_control: DirectionControl::default(),
            player: Player {},
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
pub struct Player {}

#[derive(Component, Clone)]
pub struct Boid {}

#[derive(Component, Clone)]
pub struct Prey {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct BoidStuff {
    pub cohesion_boids: i32,
    pub separation_boids: i32,
    pub flock_center: Vector2,
    pub separation_vector: Vector2,
    pub separation_distance: f32,
    pub cohesion_distance: f32,
    pub separation_factor: f32,
    pub cohesion_factor: f32,
    pub alignment_direction: Vector2,
    pub alignment_distance: f32,
    pub alignment_factor: f32,
    pub alignment_boids: i32,
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
            separation_distance: 25.0,
            cohesion_distance: 100.0,
            alignment_distance: 75.0,
            separation_factor: 0.5,
            cohesion_factor: 0.5,
            alignment_factor: 0.7,
        }
    }
}

#[derive(Component)]
pub struct AimLine {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct DirectionControl {
    pub direction: Vector2,
    pub aim_direction: Vector2,
    pub up: Vector2,
    pub aim_rotation: Rotation,
    pub aim_degrees: f32,
    pub mouse_position: Vector2,
    pub force_scale: f32,
}

impl Default for DirectionControl {
    fn default() -> Self {
        Self {
            direction: Vector2::ZERO,
            aim_direction: Vector2::Y,
            up: Vector2::Y,
            aim_rotation: Rotation::default(),
            aim_degrees: 0.0,
            mouse_position: Vector2::ZERO,
            force_scale: 10.0,
        }
    }
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

#[derive(Component, Debug)]
pub struct Hunger {
    pub per_second: f32,
    pub hunger: f32,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Hunt {
    pub until: f32
}

impl Hunger {
    pub fn new(hunger: f32, per_second: f32) -> Self {
        Self { hunger, per_second }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Hungry;


