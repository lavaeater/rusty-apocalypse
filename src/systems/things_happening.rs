use bevy::math::{Vec2, Vec3};
use bevy::prelude::{AssetServer, Commands, default, EventReader, EventWriter, Query, Res, ResMut, SpriteBundle, Transform};
use bevy_tts::Tts;
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody};
use crate::components::quads::QuadCoord;
use crate::{Layer, METERS_PER_PIXEL};
use crate::components::things_happening::{Lore, Place};
use crate::events::facts::{FactOccuredEvent, FactUpdatedEvent};
use crate::resources::facts_of_the_world::Fact::BoolFact;
use crate::resources::facts_of_the_world::{Fact, FactsOfTheWorld};

pub fn spawn_places(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    20.0,
                    20.0,
                    4.0,
                )
                    .with_scale(Vec3::new(
                        METERS_PER_PIXEL,
                        METERS_PER_PIXEL,
                        1.0,
                    )),
                texture: asset_server.load("sprites/hut.png"),
                ..default()
            },
            Place {
                id: "Place".to_string(),
            },
            Lore {
                text: "Another one of these.\
                An abandoned camp, probably raided.".to_string(),
            },
            RigidBody::Static,
            QuadCoord::default(),
            Position::from(Vec2 {
                x: 20.0,
                y: 20.0,
            }),
            Collider::ball(32.0 * METERS_PER_PIXEL),
            CollisionLayers::new([Layer::Place], [Layer::Player, Layer::Bullet])
        ));
}

pub fn fact_occured_event_listener(
    mut ev_fact_occured: EventReader<FactOccuredEvent>,
    mut ev_fact_updated: EventWriter<FactUpdatedEvent>,
    lore_query: Query<&Lore>,
    mut facts_of_the_world: ResMut<FactsOfTheWorld>,
    mut tts: ResMut<Tts>,
) {
    for FactOccuredEvent { key, fact, fact_entity, acting_entity } in ev_fact_occured.iter() {
        match fact {
            BoolFact { .. } => {

                if facts_of_the_world.update_fact(&key, fact.clone()) {
                    ev_fact_updated.send(FactUpdatedEvent {
                        key: key.clone(),
                        fact: fact.clone(),
                    });
                }
                if fact_entity.is_some() {
                    if let Ok(lore) = lore_query.get(fact_entity.unwrap()) {
                        tts.speak(&lore.text, false).unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}