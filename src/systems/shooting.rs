use bevy::prelude::Query;
use bevy_xpbd_2d::components::Position;
use crate::components::control::TriggerPulled;
use crate::components::control::PlayerControl;

pub fn shooting_system(
    is_shooting: Query<&TriggerPulled>,
    shooter_query: Query<(&Position, &mut Weapon, &PlayerControl)>
) {

}