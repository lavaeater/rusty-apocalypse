pub(crate) mod input;
pub(crate) mod startup;
pub(crate) mod camera;
pub(crate) mod movement;

use crate::components::{QuadCoord, QuadStore};
use bevy::prelude::{Entity, Query, ResMut};
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::Position;

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

