use crate::ui;
use bevy::prelude::*;
mod stages;
mod tree;

pub fn on_response_chosen(
    mut evr: EventReader<ui::ResponseButtonClicked>,
    mut dialogue_tree: ResMut<tree::DialogueTree>,
    mut app_state: ResMut<State<super::AppState>>,
) {
    for e in evr.iter() {
        // Pull the relevant response node out of its vec
        let response_node = dialogue_tree.root.responses.swap_remove(e.0);

        if let Some(node) = response_node.dialogue_node {
            // if there's new dialogue associated with the response, set the root dialogue to this
            dialogue_tree.root = node;
        } else {
            // drop dialogue entirely if there's nothing else to be said
            app_state.set(super::AppState::Game).unwrap();
        }
    }
}

pub fn setup_dialogue(
    mut dialogue_tree: ResMut<tree::DialogueTree>,
    dialogue_stage: Res<stages::DialogueStage>,
) {
    dialogue_tree.load_stage(&dialogue_stage.stage);
}

pub fn update_dialogue(
    dialogue_tree: ResMut<tree::DialogueTree>,
    mut evw: EventWriter<ui::UpdateDialogueUIEvent>,
) {
    if dialogue_tree.is_changed() {
        evw.send(ui::UpdateDialogueUIEvent {
            dialogue_text: dialogue_tree.root.text.clone(),
            response_buttons: dialogue_tree
                .root
                .responses
                .iter()
                .enumerate()
                .map(|(i, response)| ui::ResponseButtonElementData {
                    text: response.text.clone(),
                    id: i,
                    skip: false,
                })
                .collect(),
        })
    }
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<tree::DialogueTree>()
            .init_resource::<stages::DialogueStage>()
            .add_system_set(
                SystemSet::on_update(super::AppState::Game).with_system(stages::progress_stages),
            )
            .add_system_set(
                SystemSet::on_enter(super::AppState::Dialogue)
                    .with_system(setup_dialogue)
                    .with_system(crate::unlock_cursor),
            )
            .add_system_set(
                SystemSet::on_update(super::AppState::Dialogue)
                    .with_system(on_response_chosen)
                    .with_system(update_dialogue),
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Dialogue).with_system(crate::lock_cursor),
            );
    }
}
