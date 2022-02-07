use crate::interact;
use bevy::prelude::*;

pub fn setup_spawn_mob(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),

            ..Default::default()
        })
        .insert(interact::Interactable(Vec3::ONE * 1.0));
}
