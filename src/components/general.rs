use bevy::prelude::{Bundle, Component, Reflect, SpriteSheetBundle};
use bevy_ecs_ldtk::{LdtkEntity, LdtkIntCell};

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

#[derive(Component, Clone)]
pub struct Prey {}

#[derive(Component)]
pub struct AimLine {}


#[derive(Component)]
pub struct GameCam {}

#[derive(Component, Clone)]
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

#[derive(Debug, Component, Reflect, Clone)]
pub struct Health {
    pub health: i32,
    pub max: i32
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health: 100,
            max: 100,
        }
    }
}
