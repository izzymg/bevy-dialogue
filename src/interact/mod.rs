use crate::{camera, input};
use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable(pub Vec3);

pub struct Interaction {
    pub current_interaction: Option<Entity>,
}

impl Default for Interaction {
    fn default() -> Self {
        Self {
            current_interaction: None,
        }
    }
}

fn point_in_aabb(a_center: Vec3, b_center: Vec3, b_bounds: Vec3) -> bool {
    let min_bounds = b_center - b_bounds;
    let max_bounds = b_center + b_bounds;
    (a_center.x >= min_bounds.x && a_center.x <= max_bounds.x)
        && (a_center.y >= min_bounds.y && a_center.y <= max_bounds.y)
        && (a_center.z >= min_bounds.z && a_center.z <= max_bounds.z)
}

pub fn check_interactable(
    interactable_query: Query<(&Transform, &Interactable, Entity)>,
    cam_query: Query<&Transform, With<camera::MainCamera>>,
    mut interaction: ResMut<Interaction>,
) {
    let cam_trans = cam_query.single();
    for (inter_trans, inter_bounds, entity) in interactable_query.iter() {
        let cam_point = cam_trans.translation + (cam_trans.forward() * 2.0);
        if point_in_aabb(cam_point, inter_trans.translation, inter_bounds.0) {
            // interactable found
            interaction.current_interaction = Some(entity);
        } else {
            interaction.current_interaction = None;
        }
    }
}

pub fn start_interaction(
    inputs: Res<input::Inputs>,
    interaction: Res<Interaction>,
    mut app_state: ResMut<State<super::AppState>>,
) {
    if inputs.interact {
        if let Some(_) = interaction.current_interaction {
            app_state.set(super::AppState::Dialogue).unwrap();
        }
    }
}
