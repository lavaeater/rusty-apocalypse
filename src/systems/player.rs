use bevy::prelude::{Commands, default, Res, SpriteBundle, Transform};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use crate::components::player::PlayerBundle;
use crate::METERS_PER_PIXEL;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
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
            PlayerBundle::default(),
        ));
}
