use bevy::prelude::*;
use bevy_xpbd_2d::collision::Collision;
use bevy_xpbd_2d::components::LinearVelocity;
use bevy_xpbd_2d::prelude::ExternalForce;
use crate::boids::components::Boid;
use crate::components::{Health, Wall};
use crate::components::player::Player;
use crate::components::weapon::Projectile;
use crate::events::collisions::{BoidHitPlayerEvent, BulletHitBoidEvent, BulletHitPlayerEvent, BulletHitWallEvent};

pub fn bullet_hit_boid_listener(
    mut bullet_hit_boid_event_reader: EventReader<BulletHitBoidEvent>,
    mut commands: Commands,
    boid_query: Query<(&mut Health, &mut ExternalForce), With<Boid>>,
    bullet_query: Query<&LinearVelocity, With<Projectile>>,
) {
    for BulletHitBoidEvent { bullet, boid } in bullet_hit_boid_event_reader.iter() {
        if let Ok(linear_velocity) = bullet_query.get(*bullet) {
            let bullet_direction = linear_velocity.0.clone().normalize_or_zero();

            if let Ok(mut health) = boid_query.get_mut(*boid) {
                health.health -= 10;
                if health.health <= 0 {
                    commands.entity(*boid).despawn();
                }
            }
        }
        commands.entity(*bullet).despawn();
    }
}

pub fn collision_event_listener(
    mut collision_event_reader: EventReader<Collision>,
    player_query: Query<&Player>,
    boid_query: Query<&Boid>,
    bullet_query: Query<&Projectile>,
    wall_query: Query<&Wall>,
    mut ev_bullet_boid: EventWriter<BulletHitBoidEvent>,
    mut ev_bullet_wall: EventWriter<BulletHitWallEvent>,
    mut ev_bullet_player: EventWriter<BulletHitPlayerEvent>,
    mut ev_boid_player: EventWriter<BoidHitPlayerEvent>
) {
    for Collision(contact) in collision_event_reader.iter() {
        /*
        Top level first check: is it a bullet?
         */
        if bullet_query.contains(contact.entity1) || bullet_query.contains(contact.entity2) {
            let (bullet_entity, other_entity) = if bullet_query.contains(contact.entity1) {
                (&contact.entity1, &contact.entity2)
            } else {
                (&contact.entity2, &contact.entity1)
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
            } else if wall_query.contains(*other_entity) {
                ev_bullet_wall.send(BulletHitWallEvent {
                    bullet: *bullet_entity,
                    wall: *other_entity,
                })
            }
        } else if (player_query.contains(contact.entity1) || player_query.contains(contact.entity2)) &&
            (boid_query.contains(contact.entity1) || boid_query.contains(contact.entity2)) {
            /* This is boid on player hit */
            let (boid_entity, player_entity) = if boid_query.contains(contact.entity1) {
                (&contact.entity1, &contact.entity2)
            } else {
                (&contact.entity2, &contact.entity1)
            };
            ev_boid_player.send(BoidHitPlayerEvent {
                boid: *boid_entity,
                player: *player_entity,
            })
        }
    }
}