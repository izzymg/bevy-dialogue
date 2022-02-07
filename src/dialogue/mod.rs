use crate::input;
use bevy::prelude::*;

#[derive(Component)]
pub struct DialogueUIRoot;

pub fn handle_inputs(inputs: Res<input::Inputs>, mut app_state: ResMut<State<super::AppState>>) {
    if inputs.exit_dialogue {
        app_state.set(super::AppState::Game).unwrap();
    }
}

pub fn setup_dialogue(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(DialogueUIRoot)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                    ..Default::default()
                },
                color: Color::rgb(0.25, 0.25, 0.25).into(),
                ..Default::default()
            });
        });
}

pub fn cleanup_dialogue(mut commands: Commands, ui_query: Query<Entity, With<DialogueUIRoot>>) {
    let ui_root = ui_query.single();
    commands.entity(ui_root).despawn_recursive();
}
