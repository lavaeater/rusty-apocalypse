use bevy::prelude::{Component, Entity, Reflect, Resource};
use bevy::utils::{HashMap, HashSet};

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
