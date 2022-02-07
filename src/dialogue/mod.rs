use bevy::prelude::*;
use crate::input;

pub fn handle_inputs(inputs: Res<input::Inputs>, mut app_state: ResMut<State<super::AppState>>) {
    if inputs.exit_dialogue {
        app_state.set(super::AppState::Game).unwrap();
    }
}