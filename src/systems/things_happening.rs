use bevy::math::{Vec2, Vec3};
use bevy::prelude::{AssetServer, Commands, default, Res, SpriteBundle, Transform};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody, Sensor};
use crate::components::quads::QuadCoord;
use crate::{Layer, METERS_PER_PIXEL};

pub fn spawn_places(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
                texture: asset_server.load("sprites/hut.png"),
                ..default()
            },
            RigidBody::Kinematic,
            QuadCoord::default(),
            Position::from(Vec2 {
                x: 0.0,
                y: 0.0,
            }),
            Collider::cuboid(16.0 * METERS_PER_PIXEL, 8.0 * METERS_PER_PIXEL),
            Sensor {},
            CollisionLayers::new([Layer::Places], [Layer::Player])
        ));
}