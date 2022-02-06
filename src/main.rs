use bevy::prelude::*;

#[derive(Default)]
struct ModelEntity {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>
}

struct Mobs {
    pub cube: ModelEntity
}

impl Mobs {
    pub fn create_cube_bundle(&self, pos: Vec3) -> PbrBundle {
        PbrBundle {
            mesh: self.cube.mesh.clone(),
            material: self.cube.material.clone(),
            transform: Transform::from_translation(pos),
            ..Default::default()
        }
    }
}

impl FromWorld for Mobs {
    fn from_world(world: &mut World) -> Self {
        let mut mesh_assets = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 }));
        let mut material_assets = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        let mat_handle = material_assets.add(Color::rgb(0.8, 0.7, 0.6).into());

        Mobs {
            cube: ModelEntity {
                mesh: mesh_handle,
                material: mat_handle,
            }
        }
    }
}

struct SpawnCube(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Mobs>()
        .add_event::<SpawnCube>()
        .add_startup_system(setup_game_world)
        .add_system(handle_input.before("spawn_event"))
        .add_system(spawn_cube.label("spawn_event"))
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
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn handle_input(mut ev_spawn_cube: EventWriter<SpawnCube>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::W) {
        ev_spawn_cube.send(SpawnCube(Vec3::ONE))
    }
}

fn spawn_cube(
    mut commands: Commands,
    mobs: Res<Mobs>,
    mut ev_spawn_cube: EventReader<SpawnCube>,) {
    for ev in ev_spawn_cube.iter() {
        commands.spawn_bundle(mobs.create_cube_bundle(ev.0));
    }
}