use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Commands, default, Entity, Query, Res, Time, Transform, With};
use bevy::sprite::SpriteBundle;
use bevy_xpbd_2d::components::Position;
use bevy_xpbd_2d::prelude::{Collider, CollisionLayers};
use crate::components::control::TriggerPulled;
use crate::components::control::PlayerControl;
use crate::components::weapon::{CurrentWeapon, ProjectileBundle};
use crate::{Layer, METERS_PER_PIXEL};

pub fn shooting_system(
    time: Res<Time>,
    mut shooter_query: Query<(Entity, &Position, &mut CurrentWeapon, &PlayerControl), With<TriggerPulled>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (shooter, shooter_position, mut current_weapon, player_control)
    in shooter_query.iter_mut() {

        current_weapon.tick(time.delta_seconds());

        if current_weapon.did_we_fire() {
            commands.spawn((
                ProjectileBundle::new(
                    "Bullet".to_string(),
                    *shooter_position,
                    player_control.aim_direction * 100.0,
                    Collider::ball(0.5),
                    CollisionLayers::new([Layer::Bullet], [Layer::Player, Layer::Boid]),
                    shooter,
                ),
                SpriteBundle {
                    transform: Transform::from_xyz(
                        shooter_position.x,
                        shooter_position.y,
                        1.0,
                    )
                        .with_scale(Vec3::new(
                            METERS_PER_PIXEL,
                            METERS_PER_PIXEL,
                            1.0,
                        )),
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                },
            ));
        }
    }
}