use bevy::prelude::Component;

#[derive(Component)]
pub struct Stunned {
    pub cooldown: f32,
}

impl Stunned {
    pub fn default() -> Self {
        Self { cooldown: 1.0 }
    }
    pub fn new(cooldown: f32) -> Self {
        Self { cooldown }
    }
}