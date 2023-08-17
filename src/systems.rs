use crate::components::{CameraFollow, InWater, Player, GameCam};
use crate::{Layer, METERS_PER_PIXEL, PIXELS_PER_METER};
use bevy::asset::{AssetServer};
use bevy::input::keyboard::KeyboardInput;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::{
    default, Camera2dBundle, Commands, EventReader, OrthographicProjection, Query,
    Res, SpriteBundle, Transform, With, Without,
};
use bevy::render::camera::ScalingMode;
use bevy_xpbd_2d::components::{
    Collider, CollisionLayers, ExternalForce, Position, RigidBody};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    let x = 0.0;
    let y = 0.0;
        commands
            .spawn((
                CameraFollow {},
                InWater {},
                SpriteBundle {
                    transform: Transform::from_xyz(
                        x * PIXELS_PER_METER,
                        y * PIXELS_PER_METER,
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
                Player {},
                RigidBody::Dynamic,
                Position::from(Vec2 {
                    x: x * PIXELS_PER_METER,
                    y: y * PIXELS_PER_METER,
                }),
                ExternalForce::default().with_persistence(false),
                Collider::cuboid(16.0* METERS_PER_PIXEL, 8.0 * METERS_PER_PIXEL),
                CollisionLayers::new([Layer::Player], [Layer::Walls, Layer::Water]),
            ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: METERS_PER_PIXEL * 2.0,
                near: 0.0,
                far: 1000.0,
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: ScalingMode::WindowSize(PIXELS_PER_METER),
                area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            },
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

    camera_transform.translation = camera_transform.translation.lerp(target, 0.1);
}

pub fn keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut ExternalForce, With<Player>>,
) {
    use bevy::input::ButtonState;
    use bevy::math::vec2;
    use bevy::prelude::KeyCode;
    if let Ok(mut external_force) = query.get_single_mut() {
        for ev in key_evr.iter() {
            match ev.state {
                ButtonState::Pressed => match ev.key_code {
                    Some(KeyCode::A) => {
                        external_force.apply_force(vec2(-5.0, 0.0));
                    }
                    Some(KeyCode::D) => {
                        external_force.apply_force(vec2(5.0, 0.0));
                    }
                    Some(KeyCode::W) => {
                        external_force.apply_force(vec2(0.0, 5.0));
                    }
                    Some(KeyCode::S) => {
                        external_force.apply_force(vec2(0.0, -5.0));
                    }
                    _ => {}
                },
                ButtonState::Released => {
                    println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
                }
            }
        }
    }
}
