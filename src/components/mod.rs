pub(crate) mod player;
pub(crate) mod control;
pub(crate) mod weapon;
pub(crate) mod effects;

use bevy::prelude::{Bundle, Component, Entity, Reflect, Resource, SpriteSheetBundle};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};

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

#[derive(Component, Clone)]
pub struct Prey {}

#[derive(Component)]
pub struct AimLine {}


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

pub enum Rebuild {
    KeepQuadSize,
    ShrinkQuadSize,
    GrowQuadSize,
}

#[derive(Resource)]
pub struct QuadStore{
    pub entities: HashMap<QuadCoord, HashSet<Entity>>,
    pub quad_size: f32,
    pub max_quad_size: f32,
    pub min_quad_size: f32,
    pub max_entities: usize,
    pub min_entities: usize,
    pub largest_count: usize,
    pub rebuild_store: Rebuild,
}


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


