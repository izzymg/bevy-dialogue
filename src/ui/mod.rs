use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.98, 0.98, 0.98);
const HOVERED_BUTTON: Color = Color::rgb(0.70, 0.70, 0.70);

#[derive(Component)]
struct DialogueRootElement;

#[derive(Component)]
struct DialogueTextElement;

#[derive(Component)]
struct ResponseContainerElement;

#[derive(Bundle)]
struct UIDialogueRootBundle {
    tag: DialogueRootElement,
    #[bundle]
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
struct DialogueContainerElementBundle {
    tag: ResponseContainerElement,
    #[bundle]
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
struct DialogueTextElementBundle {
    tag: DialogueTextElement,
    #[bundle]
    text_bundle: TextBundle,
}

#[derive(Component)]
struct ResponseButtonElement {
    // index of the response in the current dialogue tree root
    pub response_index: usize,
}

#[derive(Bundle)]
struct ResponseButtonElementBundle {
    tag: ResponseButtonElement,
    #[bundle]
    button_bundle: ButtonBundle,
}

// Resource
struct UIData {
    font_handle: Handle<Font>,
}

impl FromWorld for UIData {
    fn from_world(world: &mut World) -> Self {
        let server = world.get_resource::<AssetServer>().unwrap();
        let font_handle = server.load("fonts/FiraCode-Regular.ttf");

        Self { font_handle }
    }
}

impl UIData {
    fn build_root_node(&self) -> UIDialogueRootBundle {
        UIDialogueRootBundle {
            tag: DialogueRootElement,
            node_bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                transform: Transform::from_scale(Vec3::ZERO),
                ..Default::default()
            },
        }
    }

    fn build_bottom_bar(&self) -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: Rect::all(Val::Percent(5.0)),
                ..Default::default()
            },
            color: Color::rgb(0.13, 0.13, 0.13).into(),
            ..Default::default()
        }
    }

    fn build_response_container(&self) -> DialogueContainerElementBundle {
        DialogueContainerElementBundle {
            tag: ResponseContainerElement,
            node_bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                color: Color::rgb(0.13, 0.13, 0.13).into(),
                ..Default::default()
            },
        }
    }

    fn build_dialogue_text(&self) -> DialogueTextElementBundle {
        DialogueTextElementBundle {
            tag: DialogueTextElement,
            text_bundle: TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                    max_size: Size {
                        width: Val::Px(900.0),
                        height: Val::Undefined,
                    },
                    margin: Rect {
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },

                text: Text::with_section(
                    "...".to_string(),
                    TextStyle {
                        font: self.font_handle.clone(),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        vertical: VerticalAlign::Bottom,
                    },
                ),
                ..Default::default()
            },
        }
    }

    fn build_response_button(&self, response_index: usize) -> ResponseButtonElementBundle {
        ResponseButtonElementBundle {
            tag: ResponseButtonElement { response_index },
            button_bundle: ButtonBundle {
                style: Style {
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            },
        }
    }

    fn build_response_button_text(&self, text: &str) -> TextBundle {
        TextBundle {
            text: Text::with_section(
                format!("> {}", text),
                TextStyle {
                    font: self.font_handle.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        }
    }
}

fn setup_dialogue_ui(mut commands: Commands, ui_data: Res<UIData>) {
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
}

fn show_dialogue_ui(mut root_query: Query<&mut Transform, With<DialogueRootElement>>) {
    for mut root_tx in root_query.iter_mut() {
        root_tx.scale = Vec3::ONE;
    }
}

fn hide_dialogue_ui(mut root_query: Query<&mut Transform, With<DialogueRootElement>>) {
    for mut root_tx in root_query.iter_mut() {
        root_tx.scale = Vec3::ZERO;
    }
}

#[derive(Clone)]
pub struct ResponseButtonElementData {
    pub text: std::string::String,
    pub id: usize,
    pub skip: bool,
}

// Used to re-render dialogue over two frames
struct PostDialogueFlushEvent(UpdateDialogueUIEvent);

pub struct UpdateDialogueUIEvent {
    pub dialogue_text: std::string::String,
    pub response_buttons: Vec<ResponseButtonElementData>,
}

// Catch dialogue updates, flush dialogue in this frame, causing refresh next frame
fn flush_dialogue_ui(
    mut commands: Commands,
    mut evr: EventReader<UpdateDialogueUIEvent>,
    mut evw: EventWriter<PostDialogueFlushEvent>,
    container_query: Query<Entity, With<ResponseContainerElement>>,
) {
    // Container may not exist, so only parse the event once it does.
    for container in container_query.iter() {
        for ev in evr.iter() {
            // Clear old response container buttons
            commands.entity(container).despawn_descendants();
            // TODO: use system chaining
            evw.send(PostDialogueFlushEvent(UpdateDialogueUIEvent {
                dialogue_text: ev.dialogue_text.clone(),
                response_buttons: ev.response_buttons.clone(),
            }));
        }
    }
}

// Refresh dialogue in response to a flush event
fn refresh_dialogue_response_ui(
    mut commands: Commands,
    mut evr: EventReader<PostDialogueFlushEvent>,
    container_query: Query<Entity, With<ResponseContainerElement>>,
    ui_data: Res<UIData>,
) {
    // Container may not exist, so only parse the event once it does.
    for container in container_query.iter() {
        for ev in evr.iter() {
            println!("Post flsuh");
            // Add response buttons to container
            ev.0.response_buttons
                .iter()
                .filter(|r| !r.skip)
                .for_each(|response_button_data| {
                    commands.entity(container).with_children(|parent| {
                        parent
                            .spawn_bundle(ui_data.build_response_button(response_button_data.id))
                            .with_children(|parent| {
                                parent.spawn_bundle(ui_data.build_response_button_text(
                                    response_button_data.text.as_str(),
                                ));
                            });
                    });
                });
        }
    }
}

fn refresh_dialogue_text_ui(
    mut query: Query<&mut Text, With<DialogueTextElement>>,
    mut evr: EventReader<PostDialogueFlushEvent>,
) {
    // Catch dialogue flush event
    for e in evr.iter() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", e.0.dialogue_text);
        }
    }
}

pub struct ResponseButtonClicked(pub usize);

fn response_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &Children, &ResponseButtonElement),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut evw: EventWriter<ResponseButtonClicked>,
) {
    for (interaction, children, response_btn) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                evw.send(ResponseButtonClicked(response_btn.response_index));
            }
            Interaction::Hovered => {
                text.sections[0].style.color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].style.color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIData>()
            .add_event::<ResponseButtonClicked>()
            .add_event::<UpdateDialogueUIEvent>()
            .add_event::<PostDialogueFlushEvent>()
            .add_startup_system(setup_dialogue_ui)
            .add_system_set(
                SystemSet::on_enter(super::AppState::Dialogue).with_system(show_dialogue_ui),
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Dialogue).with_system(hide_dialogue_ui),
            )
            .add_system(flush_dialogue_ui.label("ui-dialogue-flush"))
            .add_system(refresh_dialogue_response_ui.before("ui-dialogue-flush"))
            .add_system(refresh_dialogue_text_ui.before("ui-dialogue-flush"))
            .add_system(response_button_interactions);
    }
}
