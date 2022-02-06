use bevy::prelude::*;
use crate::physics;
use crate::input;

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
struct CameraBundle {
    main_cam: MainCamera,
    velocity: physics::Velocity,

    #[bundle]
    cam: PerspectiveCameraBundle
}

impl CameraBundle {
    fn new() -> Self {
        Self {
            cam: PerspectiveCameraBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            main_cam: MainCamera {},
            velocity: physics::Velocity(Vec3::ZERO)
        }
    }
}

pub fn startup_spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(CameraBundle::new());
}

pub fn camera_handle_input(inputs: Res<input::Inputs>, mut query: Query<&mut physics::Velocity, With<MainCamera>>) {
    let mut cam_velocity = query.single_mut();
    cam_velocity.0 = -inputs.wish_dir;
}
