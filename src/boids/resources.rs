use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BoidGenerationSettings {
    pub cool_down: f32,
    pub time_left: f32,
    pub counting_time_left: f32,
    pub boids_to_generate: i32,
    pub max_boids: usize,
    pub min_boids: usize,
    pub generate_boids: bool
}

impl BoidGenerationSettings {
    pub fn new(cool_down: f32, to_generate: i32, min_boids: usize, max_boids: usize) -> Self {
        Self {
            cool_down,
            time_left: cool_down,
            counting_time_left: cool_down * 10.0,
            boids_to_generate: to_generate,
            min_boids,
            max_boids,
            generate_boids: true
        }
    }
}