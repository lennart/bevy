use super::Node;
use bevy_core::transform::run_on_hierarchy;
use bevy_ecs::{Entity, Query, Res};
use bevy_transform::prelude::{Children, Parent, Translation};
use bevy_window::Windows;
use glam::Vec2;

pub const UI_Z_STEP: f32 = 0.001;

#[derive(Clone)]
pub struct Rect {
    pub z: f32,
    pub size: Vec2,
}

pub fn ui_update_system(
    windows: Res<Windows>,
    mut node_query: Query<(Entity, &mut Node, &mut Translation)>,
    parent_query: Query<&Parent>,
    children_query: Query<&Children>,
) {
    let window_size = if let Some(window) = windows.get_primary() {
        Vec2::new(window.width as f32, window.height as f32)
    } else {
        return;
    };
    let orphan_nodes = node_query
        .iter()
        .iter()
        // TODO: replace this filter with a legion query filter (when SimpleQuery gets support for filters)
        .filter(|(entity, _, _)| parent_query.get::<Parent>(*entity).is_err())
        .map(|(e, _, _)| e)
        .collect::<Vec<Entity>>();
    let mut window_rect = Rect {
        z: 0.0,
        size: window_size,
    };

    let mut previous_sibling_result = Some(Rect {
        z: 999.0,
        size: window_size,
    });
    for entity in orphan_nodes {
        previous_sibling_result = run_on_hierarchy(
            &children_query,
            &mut node_query,
            entity,
            Some(&mut window_rect),
            previous_sibling_result,
            &mut update_node_entity,
        );
    }
}

fn update_node_entity(
    node_query: &mut Query<(Entity, &mut Node, &mut Translation)>,
    entity: Entity,
    parent_rect: Option<&mut Rect>,
    previous_rect: Option<Rect>,
) -> Option<Rect> {
    if let Ok(mut node) = node_query.get_mut::<Node>(entity) {
        if let Ok(mut translation) = node_query.get_mut::<Translation>(entity) {
            let parent_rect = parent_rect.unwrap();
            let mut z = parent_rect.z;
            if let Some(previous_rect) = previous_rect {
                z = previous_rect.z
            };

            z -= UI_Z_STEP;
            node.update(&mut translation, z - parent_rect.z, parent_rect.size);
            return Some(Rect { size: node.size, z });
        }
    }

    None
}
