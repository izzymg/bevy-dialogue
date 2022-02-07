use crate::input;
use bevy::prelude::*;
mod tree;

const NORMAL_BUTTON: Color = Color::rgb(0.98, 0.98, 0.98);
const HOVERED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);

// UPDATE THE ROOT

pub struct DialogueUpdateEvent;

#[derive(Component)]
pub struct DialogueUIRoot;

#[derive(Component)]
pub struct ResponseUIButton(Option<&tree::DialogueNode>);

#[derive(Component)]
pub struct ResponseUIContainer;

pub fn handle_inputs(inputs: Res<input::Inputs>, mut app_state: ResMut<State<super::AppState>>) {
    if inputs.exit_dialogue {
        app_state.set(super::AppState::Game).unwrap();
    }
}

pub fn response_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ResponseUIButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                println!("Clicked");
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                println!("HOVERED");
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_dialogue(
    mut commands: Commands,
    mut evw: EventWriter<DialogueUpdateEvent>,
    dialogue_tree: Res<DialogueTreeRes>
) {
    // Unlock cursor

    // Root
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
            // Bottom bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.25, 0.25, 0.25).into(),
                    ..Default::default()
                })
                .insert(ResponseUIContainer);
        });

    evw.send(DialogueUpdateEvent(dialogue_tree.root));
}

pub fn update_dialogue_ui(
    mut commands: Commands,
    mut evr: EventReader<DialogueUpdateEvent>,
    container_query: Query<Entity, With<ResponseUIContainer>>,
    asset_server: Res<AssetServer>,
) {
    for e in evr.iter() {
        let container = container_query.single();

        for response in &e.0.responses {
            commands.entity(container).with_children(|parent| {
                // Response buttons
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // center button
                            margin: Rect::all(Val::Auto),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(ResponseUIButton(response.dialogue_node))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                response.text.as_str(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                                    font_size: 40.0,
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

pub fn cleanup_dialogue(mut commands: Commands, ui_query: Query<Entity, With<DialogueUIRoot>>) {
    let ui_root = ui_query.single();
    commands.entity(ui_root).despawn_recursive();
}

pub struct DialogueTreeRes {
    root: tree::DialogueNode,
}

impl FromWorld for DialogueTreeRes {
    fn from_world(world: &mut World) -> Self {
        let dialogue = tree::DialogueNode {
            text: "Hi".into(),
            responses: vec![tree::ResponseNode {
                text: "Hello".into(),
                dialogue_node: None,
            }],
        };
        Self {
            root: dialogue
        }
    }
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DialogueUpdateEvent>()
            .init_resource::<DialogueTreeRes>()
            // dialogue state
            .add_system_set(
                SystemSet::on_enter(super::AppState::Dialogue)
                    .with_system(setup_dialogue)
                    .with_system(crate::unlock_cursor),
            )
            .add_system_set(
                SystemSet::on_update(super::AppState::Dialogue)
                    .with_system(handle_inputs.after("input"))
                    .with_system(update_dialogue_ui)
                    .with_system(response_button_system),
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Dialogue)
                    .with_system(cleanup_dialogue)
                    .with_system(crate::lock_cursor),
            );
    }
}
