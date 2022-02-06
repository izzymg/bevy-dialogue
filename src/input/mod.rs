use bevy::prelude::*;

#[derive(Default)]
pub struct Inputs {
    pub wish_dir: Vec3
}

pub fn update_inputs(mut inputs: ResMut<Inputs>, keys: Res<Input<KeyCode>>) {
    let mut wish_dir = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        wish_dir.z = 1.;
    }
    else if keys.pressed(KeyCode::S) {
        wish_dir.z = -1.;
    }
    if keys.pressed(KeyCode::A) {
        wish_dir.x = 1.;
    }
    if keys.pressed(KeyCode::D) {
        wish_dir.x = -1.;
    }

    inputs.wish_dir = wish_dir;
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inputs{ wish_dir: Vec3::ZERO }).add_system(update_inputs.label("input"));
    }
}