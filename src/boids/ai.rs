use bevy::prelude::{Commands, Component, Entity, Query, Reflect, Res, ResMut, Time, With};
use bevy::log::{debug, trace};
use big_brain::prelude::{ActionBuilder, ActionSpan, Actor, Score, ScorerBuilder, ScorerSpan};
use big_brain::actions::ActionState;
use bevy_xpbd_2d::components::Position;
use bevy_rand::prelude::GlobalEntropy;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use crate::boids::components::{BoidAttack, BoidStuff};
use crate::components::{Health, Prey, QuadCoord};

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
