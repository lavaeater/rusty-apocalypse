use bevy::prelude::{Entity, Event};
use crate::resources::facts_of_the_world::Fact;

#[derive(Debug, Event)]
pub struct FactUpdatedEvent {
    pub key: String,
    pub fact: Fact,
}

#[derive(Debug, Event)]
pub struct FactOccuredEvent {
    pub key: String,
    pub fact: Fact,
    pub fact_entity: Option<Entity>,
    pub acting_entity: Option<Entity>,
}