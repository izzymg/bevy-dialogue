use crate::input;
use bevy::prelude::*;
mod tree;

const NORMAL_BUTTON: Color = Color::rgb(0.98, 0.98, 0.98);
const HOVERED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);

#[derive(Component)]
pub struct DialogueUIRoot;

#[derive(Component)]
pub struct ResponseUIButton {
    // index of the response in the current dialogue tree root
    response_index: usize,
}

impl ResponseUIButton {
    fn new(response_index: usize) -> Self {
        Self { response_index }
    }
}

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct ResponseUIContainer;

pub struct PostFlushEvent;


pub fn handle_inputs(inputs: Res<input::Inputs>, mut app_state: ResMut<State<super::AppState>>) {
    if inputs.exit_dialogue {
        app_state.set(super::AppState::Game).unwrap();
    }
}

pub fn response_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ResponseUIButton),
        Changed<Interaction>,
    >,
    mut dialogue_tree: ResMut<tree::DialogueTreeRes>,
    mut app_state: ResMut<State<super::AppState>>,
) {
    for (interaction, mut color, response_btn) in interaction_query.iter_mut() {
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
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_dialogue_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root UI elements
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(DialogueUIRoot)
        .with_children(|parent| {
            // Bottom bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::rgb(0.55, 0.25, 0.25).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::rgb(0.25, 0.35, 0.25).into(),
                            ..Default::default()
                        })
                        .insert(ResponseUIContainer);
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                ..Default::default()
                            },

                            text: Text::with_section(
                                "...".to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(DialogueText);
                });
        });
}

pub fn flush_dialogue_ui(
    mut commands: Commands,
    dialogue_tree: Res<tree::DialogueTreeRes>,
    container_query: Query<Entity, With<ResponseUIContainer>>,
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
    mut query: Query<&mut Text, With<DialogueText>>,
    dialogue_tree: Res<tree::DialogueTreeRes>,
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
    dialogue_tree: Res<tree::DialogueTreeRes>,
    container_query: Query<Entity, With<ResponseUIContainer>>,
    asset_server: Res<AssetServer>,
    mut evr: EventReader<PostFlushEvent>,
) {
    // Catch dialogue flush event
    for _ in evr.iter() {
        // Iterate over possible responses in root node
        let container = container_query.single();
        for (i, response_node) in dialogue_tree.root.responses.iter().enumerate() {
            // Add response buttons to container
            commands.entity(container).with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(ResponseUIButton::new(i))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                response_node.text.as_str(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            });
        }
    }
}

pub fn cleanup_dialogue_ui(mut commands: Commands, ui_query: Query<Entity, With<DialogueUIRoot>>) {
    let ui_root = ui_query.single();
    commands.entity(ui_root).despawn_recursive();
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<tree::DialogueTreeRes>()
            .add_event::<PostFlushEvent>()
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
