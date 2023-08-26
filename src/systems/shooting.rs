use bevy::prelude::{Commands, Entity, Query, With};
use bevy_xpbd_2d::components::Position;
use bevy_xpbd_2d::prelude::{Collider, CollisionLayers};
use crate::components::control::TriggerPulled;
use crate::components::control::PlayerControl;
use crate::components::weapon::{CurrentWeapon, ProjectileBundle};
use crate::Layer;

pub fn shooting_system(
    mut shooter_query: Query<(Entity, &Position, &mut CurrentWeapon, &PlayerControl), With<TriggerPulled>>,
    mut commands: Commands,
) {
    for (shooter, shooter_position, mut current_weapon, player_control)
    in shooter_query.iter_mut() {
        if current_weapon.fire_and_report_back() {
            commands.spawn(
                ProjectileBundle::new(
                    "Bullet".to_string(),
                    *shooter_position,
                    player_control.aim_direction * 100.0,
                    Collider::ball(0.5),
                    CollisionLayers::new([Layer::Bullet], [Layer::Player, Layer::Boid]),
                    shooter,
                ));
        }
    }
}