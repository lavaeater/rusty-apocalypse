use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct BulletHitBoidEvent {
    pub bullet: Entity,
    pub boid: Entity,
}

#[derive(Event)]
pub struct BulletHitSomethingEvent {
    pub bullet: Entity,
    pub something: Entity,
}

#[derive(Event)]
pub struct BulletHitPlayerEvent {
    pub bullet: Entity,
    pub player: Entity,
}

#[derive(Event)]
pub struct BoidHitPlayerEvent {
    pub boid: Entity,
    pub player: Entity,
}