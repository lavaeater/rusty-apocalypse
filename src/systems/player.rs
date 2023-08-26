use std::sync::Arc;
use std::thread::current;
use bevy::prelude::{Commands, default, Entity, Query, Res, SpriteBundle, Transform, With};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use crate::components::control::CycleWeapon;
use crate::components::player::{PlayerBundle, WeaponInventory};
use crate::components::weapon::{create_weapon_arc, CurrentWeapon, WeaponDefs};
use crate::METERS_PER_PIXEL;

pub fn cycle_weapon(
    mut commands: Commands,
    mut query: Query<(Entity, &WeaponInventory, &mut CurrentWeapon), With<CycleWeapon>>,
) {
    for (entity, mut weapon_inventory, mut current_weapon) in query.iter_mut() {
        current_weapon.weapon = Some(Arc::clone(&weapon_inventory.weapons[0]));
        commands.entity(entity).remove::<CycleWeapon>();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weapon_definitions: Res<WeaponDefs>) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    1.0,
                )
                    .with_scale(Vec3::new(
                        METERS_PER_PIXEL,
                        METERS_PER_PIXEL,
                        1.0,
                    )),
                texture: asset_server.load("sprites/person.png"),
                ..default()
            },
            PlayerBundle {
                weapon_inventory: WeaponInventory {
                    weapons: vec![create_weapon_arc(Arc::clone(&weapon_definitions
                        .defs[0]))],
                },
                ..Default::default()
            },
        ));
}
