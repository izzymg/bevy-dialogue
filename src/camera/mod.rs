use crate::input;
use crate::physics;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
struct CameraBundle {
    main_cam: MainCamera,
    velocity: physics::Velocity,

    #[bundle]
    cam: PerspectiveCameraBundle,
}

impl CameraBundle {
    fn new() -> Self {
        Self {
            cam: PerspectiveCameraBundle {
                transform: Transform::from_xyz(0., 1., 3.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            main_cam: MainCamera {},
            velocity: physics::Velocity(Vec3::ZERO),
        }
    }
}

pub fn startup_spawn_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(CameraBundle::new());
    commands.spawn_bundle(UiCameraBundle::default());

    let win = windows.get_primary_mut().unwrap();
    win.set_cursor_lock_mode(true);
    win.set_cursor_visibility(false);
}

pub fn camera_handle_input(
    inputs: Res<input::Inputs>,
    mut query: Query<(&mut physics::Velocity, &mut Transform), With<MainCamera>>,
) {
    let (mut cam_velocity, mut trans) = query.single_mut();
    trans.rotation =
        Quat::from_rotation_y(inputs.rot_dir.x) * Quat::from_rotation_x(inputs.rot_dir.y);
    cam_velocity.0 = trans.rotation * -inputs.wish_dir;
}
