use bevy::prelude::*;

mod camera;
mod input;
mod interact;
mod math;
mod mobs;
mod physics;
mod dialogue;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Game,
    Dialogue,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(input::InputPlugin)

        .insert_resource(interact::Interaction::default())

        .add_startup_system(setup_game_world)
        .add_startup_system(camera::startup_spawn_camera)
        .add_startup_system(mobs::setup_spawn_mob)
        .add_system(bevy::input::system::exit_on_esc_system)
        
        .add_state(AppState::Game)
        .add_system_set(
            SystemSet::on_update(AppState::Game)
            .with_system(camera::camera_handle_input.after("input").before("physics"))
            .with_system(physics::apply_velocity.label("physics").after("input"))
            .with_system(interact::check_interactable.after("physics"))
            .with_system(interact::start_interaction.after("input"))
        )

        .add_system_set(
            SystemSet::on_update(AppState::Dialogue)
            .with_system(dialogue::handle_inputs.after("input"))
        )


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
