use std::ops::AddAssign;
use crate::components::{AimLine, CameraFollow, DirectionControl, GameCam, Player, PlayerBundle, QuadCoord, QuadStore};
use crate::{CAMERA_SCALE, METERS_PER_PIXEL, PIXELS_PER_METER};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, default, Entity, EventReader, GlobalTransform, OrthographicProjection, Query, Res, ResMut, SpriteBundle, Transform, With, Without};
use bevy::render::camera::ScalingMode;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::{
    ExternalForce, Position};
use bevy_xpbd_2d::prelude::{LinearVelocity, Rotation};
use bevy::window::{PrimaryWindow, Window};
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::{Path, ShapeBundle};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::path::ShapePath;
use bevy_prototype_lyon::shapes;
use crate::boids::{Boid, BoidDirection};

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

pub fn keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut DirectionControl, With<Player>>,
) {
    use bevy::input::ButtonState;
    use bevy::prelude::KeyCode;
    if let Ok(mut direction_control) = query.get_single_mut() {
        for ev in key_evr.iter() {
            // println!("{:?}:{:?}", ev.state, ev.key_code);
            match ev.state {
                ButtonState::Pressed => match ev.key_code {
                    Some(KeyCode::A) => {
                        direction_control.direction.x = -1.0;
                    }
                    Some(KeyCode::D) => {
                        direction_control.direction.x = 1.0;
                    }
                    Some(KeyCode::W) => {
                        direction_control.direction.y = 1.0;
                    }
                    Some(KeyCode::S) => {
                        direction_control.direction.y = -1.0;
                    }
                    _ => {}
                },
                ButtonState::Released => match ev.key_code {
                    Some(KeyCode::A) => {
                        direction_control.direction.x = 0.0;
                    }
                    Some(KeyCode::D) => {
                        direction_control.direction.x = 0.0;
                    }
                    Some(KeyCode::W) => {
                        direction_control.direction.y = 0.0;
                    }
                    Some(KeyCode::S) => {
                        direction_control.direction.y = 0.0;
                    }
                    _ => {}
                }
            }
        }
        direction_control.direction = direction_control.direction.normalize_or_zero();
    }
}

pub fn mouse_position(
    mut q_direction: Query<&mut DirectionControl, With<Player>>,
    // need to get window dimensions
    q_windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCam>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let mut direction_control = q_direction.single_mut();
    if let Some(position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor|
            camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        direction_control.mouse_position = position;
    }
}

pub fn add_mouse_aim_line(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(199.0, 200.0));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        Stroke::new(Color::RED, 0.05),
        Fill::color(Color::RED),
        AimLine {},
    ));
}

pub fn draw_mouse_aim(
    q_mouse_aim: Query<(&Transform, &DirectionControl), With<Player>>,
    mut query: Query<&mut Path, With<AimLine>>,
) {
    let (transform, direction_control) = q_mouse_aim.single();
    let mut path = query.single_mut();
    let line = shapes::Line(Vec2::new(transform.translation.x, transform.translation.y), direction_control.mouse_position);
    *path = ShapePath::build_as(&line)
}

pub fn mouse_look(
    mut query: Query<(
        &mut Rotation,
        &mut DirectionControl,
        &Transform), With<Player>>,
) {
    if let Ok((
                  mut rotation,
                  mut direction_control,
                  transform)) = query.get_single_mut() {
        direction_control.up = Vec2::new(transform.up().x, transform.up().y);

        direction_control.aim_direction =
            (direction_control.mouse_position - Vec2::new(
                transform.translation.x,
                transform.translation.y)
            )
                .try_normalize()
                .unwrap_or(Vec2::X);

        let target_up = direction_control.up.lerp(direction_control.aim_direction, 0.5);
        let to_add = Rotation::from_radians(
            target_up
                .angle_between(
                    direction_control
                        .aim_direction
                )
        );
        direction_control.aim_rotation = to_add;
        direction_control.aim_degrees = to_add.as_degrees();

        rotation.add_assign(to_add);
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
                let mut set = quad_store.0.get_mut(&old_coord).unwrap();
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

