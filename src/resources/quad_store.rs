use bevy::prelude::{Entity, Resource};
use bevy::utils::{HashMap, HashSet};
use crate::components::quads::QuadCoord;

#[derive(Resource)]
pub struct QuadStore(pub HashMap<QuadCoord, HashSet<Entity>>);
