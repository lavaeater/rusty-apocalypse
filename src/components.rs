use bevy::prelude::{Bundle, Component, SpriteSheetBundle};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};
use bevy_xpbd_2d::math::Vector2;

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

#[derive(Copy, Clone, Debug, Component)]
pub struct DirectionControl {
    pub direction: Vector2,
    pub force_scale: f32,
}

impl Default for DirectionControl {
    fn default() -> Self {
        Self {
            direction: Vector2::ZERO,
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
