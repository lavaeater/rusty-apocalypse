use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BoidGenerationSettings {
    pub cool_down: f32,
    pub time_left: f32,
    pub boids_to_generate: i32
}

impl BoidGenerationSettings {
    pub fn new(cool_down: f32, to_generate: i32) -> Self {
        Self {
            cool_down,
            time_left: cool_down,
            boids_to_generate: to_generate
        }
    }
}