use bevy::prelude::{Commands, Query, With};
use bevy_xpbd_2d::components::Position;
use crate::components::control::TriggerPulled;
use crate::components::control::PlayerControl;
use crate::components::weapon::{CurrentWeapon, ProjectileBundle, Weapon};

pub fn shooting_system(
    mut shooter_query: Query<(&Position, &mut CurrentWeapon, &PlayerControl), With<TriggerPulled>>,
    mut commands: Commands,
) {
    for(shooter_position, mut current_weapon, player_control)
    in shooter_query.iter_mut() {
        if current_weapon.fire_and_report_back() {
            commands.spawn((
                ProjectileBundle::new(
                "Bullet".to_string(),
                *shooter_position,
                    player_control.aim_direction * 100.0,



            )));
        }
    }
}