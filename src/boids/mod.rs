use bevy::prelude::{Commands, Component, default, Entity, Query, Reflect, Res, ResMut, SpriteBundle, Time, Transform, With};
use big_brain::prelude::{ActionBuilder, ActionSpan, Actor, Score, ScorerBuilder, ScorerSpan, Thinker};
use bevy::log::{debug, trace};
use big_brain::actions::{ActionState, Steps};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody, Rotation};
use bevy::math::{Vec2, Vec3};
use bevy_xpbd_2d::math::Vector2;
use bevy::asset::AssetServer;
use big_brain::pickers::FirstToScore;
use bevy::core::Name;
use rand::Rng;
use std::ops::{AddAssign, Range};
use bevy_rand::resource::GlobalEntropy;
use rand_chacha::ChaCha8Rng;
use crate::components::{Health, Prey, QuadCoord, QuadStore};
use crate::{Layer, METERS_PER_PIXEL};

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

pub fn hunger_system(time: Res<Time>, mut hungers: Query<&mut Hunger>) {
    for mut hungry in &mut hungers {
        hungry.hunger += hungry.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if hungry.hunger >= 100.0 {
            hungry.hunger = 100.0;
        }
        trace!("Thirst: {}", hungry.hunger);
    }
}

pub fn attack_and_eat_action_system(
    mut query: Query<(&Actor, &mut ActionState, &AttackAndEat, &ActionSpan)>,
    mut boid_query: Query<(&HuntTarget, &mut BoidStuff, &mut BoidAttack, &mut Hunger, &Position)>,
    mut target_query: Query<(&mut Health, &Position)>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut commands: Commands
) {
    for (Actor(actor), mut state, _, _) in &mut query {
        /*
        Hunting, how is it done?
        Well, if we are using some kind of naïve grid sub-division system, I would say
        we simply check the grid square we are in and the neighbouring ones for things we can
        prey upon.

        If we don't find any prey, we move to some quadrant.

        Otherwise, I guess something happens.
         */
        match *state {
            ActionState::Requested => {
                debug!("Time to look for some prey!");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                trace!("Do we have a hunt target?");
                if let Ok((hunt_target, mut hunter_boid, mut boid_attack, mut hunger, hunter_position)) = boid_query.get_mut(*actor) {
                    if let Ok((mut health, hunted_position)) = target_query.get_mut(hunt_target.0) {
                        let delta = hunted_position.0 - hunter_position.0;
                        hunter_boid.desired_direction = delta.normalize_or_zero();

                        boid_attack.cool_down -= time.delta().as_secs_f32();
                        if boid_attack.cool_down < 0.0 {
                            boid_attack.cool_down = boid_attack.cool_down_default.clone();
                            if rng.gen_range(1..=100) <= boid_attack.skill_level {
                                debug!("We hit our prey!");
                                let damage =  rng.gen_range( boid_attack.max_damage.clone());
                                health.health -= damage;
                                hunger.hunger -= (damage * 2 ) as f32;
                                if hunger.hunger < 10.0 || health.health <= 0 {
                                    commands.entity(*actor).remove::<HuntTarget>();
                                    *state = ActionState::Success;
                                }
                            }
                        }

                    } else {
                        debug!("We did not have a hunting target");
                        *state = ActionState::Failure;
                    }

                } else {
                    debug!("We did not have a hunting target");
                    *state = ActionState::Failure;
                }
            }
            // All Actions should make sure to handle cancellations!
            ActionState::Cancelled => {
                debug!("Action was cancelled. Considering this a failure.");
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn hunt_prey_action_system(
    mut query: Query<(&Actor, &mut ActionState, &Hunt, &ActionSpan)>,
    mut boid_query: Query<(&HuntTarget, &mut BoidStuff, &Position)>,
    hunt_target_position_query: Query<&Position>,
) {
    for (Actor(actor), mut state, _, _) in &mut query {
        /*
        Hunting, how is it done?
        Well, if we are using some kind of naïve grid sub-division system, I would say
        we simply check the grid square we are in and the neighbouring ones for things we can
        prey upon.

        If we don't find any prey, we move to some quadrant.

        Otherwise, I guess something happens.
         */
        match *state {
            ActionState::Requested => {
                debug!("Time to look for some prey!");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                trace!("Do we have a hunt target?");
                if let Ok((hunt_target, mut hunter_boid, hunter_position)) = boid_query.get_mut(*actor) {
                    let hunted_position = hunt_target_position_query.get(hunt_target.0).unwrap();
                    let delta = hunted_position.0 - hunter_position.0;
                    if delta.length_squared() < 5.0 {
                        *state = ActionState::Success
                    } else {
                        hunter_boid.desired_direction = delta.normalize_or_zero();
                    }
                } else {
                    debug!("We did not have a hunting target");
                    *state = ActionState::Failure;
                }
            }
            // All Actions should make sure to handle cancellations!
            ActionState::Cancelled => {
                debug!("Action was cancelled. Considering this a failure.");
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn find_prey_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &FindPrey, &ActionSpan)>,
    pos_query: Query<(&Position, &QuadCoord)>,
    prey_query: Query<(Entity, &QuadCoord, &Position), With<Prey>>,
) {
    for (Actor(actor), mut state, _, span) in &mut query {
        /*
        Hunting, how is it done?
        Well, if we are using some kind of naïve grid sub-division system, I would say
        we simply check the grid square we are in and the neighbouring ones for things we can
        prey upon.

        If we don't find any prey, we move to some quadrant.

        Otherwise, I guess something happens.
         */


        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        match *state {
            ActionState::Requested => {
                debug!("Time to look for some prey!");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                trace!("Searching...");
                let prey_iter = prey_query.iter();
                if let Ok((position, quad_coord)) = pos_query.get(*actor) {
                    debug!("Searching for prey in quadrant: {:?}", quad_coord);
                    if let Some((entity, _, _)) = prey_iter.filter(|(_, prey_quad_coord, _)| {
                        prey_quad_coord.x >= quad_coord.x - 1 && prey_quad_coord.x <= quad_coord.x + 1 &&
                            prey_quad_coord.y >= quad_coord.y - 1 && prey_quad_coord.y <= quad_coord.y + 1
                    }).min_by_key(|(_, _, prey_position)| {
                        let delta = prey_position.0 - position.0;
                        let distance_sq: f32 = delta.length_squared();
                        distance_sq as i32
                    }) {
                        commands.entity(*actor).insert(HuntTarget(entity));
                        debug!("Found prey!");
                        *state = ActionState::Success;
                    } else {
                        debug!("No prey found!");
                        /*
                        Now we would want the AI to try to move this boid to some other quadrant.
                        We will do that later.
                         */
                        *state = ActionState::Failure;
                    }
                } else {
                    debug!("No position found for actor!");
                }
            }
            // All Actions should make sure to handle cancellations!
            ActionState::Cancelled => {
                debug!("Action was cancelled. Considering this a failure.");
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}


// Looks familiar? It's a lot like Actions!
pub fn hunger_scorer_system(
    hungers: Query<&Hunger>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Hungry>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(hunger) = hungers.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(hunger.hunger / 100.0);
            if hunger.hunger >= 80.0 {
                span.span().in_scope(|| {
                    debug!("Thirst above threshold! Score: {}", hunger.hunger / 100.0)
                });
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct Boid {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct BoidStuff {
    pub cohesion_boids: i32,
    pub separation_boids: i32,
    pub desired_direction: Vector2,
    pub flock_center: Vector2,
    pub separation_vector: Vector2,
    pub alignment_direction: Vector2,
    pub separation_distance: f32,
    pub cohesion_distance: f32,
    pub desired_factor: f32,
    pub separation_factor: f32,
    pub cohesion_factor: f32,
    pub alignment_distance: f32,
    pub alignment_factor: f32,
    pub alignment_boids: i32,
    pub turn_speed: f32,
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
            desired_direction: Vector2::ZERO,
            separation_distance: 25.0,
            cohesion_distance: 100.0,
            alignment_distance: 75.0,
            desired_factor: 1.0,
            separation_factor: 0.5,
            cohesion_factor: 0.5,
            alignment_factor: 0.7,
            turn_speed: 0.05,
        }
    }
}

#[derive(Reflect)]
#[derive(Clone, Debug, Component)]
pub struct BoidAttack {
    pub max_damage: Range<i32>,
    pub cool_down: f32,
    pub cool_down_default: f32,
    pub skill_level: i32
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

#[derive(Component, Debug, Reflect)]
pub struct Hunger {
    pub per_second: f32,
    pub hunger: f32,
}

#[derive(Component, Debug, Reflect)]
pub struct HuntTarget(pub Entity);

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Hunt {}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct AttackAndEat {
    pub per_second: f32,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FindPrey {}

impl Hunger {
    pub fn new(hunger: f32, per_second: f32) -> Self {
        Self { hunger, per_second }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Hungry;
