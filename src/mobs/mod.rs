use bevy::{prelude::*};

use crate::camera;

#[derive(Component)]
pub struct Interactable(Vec3);

pub fn setup_spawn_mob(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),

            ..Default::default()
        })
        .insert(Interactable(Vec3::ONE * 1.0));
}

fn point_in_aabb(a_center: Vec3, b_center: Vec3, b_bounds: Vec3) -> bool {
    let min_bounds = b_center - b_bounds;
    let max_bounds = b_center + b_bounds;
    (a_center.x >= min_bounds.x && a_center.x <= max_bounds.x) &&
           (a_center.y >= min_bounds.y && a_center.y <= max_bounds.y) &&
           (a_center.z >= min_bounds.z && a_center.z <= max_bounds.z)
  }

pub fn check_interactable(
    interactable_query: Query<(&Transform, &Interactable)>,
    cam_query: Query<&Transform, With<camera::MainCamera>>,
) {
    let cam_trans = cam_query.single();
    for (inter_trans, inter_bounds) in interactable_query.iter() {
        let cam_point = cam_trans.translation + (cam_trans.forward() * 2.0);
        if point_in_aabb(cam_point, inter_trans.translation, inter_bounds.0) {
            println!("in box");
        }
    }
}
