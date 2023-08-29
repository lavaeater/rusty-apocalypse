use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Place {
    pub id: String,
}

#[derive(Component, Debug)]
pub struct Lore {
    pub text: String,
}
