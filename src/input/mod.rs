use crate::math;
use bevy::{input::mouse::MouseMotion, prelude::*};

const ROT_SPEED: f32 = 0.001;
const PITCH_MIN: f32 = 0. * math::DEG_TO_RAD;
const PITCH_MAX: f32 = 360. * math::DEG_TO_RAD;
const YAW_MIN: f32 = -180. * math::DEG_TO_RAD;
const YAW_MAX: f32 = 180. * math::DEG_TO_RAD;

#[derive(Default)]
pub struct Inputs {
    pub wish_dir: Vec3,
    pub rot_dir: Vec2,

    pub interact: bool,
    pub exit_dialogue: bool,
}

fn wrap_rotation(n: f32, min: f32, max: f32) -> f32 {
    if n < min {
        max
    } else if n > max {
        min
    } else {
        n
    }
}

pub fn update_inputs(
    mut inputs: ResMut<Inputs>,
    keys: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut wish_dir = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        wish_dir.z = 1.;
    } else if keys.pressed(KeyCode::S) {
        wish_dir.z = -1.;
    }
    if keys.pressed(KeyCode::A) {
        wish_dir.x = 1.;
    }
    if keys.pressed(KeyCode::D) {
        wish_dir.x = -1.;
    }

    inputs.wish_dir = wish_dir;

    for ev in motion_evr.iter() {
        inputs.rot_dir.x = wrap_rotation(
            inputs.rot_dir.x - (ev.delta.x * ROT_SPEED),
            PITCH_MIN,
            PITCH_MAX,
        );
        inputs.rot_dir.y = math::clamp(
            inputs.rot_dir.y - (ev.delta.y * ROT_SPEED),
            YAW_MIN,
            YAW_MAX,
        );

        // println!("rot: {}", inputs.rot_dir * math::RAD_TO_DEG);
    }

    inputs.interact = keys.just_released(KeyCode::E);
    inputs.exit_dialogue = keys.just_released(KeyCode::Space);
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inputs::default())
            .add_system(update_inputs.label("input"));
    }
}
