use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkIntCell;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStart;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
}

#[derive(Component, Clone)]
pub struct Player {}
