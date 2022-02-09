use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.98, 0.98, 0.98);
pub const HOVERED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);

#[derive(Component)]
pub struct UIDialogueRoot;

#[derive(Component)]
pub struct UIDialogueText;

#[derive(Component)]
pub struct UIResponseContainer;

#[derive(Bundle)]
pub struct UIDialogueRootBundle {
    tag: UIDialogueRoot,
    #[bundle]
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
pub struct UIResponseContainerBundle {
    tag: UIResponseContainer,
    #[bundle]
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
pub struct UIDialogueTextBundle {
    tag: UIDialogueText,
    #[bundle]
    text_bundle: TextBundle,
}

#[derive(Component)]
pub struct UIResponseButton {
    // index of the response in the current dialogue tree root
    pub response_index: usize,
}

#[derive(Bundle)]
pub struct UIResponseButtonBundle {
    tag: UIResponseButton,
    #[bundle]
    button_bundle: ButtonBundle,
}

// Resource
pub struct UIData {
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
    pub fn build_root_node(&self) -> UIDialogueRootBundle {
        UIDialogueRootBundle {
            tag: UIDialogueRoot,
            node_bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            },
        }
    }

    pub fn build_bottom_bar(&self) -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::rgb(0.55, 0.25, 0.25).into(),
            ..Default::default()
        }
    }

    pub fn build_response_container(&self) -> UIResponseContainerBundle {
        UIResponseContainerBundle {
            tag: UIResponseContainer,
            node_bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                color: Color::rgb(0.25, 0.35, 0.25).into(),
                ..Default::default()
            },
        }
    }

    pub fn build_dialogue_text(&self) -> UIDialogueTextBundle {
        UIDialogueTextBundle {
            tag: UIDialogueText,
            text_bundle: TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                    ..Default::default()
                },

                text: Text::with_section(
                    "...".to_string(),
                    TextStyle {
                        font: self.font_handle.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            },
        }
    }

    pub fn build_response_button(&self, response_index: usize) -> UIResponseButtonBundle {
        UIResponseButtonBundle {
            tag: UIResponseButton { response_index },
            button_bundle: ButtonBundle {
                style: Style {
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: NORMAL_BUTTON.into(),
                ..Default::default()
            },
        }
    }

    pub fn build_response_button_text(&self, text: &str) -> TextBundle {
        TextBundle {
            text: Text::with_section(
                text,
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
