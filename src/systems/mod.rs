pub(crate) mod input;

use crate::components::{CameraFollow, DirectionControl, GameCam, PlayerBundle, QuadCoord, QuadStore};
use crate::{CAMERA_SCALE, METERS_PER_PIXEL, PIXELS_PER_METER};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::{Camera2dBundle, Commands, default, Entity, OrthographicProjection, Query, Res, ResMut, SpriteBundle, Transform, With, Without};
use bevy::render::camera::ScalingMode;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::{
    ExternalForce, Position};
use bevy_xpbd_2d::prelude::LinearVelocity;
use crate::boids::{Boid, BoidDirection};
use crate::components::player::Player;

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

pub fn camera_follow(
    to_follow: Query<&Transform, (With<CameraFollow>, Without<GameCam>)>,
    mut camera: Query<&mut Transform, (With<GameCam>, Without<CameraFollow>)>,
) {
    let Ok(player_position) = to_follow.get_single() else { return; };
    let Ok(mut camera_transform) = camera.get_single_mut() else { return; };
    let target = Vec3 {
        x: player_position.translation.x,
        y: player_position.translation.y,
        z: camera_transform.translation.z,
    };

    camera_transform.translation = camera_transform.translation.lerp(target, 0.5);
}

#[allow(dead_code)]
fn external_force_player_control(
    mut query: Query<(&mut ExternalForce, &DirectionControl), With<Player>>
) {
    if let Ok((mut external_force, direction_control)) = query.get_single_mut() {
        external_force.apply_force(direction_control.direction * direction_control.force_scale);
    }
}

pub fn linear_velocity_control_boid(
    mut query: Query<(&mut LinearVelocity, &BoidDirection), (With<Boid>, Without<Player>)>
) {
    let mut iter = query.iter_mut();
    while let Some((mut linear_velocity, direction_control)) = iter.next() {
        linear_velocity.x = direction_control.direction.x * direction_control.force_scale;
        linear_velocity.y = direction_control.direction.y * direction_control.force_scale;
    }
}

pub fn linear_velocity_control_player(
    mut query: Query<(&mut LinearVelocity, &DirectionControl), (Without<Boid>, With<Player>)>
) {
    if let Ok((mut linear_velocity, direction_control)) = query.get_single_mut() {
        linear_velocity.x = direction_control.direction.x * direction_control.force_scale;
        linear_velocity.y = direction_control.direction.y * direction_control.force_scale;
    }
}

pub fn naive_quad_system(
    mut query: Query<(Entity, &Position, &mut QuadCoord)>,
    mut quad_store: ResMut<QuadStore>,
) {
    let mut iter = query.iter_mut();
    while let Some((entity, position, mut quad_coord)) = iter.next() {
        let new_coord = QuadCoord::new(
            (position.0.x / 25.0).floor() as i32,
            (position.0.y / 25.0).floor() as i32,
        );

        if !new_coord.eq(&quad_coord) {
            if !quad_store.0.contains_key(&new_coord) {
                quad_store.0.insert(new_coord, HashSet::new());
            }
            let old_coord = quad_coord.clone();
            if quad_store.0.contains_key(&old_coord) {
                let set = quad_store.0.get_mut(&old_coord).unwrap();
                set.remove(&entity);
                if set.is_empty() {
                    quad_store.0.remove(&old_coord);
                }
            }

            quad_store.0.get_mut(&new_coord).unwrap().insert(entity);

            quad_coord.x = new_coord.x;
            quad_coord.y = new_coord.y;
        }
    }
}

