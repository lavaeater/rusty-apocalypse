use std::ops::Range;
use bevy::prelude::{Component, Reflect};


#[derive(Reflect, Clone)]
pub enum AmmoType {
    Bullet(String),
    Rocket(String),
    Grenade(String),
}

//Should perhaps be some kind of macro I guess?
#[derive(Reflect, Clone)]
pub struct WeaponDef {
    pub name: String,
    pub damage: Range<i32>,
    pub ammo: i32,
    pub rof: f32,
    pub ammo_type: AmmoType,
}

#[derive(Reflect, Clone)]
pub struct Weapon {
    pub weapon_def: WeaponDef,
    pub ammo_left: i32
}

impl Weapon {
    fn new(weapon_def: WeaponDef) -> Self {
        Self {
            weapon_def: weapon_def.clone(),
            ammo_left: weapon_def.ammo.clone()
        }
    }
}

#[derive(Component, Clone)]
pub struct CurrentWeapon {
    pub weapon: Option<Weapon>,
}

impl Default for CurrentWeapon {
    fn default() -> Self {
        Self {
            weapon: None
        }
    }
}