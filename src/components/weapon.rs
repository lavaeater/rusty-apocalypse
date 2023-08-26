use std::ops::Range;
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
    pub ammo_type: AmmoType,
}

impl WeaponDef {
    pub fn create_weapon(&self) -> Weapon {
        Weapon::new(&self)
    }
}

impl Weapon {
    pub fn rof_to_cooldown(&self) -> f32 {
        1.0 / self.rof
    }
}




#[derive(Clone, Reflect)]
pub struct Weapon {
    pub ammo_left: i32,
    pub name: String,
    pub damage: Range<i32>,
    pub current_ammo: i32,
    pub max_ammo: i32,
    pub rof: f32,
    pub ammo_type: AmmoType,
}


#[derive(Resource)]
pub struct WeaponDefs {
    pub defs: Vec<WeaponDef>,
}

impl Default for WeaponDefs {
    fn default() -> Self {
        Self {
            defs: vec![
                WeaponDef {
                    name: "Pistol".to_string(),
                    damage: 1..2,
                    ammo: 10,
                    rof: 2.0,
                    ammo_type: AmmoType::Bullet("Bullet".to_string()),
                },
                WeaponDef {
                    name: "Rocket Launcher".to_string(),
                    damage: 10..20,
                    ammo: 3,
                    rof: 1.0,
                    ammo_type: AmmoType::Rocket("Rocket".to_string()),
                },
                WeaponDef {
                    name: "Grenade Launcher".to_string(),
                    damage: 10..20,
                    ammo: 3,
                    rof: 1.0,
                    ammo_type: AmmoType::Grenade("Grenade".to_string()),
                },
            ],
        }
    }
}

impl Weapon {
    pub fn new(weapon_def: &WeaponDef) -> Self {
        Self {
            ammo_left: weapon_def.ammo.clone(),
            name: weapon_def.name.clone(),
            damage: weapon_def.damage.clone(),
            current_ammo: weapon_def.ammo.clone(),
            rof: weapon_def.rof.clone(),
            ammo_type: weapon_def.ammo_type.clone(),
            max_ammo: weapon_def.ammo.clone(),
        }
    }
}

#[derive(Component, Clone, Reflect)]
pub struct CurrentWeapon {
    pub weapon: Option<Weapon>,
    pub time_to_next_shot: f32,
}

impl CurrentWeapon {
    pub fn set_weapon(&mut self, weapon: Weapon) {
        self.time_to_next_shot = weapon.rof_to_cooldown();
        self.weapon = Some(weapon);
    }

    pub fn tick(&mut self, delta: f32) {
        self.time_to_next_shot -= delta;
    }

    pub fn can_fire(&self) -> bool {
        self.weapon.is_some() && self.time_to_next_shot <= 0.0 && self.weapon.as_ref().unwrap().ammo_left > 0
    }
    pub fn fire(&mut self) {
        if let Some(weapon) = self.weapon.as_mut() {
            weapon.ammo_left -= 1;
            self.time_to_next_shot = weapon.rof_to_cooldown();
        }
    }

    pub fn did_we_fire(&mut self) -> bool {
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