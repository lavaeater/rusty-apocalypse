use bevy::prelude::*;
use bevy_xpbd_2d::collision::Collision;
use bevy_xpbd_2d::components::LinearVelocity;
use bevy_xpbd_2d::prelude::{CollisionStarted, ExternalForce};
use crate::boids::components::Boid;
use crate::components::general::{Health, Wall};
use crate::components::player::Player;
use crate::components::things_happening::{Lore, Place};
use crate::components::weapon::Projectile;
use crate::events::collisions::{BoidHitPlayerEvent, BulletHitBoidEvent, BulletHitPlayerEvent, BulletHitSomethingEvent};
use crate::events::facts::FactOccuredEvent;
use crate::resources::facts_of_the_world::Fact::BoolFact;

pub fn bullet_hit_boid_listener(
    mut bullet_hit_boid_event_reader: EventReader<BulletHitBoidEvent>,
    mut commands: Commands,
    mut boid_query: Query<(&mut Health, &mut ExternalForce), With<Boid>>,
    bullet_query: Query<&LinearVelocity, With<Projectile>>,
) {
    for BulletHitBoidEvent { bullet, boid } in bullet_hit_boid_event_reader.iter() {
        if let Ok(linear_velocity) = bullet_query.get(*bullet) {
            let _bullet_direction = linear_velocity.0.clone().normalize_or_zero();

            if let Ok((mut health, mut _external_force)) = boid_query.get_mut(*boid) {
                health.health -= 50;
                if health.health <= 0 {
                    commands.entity(*boid).despawn();
                }
            }
        }
        commands.entity(*bullet).despawn();
    }
}

pub fn bullet_hit_something_listener(
    mut bullet_hit_something_ev_reader: EventReader<BulletHitSomethingEvent>,
    mut commands: Commands,
) {
    for BulletHitSomethingEvent { bullet, something: _something } in bullet_hit_something_ev_reader.iter() {
        commands.entity(*bullet).despawn();
    }
}

pub fn collision_started_event_listener(
    mut collision_event_reader: EventReader<CollisionStarted>,
    player_query: Query<&Player>,
    boid_query: Query<&Boid>,
    bullet_query: Query<&Projectile>,
    wall_query: Query<&Wall>,
    place_query: Query<&Place, Option<&Lore>>,
    mut ev_bullet_boid: EventWriter<BulletHitBoidEvent>,
    mut ev_bullet_wall: EventWriter<BulletHitSomethingEvent>,
    mut ev_bullet_player: EventWriter<BulletHitPlayerEvent>,
    mut ev_boid_player: EventWriter<BoidHitPlayerEvent>,
    mut ev_fact_occured: EventWriter<FactOccuredEvent>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.iter() {
        /*
        Top level first check: is it a bullet?
         */
        if bullet_query.contains(*entity1) || bullet_query.contains(*entity2) {
            let (bullet_entity, other_entity) = if bullet_query.contains(*entity1) {
                (&*entity1, &*entity2)
            } else {
                (&*entity2, &*entity1)
            };

            if boid_query.contains(*other_entity) {
                ev_bullet_boid.send(BulletHitBoidEvent {
                    bullet: *bullet_entity,
                    boid: *other_entity,
                })
            } else if player_query.contains(*other_entity) {
                ev_bullet_player.send(BulletHitPlayerEvent {
                    bullet: *bullet_entity,
                    player: *other_entity,
                })
            } else if wall_query.contains(*other_entity) || place_query.contains(*other_entity) {
                ev_bullet_wall.send(BulletHitSomethingEvent {
                    bullet: *bullet_entity,
                    something: *other_entity,
                })
            }
        } else if player_query.contains(*entity1) || player_query.contains(*entity2) {
            let (player_entity, other_entity) = if player_query.contains(*entity1) {
                (&*entity1, &*entity2)
            } else {
                (&*entity2, &*entity1)
            };

            if boid_query.contains(*other_entity) {
                /* This is boid on player hit */
                ev_boid_player.send(BoidHitPlayerEvent {
                    boid: *other_entity,
                    player: *player_entity,
                })
            } else if let Ok(place) = place_query.get(*other_entity) {
                ev_fact_occured.send(FactOccuredEvent {
                    key: format!("PlaceVisited.{}", place.id),
                    fact: BoolFact(true),
                    fact_entity: Some(*other_entity),
                    acting_entity: Some(*player_entity),
                })
            }
        }
    }
}