use std::ops::Range;
use bevy::core::Name;
use bevy::prelude::{Bundle, Component, Entity, Reflect};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, LinearVelocity, RigidBody};
use bevy_xpbd_2d::math::Vector2;
use bevy_xpbd_2d::prelude::Position;

#[derive(Component, Clone)]
pub struct Projectile {}

#[derive(Reflect, Clone)]
pub enum WeaponType {
    Projectile,
    Melee,
}

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
    pub ammo_left: i32,
}

#[derive(Component, Clone)]
pub struct CurrentWeapon {
    pub weapon: Option<Weapon>,
    pub time_to_next_shot: f32,
}

impl CurrentWeapon {
    pub fn can_fire(&self) -> bool {
        self.weapon.is_some() && self.time_to_next_shot <= 0.0
    }
    pub fn fire(&mut self) {
        if let Some(weapon) = self.weapon.as_mut() {
            weapon.ammo_left -= 1;
            self.time_to_next_shot = weapon.weapon_def.rof;
        }
    }

    pub fn fire_and_report_back(&mut self) -> bool {
        self.can_fire() && {
            self.fire();
            true
        }
    }
}

impl Default for CurrentWeapon {
    fn default() -> Self {
        Self {
            weapon: None,
            time_to_next_shot: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Shooter(Entity);

#[derive(Bundle)]
pub struct ProjectileBundle {
    name: Name,
    position: Position,
    rigid_body: RigidBody,
    collider: Collider,
    collision_layers: CollisionLayers,
    shooter: Shooter,
    linear_velocity: LinearVelocity,
}

impl ProjectileBundle {
    pub fn new(
        name: String,
        from: Position,
        lv: Vector2,
        collider: Collider,
        collision_layers: CollisionLayers,
        shooter: Entity) -> Self {
        Self {
            name: Name::from(name),
            position: from.clone(),
            rigid_body: RigidBody::Kinematic,
            collider,
            collision_layers,
            shooter: Shooter(shooter),
            linear_velocity: LinearVelocity(lv),
        }
    }
}