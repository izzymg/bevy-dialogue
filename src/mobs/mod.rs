use crate::interact;
use bevy::prelude::*;
pub fn setup_spawn_mob(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/cube.gltf#Scene0"));
        })
        .insert(interact::Interactable(Vec3::ONE * 1.0));
}
