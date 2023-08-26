use bevy::prelude::{Commands, default, Entity, Query, Res, SpriteBundle, Transform};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use crate::components::control::{CycleDirection, CycleWeapon};
use crate::components::player::{PlayerBundle, WeaponInventory};
use crate::components::weapon::{CurrentWeapon, WeaponDefs};
use crate::METERS_PER_PIXEL;

pub fn cycle_weapon_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut WeaponInventory, &mut CurrentWeapon, &CycleWeapon)>,
) {
    for (entity, mut weapon_inventory, mut current_weapon, cycle) in query.iter_mut() {
        match &current_weapon.weapon {
            Some(weapon) => weapon_inventory.weapons.push_back(weapon.clone()),
            None => {}
        }
        match cycle.direction{
            CycleDirection::Forward => {
                weapon_inventory.weapons.rotate_right(1);
                if let Some(new_weapon) = weapon_inventory.weapons.pop_front() {
                    current_weapon.set_weapon(new_weapon.clone());
                }
            }
            CycleDirection::Backward => {
                weapon_inventory.weapons.rotate_left(1);
                if let Some(new_weapon) = weapon_inventory.weapons.pop_front() {
                    current_weapon.set_weapon(new_weapon.clone());
                }
            }
        }

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
                    weapons: weapon_definitions.defs.iter().map(|def| def.create_weapon()).collect(),
                },
                ..Default::default()
            },
        ));
}
