use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct BulletHitBoidEvent {
    bullet: Entity,
    boid: Entity,
}

#[derive(Event)]
pub struct BulletHitWallEvent {
    bullet: Entity,
    wall: Entity,
}

#[derive(Event)]
pub struct BulletHitPlayerEvent {
    bullet: Entity,
    player: Entity,
}

#[derive(Event)]
pub struct BoidHitPlayerEvent {
    boid: Entity,
    player: Entity,
}