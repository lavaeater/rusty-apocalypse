use bevy::math::{Vec2, Vec3};
use bevy::prelude::{AssetServer, Commands, default, Res, SpriteBundle, Transform};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, Position, RigidBody};
use crate::components::quads::QuadCoord;
use crate::{Layer, METERS_PER_PIXEL};
use crate::components::things_happening::{Lore, Place};

pub fn spawn_places(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    20.0,
                    20.0,
                    4.0,
                )
                    .with_scale(Vec3::new(
                        METERS_PER_PIXEL,
                        METERS_PER_PIXEL,
                        1.0,
                    )),
                texture: asset_server.load("sprites/hut.png"),
                ..default()
            },
            Place {
                id: "Place".to_string(),
            },
            Lore {
                text: "Lore".to_string(),
            },
            RigidBody::Static,
            QuadCoord::default(),
            Position::from(Vec2 {
                x: 20.0,
                y: 20.0,
            }),
            Collider::ball(32.0 * METERS_PER_PIXEL),
            CollisionLayers::new([Layer::Place], [Layer::Player, Layer::Bullet])
        ));
}