use bevy::prelude::{Camera, Color, Commands, default, Entity, EventReader, GlobalTransform, MouseButton, Query, Transform, Window, With};
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::shapes;
use bevy::math::Vec2;
use bevy_prototype_lyon::entity::{Path, ShapeBundle};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_xpbd_2d::components::Rotation;
use bevy_prototype_lyon::path::ShapePath;
use bevy::input::keyboard::KeyboardInput;
use std::ops::AddAssign;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use crate::components::control::{CycleDirection, CycleWeapon, PlayerControl, TriggerPulled};
use crate::components::player::Player;
use bevy::prelude::KeyCode;
use crate::components::general::{AimLine, GameCam};

pub fn keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(Entity, &mut PlayerControl), With<Player>>,
    mut commands: Commands,
) {
    if let Ok((entity, mut player_control)) = query.get_single_mut() {
        for ev in key_evr.iter() {
            match ev.state {
                ButtonState::Pressed => match ev.key_code {
                    Some(KeyCode::A) => {
                        player_control.direction.x = -1.0;
                    }
                    Some(KeyCode::D) => {
                        player_control.direction.x = 1.0;
                    }
                    Some(KeyCode::W) => {
                        player_control.direction.y = 1.0;
                    }
                    Some(KeyCode::S) => {
                        player_control.direction.y = -1.0;
                    }
                    Some(KeyCode::Space) => {
                        commands.entity(entity).insert(TriggerPulled {});
                    }
                    _ => {}
                },
                ButtonState::Released => match ev.key_code {
                    Some(KeyCode::Right) => {
                        commands.entity(entity).insert(CycleWeapon {
                            direction: CycleDirection::Forward,
                        });
                    }
                    Some(KeyCode::Left) => {
                        commands.entity(entity).insert(CycleWeapon {
                            direction: CycleDirection::Backward,
                        });
                    }
                    Some(KeyCode::A) => {
                        player_control.direction.x = 0.0;
                    }
                    Some(KeyCode::D) => {
                        player_control.direction.x = 0.0;
                    }
                    Some(KeyCode::W) => {
                        player_control.direction.y = 0.0;
                    }
                    Some(KeyCode::S) => {
                        player_control.direction.y = 0.0;
                    }
                    Some(KeyCode::Space) => {
                        commands.entity(entity).remove::<TriggerPulled>();
                    }
                    _ => {}
                }
            }
        }
        player_control.direction = player_control.direction.normalize_or_zero();
    }
}

pub fn mouse_key_input(
    mut mouse_ev_reader: EventReader<MouseButtonInput>,
    mut query: Query<Entity, With<Player>>,
    mut commands: Commands) {
    if let Ok(entity) = query.get_single_mut() {
        for ev in mouse_ev_reader.iter() {
            match ev.state {
                ButtonState::Pressed => {
                    match ev.button {
                        MouseButton::Left => {
                            commands.entity(entity).insert(TriggerPulled {});
                        }
                        _ => {}
                    }
                }

                ButtonState::Released => {
                    match ev.button {
                        MouseButton::Left => {
                            commands.entity(entity).remove::<TriggerPulled>();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn mouse_position(
    mut q_direction: Query<&mut PlayerControl, With<Player>>,
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
    q_mouse_aim: Query<(&Transform, &PlayerControl), With<Player>>,
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
        &mut PlayerControl,
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
