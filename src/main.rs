use bevy::prelude::*;

mod camera;
mod input;
mod math;
mod mobs;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(input::InputPlugin)
        .add_startup_system(setup_game_world)
        .add_startup_system(camera::startup_spawn_camera)
        .add_startup_system(mobs::setup_spawn_mob)
        .add_system(camera::camera_handle_input.after("input").before("physics"))
        .add_system(physics::apply_velocity.label("physics").after("input"))
        .add_system(mobs::check_interactable.after("physics"))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
