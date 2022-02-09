use crate::input;
use bevy::prelude::*;
mod tree;
mod ui;

pub struct PostFlushEvent;

pub fn handle_inputs(inputs: Res<input::Inputs>, mut app_state: ResMut<State<super::AppState>>) {
    if inputs.exit_dialogue {
        app_state.set(super::AppState::Game).unwrap();
    }
}

pub fn response_button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, &ui::UIResponseButton),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut dialogue_tree: ResMut<tree::DialogueTree>,
    mut app_state: ResMut<State<super::AppState>>,
) {
    for (interaction, children, response_btn) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                // TODO: probably inefficient
                if let Some(node) = dialogue_tree.root.responses[response_btn.response_index]
                    .dialogue_node
                    .clone()
                {
                    // if there's new dialogue associated with the response, set the root dialogue to this
                    dialogue_tree.root = node;
                } else {
                    // drop dialogue entirely if there's nothing else to be said
                    app_state.set(super::AppState::Game).unwrap();
                }
            }
            Interaction::Hovered => {
                text.sections[0].style.color = ui::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].style.color = ui::NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_dialogue_ui(
    mut commands: Commands,
    ui_data: Res<ui::UIData>,
    mut dialogue_tree: ResMut<tree::DialogueTree>,
) {
    // Root UI elements
    commands
        .spawn_bundle(ui_data.build_root_node())
        .with_children(|parent| {
            parent
                .spawn_bundle(ui_data.build_bottom_bar())
                .with_children(|parent| {
                    parent.spawn_bundle(ui_data.build_response_container());
                    parent.spawn_bundle(ui_data.build_dialogue_text());
                });
        });

    // spawn dialogue tree
    dialogue_tree.regenerate();
}

pub fn flush_dialogue_ui(
    mut commands: Commands,
    dialogue_tree: Res<tree::DialogueTree>,
    container_query: Query<Entity, With<ui::UIResponseContainer>>,
    mut evw: EventWriter<PostFlushEvent>,
) {
    if dialogue_tree.is_changed() {
        let container = container_query.single();
        // Clear old response container buttons
        commands.entity(container).despawn_descendants();
        evw.send(PostFlushEvent);
    }
}

pub fn update_dialogue_text_ui(
    mut query: Query<&mut Text, With<ui::UIDialogueText>>,
    dialogue_tree: Res<tree::DialogueTree>,
    mut evr: EventReader<PostFlushEvent>,
) {
    // Catch dialogue flush event
    for _ in evr.iter() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", dialogue_tree.root.text);
        }
    }
}

pub fn update_dialogue_response_ui(
    mut commands: Commands,
    dialogue_tree: Res<tree::DialogueTree>,
    container_query: Query<Entity, With<ui::UIResponseContainer>>,
    ui_data: Res<ui::UIData>,
    mut evr: EventReader<PostFlushEvent>,
) {
    // Catch dialogue flush event
    for _ in evr.iter() {
        let container = container_query.single();
        // Iterate over possible responses in root node
        for (i, response_node) in dialogue_tree.root.responses.iter().enumerate() {
            // Add response buttons to container
            commands.entity(container).with_children(|parent| {
                parent
                    .spawn_bundle(ui_data.build_response_button(i))
                    .with_children(|parent| {
                        parent.spawn_bundle(
                            ui_data.build_response_button_text(response_node.text.as_str()),
                        );
                    });
            });
        }
    }
}

pub fn cleanup_dialogue_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<ui::UIDialogueRoot>>,
) {
    let ui_root = ui_query.single();
    commands.entity(ui_root).despawn_recursive();
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<tree::DialogueTree>()
            .add_event::<PostFlushEvent>()
            .init_resource::<ui::UIData>()
            .add_system_set(
                SystemSet::on_enter(super::AppState::Dialogue)
                    .with_system(setup_dialogue_ui)
                    .with_system(crate::unlock_cursor),
            )
            .add_system_set(
                SystemSet::on_update(super::AppState::Dialogue)
                    .with_system(handle_inputs.after("input"))
                    .with_system(update_dialogue_response_ui.before("flush"))
                    .with_system(update_dialogue_text_ui.before("flush"))
                    .with_system(response_button_system.before("flush"))
                    .with_system(flush_dialogue_ui.label("flush")),
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Dialogue)
                    .with_system(cleanup_dialogue_ui)
                    .with_system(crate::lock_cursor),
            );
    }
}
