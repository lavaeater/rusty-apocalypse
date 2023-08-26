use std::ops::Range;
use std::sync::Arc;
use bevy::core::Name;
use bevy::prelude::{Bundle, Component, Entity, Reflect, Resource};
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
#[derive(Clone)]
pub struct WeaponDef {
    pub name: String,
    pub damage: Range<i32>,
    pub ammo: i32,
    pub rof: f32,
    pub ammo_type: Arc<AmmoType>,
}

impl WeaponDef {
    pub fn rof_to_cooldown(&self) -> f32 {
        1.0 / self.rof
    }

}

pub fn create_weapon_arc(weapon_def: Arc<WeaponDef>) -> Arc<Weapon> {
    Arc::new(create_weapon(weapon_def))
}

pub fn create_weapon(weapon_def: Arc<WeaponDef>) -> Weapon {
    Weapon::new(Arc::clone(&weapon_def))
}

#[derive(Clone)]
pub struct Weapon {
    pub weapon_def: Arc<WeaponDef>,
    pub ammo_left: i32,
}

#[derive(Resource)]
pub struct WeaponDefs {
    pub defs: Vec<Arc<WeaponDef>>,
}

impl Default for WeaponDefs {
    fn default() -> Self {
        Self {
            defs: vec![
                Arc::new(
                WeaponDef {
                    name: "Pistol".to_string(),
                    damage: 1..2,
                    ammo: 10,
                    rof: 2.0,
                    ammo_type: Arc::new(AmmoType::Bullet("Bullet".to_string())),
                }),
                Arc::new(
                    WeaponDef {
                    name: "Rocket Launcher".to_string(),
                    damage: 10..20,
                    ammo: 3,
                    rof: 1.0,
                    ammo_type: Arc::new(AmmoType::Rocket("Rocket".to_string())),
                }),
                Arc::new(
                    WeaponDef {
                    name: "Grenade Launcher".to_string(),
                    damage: 10..20,
                    ammo: 3,
                    rof: 1.0,
                    ammo_type: Arc::new(AmmoType::Grenade("Grenade".to_string())),
                }),
            ],
        }
    }
}

impl Weapon {
    pub fn new(weapon_def: Arc<WeaponDef>) -> Self {
        Self {
            weapon_def: Arc::clone(&weapon_def),
            ammo_left: weapon_def.ammo.clone()
        }
    }
}

#[derive(Component, Clone)]
pub struct CurrentWeapon {
    pub weapon: Option<Arc<Weapon>>,
    pub time_to_next_shot: f32,
}

impl CurrentWeapon {
    pub fn set_weapon(&mut self, weapon: Arc<Weapon>) {
        self.time_to_next_shot = weapon.weapon_def.rof_to_cooldown();
        self.weapon = Some(weapon);
    }

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