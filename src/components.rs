use bevy::prelude::{Bundle, Component, Reflect, SpriteSheetBundle};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};
use bevy_xpbd_2d::math::Vector2;
use bevy_xpbd_2d::prelude::Rotation;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Water;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WaterBundle {
    water: Water,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStart;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct AimLine {}

#[derive(Reflect)]
#[derive(Copy, Clone, Debug, Component)]
pub struct DirectionControl {
    pub direction: Vector2,
    pub aim_direction: Vector2,
    pub up: Vector2,
    pub aim_rotation: Rotation,
    pub aim_degrees: f32,
    pub mouse_position: Vector2,
    pub force_scale: f32,
}

impl Default for DirectionControl {
    fn default() -> Self {
        Self {
            direction: Vector2::ZERO,
            aim_direction: Vector2::Y,
            up: Vector2::X,
            aim_rotation: Rotation::default(),
            aim_degrees: 0.0,
            mouse_position: Vector2::ZERO,
            force_scale: 10.0,
        }
    }
}


#[derive(Component)]
pub struct GameCam {}

#[derive(Component)]
pub struct CameraFollow {}

#[derive(Bundle, LdtkEntity)]
pub struct MapEntity {
    #[sprite_sheet_bundle]
    #[bundle()]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component)]
pub struct InWater {}

#[derive(Bundle, LdtkIntCell)]
pub struct IntCell {
    #[bundle()]
    sprite_bundle: SpriteSheetBundle,
}
