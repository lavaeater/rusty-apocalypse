use bevy::prelude::{Commands, default, Entity, Query, Res, ResMut, SpriteBundle, Transform, With};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody, Rotation};
use bevy::math::{Vec2, Vec3};
use bevy::asset::AssetServer;
use bevy_rand::prelude::GlobalEntropy;
use rand_chacha::ChaCha8Rng;
use big_brain::actions::Steps;
use big_brain::prelude::Thinker;
use big_brain::pickers::FirstToScore;
use bevy::core::Name;
use bevy_xpbd_2d::math::Vector2;
use rand::Rng;
use std::ops::AddAssign;
use crate::components::{QuadCoord, QuadStore};
use crate::{Layer, METERS_PER_PIXEL};
use crate::boids::ai::{AttackAndEat, FindPrey, Hunger, Hungry, Hunt};
use crate::boids::components::{Boid, BoidAttack, BoidDirection, BoidStuff};

pub fn spawn_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>
) {
    for n in 0..5 {
        let x = rng.gen_range(-15.0..15.0);
        let y = rng.gen_range(-15.0..15.0);
        let hunt_and_eat = Steps::build()
            .label("Hunt And Eat")
            // Try to find prey...
            .step(FindPrey {})
            // ...hunting it...
            .step(Hunt {})
            // ...and eating it.
            .step(AttackAndEat { per_second: 10.0 });

        let thinker = Thinker::build()
            .label("Boid Thinker")
            .picker(FirstToScore { threshold: 0.8 })
            // Technically these are supposed to be ActionBuilders and
            // ScorerBuilders, but our Clone impls simplify our code here.
            .when(
                Hungry,
                hunt_and_eat,
            );

        commands
            .spawn((
                Name::from("Boid ".to_string() + &n.to_string()),
                Hunger::new(75.0, 2.0),
                BoidDirection {
                    force_scale: 5.0,
                    direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).try_normalize().unwrap_or(Vec2::Y),
                    ..default()
                },
                thinker,
                Boid {},
                BoidAttack {
                    max_damage: (5..rng.gen_range(10..=20)),
                    cool_down: 0.0,
                    cool_down_default: rng.gen_range(1.0..=3.0),
                    skill_level: rng.gen_range(15..=75),
                },
                QuadCoord::default(),
                BoidStuff {
                    separation_factor: rng.gen_range(0.25..1.0),
                    cohesion_factor: rng.gen_range(0.25..1.0),
                    alignment_factor: rng.gen_range(0.25..1.0),
                    turn_speed: rng.gen_range(0.01..0.25),
                    ..default()
                },
                SpriteBundle {
                    transform: Transform::from_xyz(
                        x,
                        y,
                        2.0,
                    )
                        .with_scale(Vec3::new(
                            METERS_PER_PIXEL,
                            METERS_PER_PIXEL,
                            1.0,
                        )),
                    texture: asset_server.load("sprites/boid.png"),
                    ..default()
                },
                RigidBody::Kinematic,
                Position::from(Vec2 {
                    x,
                    y,
                }),
                Collider::triangle(
                    Vec2::new(0.0, 8.0 * METERS_PER_PIXEL),
                    Vec2::new(8.0 * METERS_PER_PIXEL, -8.0 * METERS_PER_PIXEL),
                    Vec2::new(-8.0 * METERS_PER_PIXEL, -8.0 * METERS_PER_PIXEL)),
                CollisionLayers::new([Layer::Boid], [Layer::Player]),
            ));
    }
}

pub fn boid_steering(mut query: Query<(
    &mut BoidDirection,
    &mut Rotation,
    &BoidStuff,
    &Transform,
    &Position), With<Boid>>) {
    let mut iter = query.iter_mut();
    while let Some((mut direction_control, mut rotation, boid_stuff, transform, position)) = iter.next() {
        direction_control.up = Vec2::new(transform.up().x, transform.up().y);
        let cohesion_direction = (boid_stuff.flock_center - position.0).normalize_or_zero() * boid_stuff.cohesion_factor;
        let separation_direction = if boid_stuff.separation_boids > 0 { boid_stuff.separation_vector.normalize_or_zero() * boid_stuff.separation_factor } else { Vec2::ZERO };
        let alignment_direction = if boid_stuff.alignment_boids > 0 { boid_stuff.alignment_direction * boid_stuff.alignment_factor } else { Vec2::ZERO };
        let desired_direction = boid_stuff.desired_direction * boid_stuff.desired_factor;
        direction_control.direction = direction_control.direction.lerp(((cohesion_direction + separation_direction + alignment_direction + desired_direction) / 4.0).normalize_or_zero(), boid_stuff.turn_speed);

        //We skip this lerp, because it is silly
        let target_up = direction_control.direction.clone(); // direction_control.up.lerp(direction_control.direction, 0.5);
        let to_add = Rotation::from_radians(
            target_up
                .angle_between(
                    direction_control
                        .direction
                )
        );
        rotation.add_assign(to_add);
    }
}

pub fn quad_boid_flocking(
    mut query: Query<(
        Entity,
        &Position,
        &QuadCoord,
        &mut BoidStuff)>,
    other_query: Query<(&Position, &BoidDirection)>,
    quad_store: Res<QuadStore>,
) {
    let mut iter = query.iter_mut();
    while let Some((entity, position, quad_coord, mut boid_stuff)) = iter.next() {
        boid_stuff.flock_center = Vector2::ZERO;
        boid_stuff.cohesion_boids = 0;
        boid_stuff.separation_vector = Vector2::ZERO;
        boid_stuff.separation_boids = 0;
        boid_stuff.alignment_boids = 0;
        boid_stuff.alignment_direction = Vector2::ZERO;

        let quad_coords =
            (-1..=1).map(|x|
                (-1..=1).map(move |y|
                    QuadCoord::new(quad_coord.x + x, quad_coord.y + y))).flatten().collect::<Vec<_>>();

        let others = quad_coords
            .iter()
            .filter_map(|coord|
                quad_store.0.get(coord)
            ).flatten().collect::<Vec<_>>();

        for other in others {
            if !entity.eq(other) {
                if let Ok((other_position, other_boid_direction)) = other_query.get(*other) {
                    let delta: Vec2 = other_position.0 - position.0;
                    let distance_sq: f32 = delta.length_squared();
                    if distance_sq < boid_stuff.cohesion_distance {
                        // cohesion
                        boid_stuff.flock_center += other_position.0;
                        boid_stuff.cohesion_boids += 1;

                        if distance_sq < boid_stuff.separation_distance {
                            boid_stuff.separation_vector += delta * -1.0;
                            boid_stuff.separation_boids += 1;
                        }
                    }
                    if distance_sq < boid_stuff.alignment_distance {
                        boid_stuff.alignment_direction += other_boid_direction.direction;
                        boid_stuff.alignment_boids += 1;
                    }
                }
            }
        }

        if boid_stuff.cohesion_boids > 0 {
            boid_stuff.flock_center = boid_stuff.flock_center / boid_stuff.cohesion_boids as f32;
        }
        if boid_stuff.separation_boids > 0 {
            boid_stuff.separation_vector = boid_stuff.separation_vector / boid_stuff.separation_boids as f32;
        }
        if boid_stuff.alignment_boids > 0 {
            boid_stuff.alignment_direction = boid_stuff.alignment_direction / boid_stuff.alignment_boids as f32;
        }
    }
}
