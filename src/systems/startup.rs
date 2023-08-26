use bevy::prelude::{Camera2dBundle, Commands, default, OrthographicProjection, Res, SpriteBundle, Transform};
use bevy::asset::AssetServer;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::core::Name;
use bevy::render::camera::ScalingMode;
use crate::components::{GameCam, PlayerBundle};
use crate::{CAMERA_SCALE, METERS_PER_PIXEL, PIXELS_PER_METER};

pub fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    0.0,
                )
                    .with_scale(Vec3::new(
                        METERS_PER_PIXEL * 10.0,
                        METERS_PER_PIXEL * 10.0,
                        1.0,
                    )),
                texture: asset_server.load("background/background.png"),
                ..default()
            },
        ));
}

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

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::from("Camera".to_string()),
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: CAMERA_SCALE,
                near: 0.0,
                far: 1000.0,
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: ScalingMode::WindowSize(PIXELS_PER_METER),
                area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            ..default()
        },
        GameCam {},
    ));
}
