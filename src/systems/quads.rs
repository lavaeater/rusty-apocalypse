//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Quadrant {
//     NorthWest = 0,
//     NorthEast = 1,
//     SouthWest = 2,
//     SouthEast = 3,
// }
//
// pub enum Quad {
//     Leaf { entities: HashSet<Entity> },
//     Internal {
//         bounds: Rect,
//         quads: Box<HashMap<Quadrant, Quad>>,
//     },
// }
//
// pub fn create_quad_tree(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Quad {
//     let bounds = Rect {
//         min: Vec2::new(min_x, min_y),
//         max: Vec2::new(max_x, max_y),
//     };
//
//     Quad::Internal {
//         bounds,
//         quads: Box::new(HashMap::new()),
//     }
// }
//
// pub fn create_quad_tree_from_bounds(bounds: Rect) -> Quad {
//     Quad::Internal {
//         bounds,
//         quads: Box::new(HashMap::new()),
//     }
// }
//
// fn split_leaf_node(quad: &mut Quad, bounds: &Rect, position_query: Query<&Position>) {
//     // Create child nodes and redistribute entities.
//     // Calculate bounds for each child node based on the parent bounds.
//     let child_bounds = calculate_child_bounds(bounds);
//
//     let mut child_entities = vec![Vec::new(); 4]; // One vector for each quadrant.
//
//     // Iterate through the entities in the original leaf node and redistribute them.
//     for entity in quad.take_entities() {
//         let quadrant = determine_quadrant(position_query.get(entity).unwrap().0, bounds.center());
//
//         // Place the entity into the appropriate child node.
//         child_entities[quadrant].push(entity);
//     }
//
//     // Create the new internal node with child nodes.
//     *quad = QuadTree::Internal {
//         bounds: bounds.clone(),
//         north_west: Box::new(QuadTree::Leaf {
//             bounds: child_bounds[0].clone(),
//             entities: child_entities[0].clone(),
//         }),
//         north_east: Box::new(QuadTree::Leaf {
//             bounds: child_bounds[1].clone(),
//             entities: child_entities[1].clone(),
//         }),
//         south_west: Box::new(QuadTree::Leaf {
//             bounds: child_bounds[2].clone(),
//             entities: child_entities[2].clone(),
//         }),
//         south_east: Box::new(QuadTree::Leaf {
//             bounds: child_bounds[3].clone(),
//             entities: child_entities[3].clone(),
//         }),
//     };
// }
//
// fn calculate_child_bounds(parent_bounds: &Rect) -> Vec<Rect> {
//     let center = parent_bounds.center();
//     let half_width = parent_bounds.width() / 2.0;
//     let half_height = parent_bounds.height() / 2.0;
//
//     vec![
//         Rect {
//             min: bevy::math::Vec2::new(center.x - half_width, center.y),
//             max: bevy::math::Vec2::new(center.x, center.y + half_height),
//         },
//         Rect {
//             min: bevy::math::Vec2::new(center.x, center.y),
//             max: bevy::math::Vec2::new(center.x + half_width, center.y + half_height),
//         },
//         Rect {
//             min: bevy::math::Vec2::new(center.x - half_width, center.y - half_height),
//             max: bevy::math::Vec2::new(center.x, center.y),
//         },
//         Rect {
//             min: bevy::math::Vec2::new(center.x, center.y - half_height),
//             max: bevy::math::Vec2::new(center.x + half_width, center.y),
//         },
//     ]
// }
//
// pub fn determine_quadrant(position: &Position, bounds: &Rect) -> Quadrant {
//     let center = bounds.center();
//     let x = position.0.x;
//     let y = position.0.y;
//     if x < center.x {
//         if y < center.y {
//             Quadrant::NorthWest
//         } else {
//             Quadrant::SouthWest
//         }
//     } else {
//         if y < center.y {
//             Quadrant::NorthEast
//         } else {
//             Quadrant::SouthEast
//         }
//     }
// }
//
// pub fn get_bounds_for_quadrant(quadrant: Quadrant, bounds: &Rect) -> Rect {
//     match quadrant {
//         Quadrant::NorthWest => {
//             Rect {
//                 min: Vec2::new(bounds.min.x, bounds.center().y),
//                 max: Vec2::new(bounds.center().x, bounds.max.y),
//             }
//         }
//         Quadrant::NorthEast => {
//             Rect {
//                 min: bounds.center().clone(),
//                 max: bounds.max.clone(),
//             }
//         }
//         Quadrant::SouthWest => {
//             Rect {
//                 min: bounds.min.clone(),
//                 max: bounds.center().clone(),
//             }
//         }
//         Quadrant::SouthEast => {
//             Rect {
//                 min: Vec2::new(bounds.center().x, bounds.min.y),
//                 max: Vec2::new(bounds.max.x, bounds.center().y),
//             }
//         }
//     }
// }
//
// pub fn insert_entity_into_quad_tree(entity: Entity, position: &Position, mut quad: &Quad) {
//     match quad {
//         Quad::Leaf { mut entities } => {
//             entities.insert(entity);
//             quad = &Quad::Leaf { entities };
//         }
//         Quad::Internal { bounds, mut quads } => {
//             let quadrant = determine_quadrant(position, &bounds);
//             if !quads.contains_key(&quadrant) {
//                 let quad_bounds = get_bounds_for_quadrant(quadrant, &bounds);
//                 quads.insert(quadrant, create_quad_tree_from_bounds(quad_bounds));
//             }
//             if let Some(q) = quads.get(&quadrant) {
//                 insert_entity_into_quad_tree(entity, position, q);
//             }
//         }
//     }
// }

use bevy::log::info;
use bevy::prelude::{Entity, Query, ResMut};
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::Position;
use crate::components::{QuadCoord, QuadStore, Rebuild};

pub fn naive_quad_system(
    mut query: Query<(Entity, &Position, &mut QuadCoord)>,
    mut quad_store: ResMut<QuadStore>,
) {
    let mut iter = query.iter_mut();
    let quad_store = quad_store.into_inner();
    match quad_store.rebuild_store {
        Rebuild::KeepQuadSize => {
            quad_store.largest_count = 0;
        }
        Rebuild::ShrinkQuadSize => {
            if quad_store.quad_size <= quad_store.min_quad_size {
                quad_store.rebuild_store = Rebuild::KeepQuadSize;
            } else {
                info!("Shrinking quad size");
                quad_store.entities.clear();
                quad_store.quad_size = (quad_store.quad_size / 2.0).clamp(quad_store.min_quad_size, quad_store.max_quad_size);
                quad_store.rebuild_store = Rebuild::KeepQuadSize;
                info!("Size: {:.2}", quad_store.quad_size);
            }
        }
        Rebuild::GrowQuadSize => {
            if quad_store.quad_size >= quad_store.max_quad_size {
                quad_store.rebuild_store = Rebuild::KeepQuadSize;
            } else {
                info!("Growing quad size");
                quad_store.entities.clear();
                quad_store.quad_size = (quad_store.quad_size * 2.0).clamp(quad_store.min_quad_size, quad_store.max_quad_size);
                quad_store.rebuild_store = Rebuild::KeepQuadSize;
                info!("Size: {:.2}", quad_store.quad_size);
            }
        }
    }
    while let Some((entity, position, mut quad_coord)) = iter.next() {
        let new_coord = QuadCoord::new(
            (position.0.x / quad_store.quad_size).floor() as i32,
            (position.0.y / quad_store.quad_size).floor() as i32,
        );

        if !new_coord.eq(&quad_coord) {
            if !quad_store.entities.contains_key(&new_coord) {
                quad_store.entities.insert(new_coord, HashSet::new());
            }
            let old_coord = quad_coord.clone();
            if let Some(old_set) = quad_store.entities.get_mut(&old_coord) {
                old_set.remove(&entity);
                if old_set.is_empty() {
                    quad_store.entities.remove(&old_coord);
                }
            }
            // if quad_store.entities.contains_key(&old_coord) {
            //     let set = quad_store.entities.get_mut(&old_coord).unwrap();
            //     set.remove(&entity);
            //     if set.is_empty() {
            //         quad_store.entities.remove(&old_coord);
            //     }
            // }

            if let Some(set) = quad_store.entities.get_mut(&new_coord) {
                set.insert(entity);
                if set.len() > quad_store.largest_count {
                    quad_store.largest_count = set.len();
                }
            } else {
                quad_store.entities.insert(new_coord, HashSet::new());
                quad_store.entities.get_mut(&new_coord).unwrap().insert(entity);
            }

            quad_coord.x = new_coord.x;
            quad_coord.y = new_coord.y;
        }
    }
    if quad_store.largest_count > quad_store.max_entities {
        quad_store.rebuild_store = Rebuild::ShrinkQuadSize;
    } else if quad_store.largest_count < quad_store.min_entities {
        quad_store.rebuild_store = Rebuild::GrowQuadSize;
    }
}