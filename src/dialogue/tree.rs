use bevy::prelude::*;

#[derive(Clone)]
pub struct DialogueNode {
    pub text: std::string::String,
    pub responses: Vec<ResponseNode>,
}

impl Drop for DialogueNode {
    fn drop(&mut self) {
        println!("Dropped: {}", self.text);
    }
}

#[derive(Clone)]
pub struct ResponseNode {
    pub text: std::string::String,
    pub dialogue_node: Option<DialogueNode>,
}

pub struct DialogueTree {
    pub root: DialogueNode,
}

impl FromWorld for DialogueTree {
    fn from_world(_: &mut World) -> Self {
        let dialogue = DialogueNode {
            text: "Hi".into(),
            responses: vec![
                ResponseNode {
                    text: "Hello".into(),
                    dialogue_node: Some(DialogueNode {
                        text: "Leave me alone, now.".into(),
                        responses: vec![ResponseNode {
                            text: "Okay...".into(),
                            dialogue_node: None,
                        }],
                    }),
                },
                ResponseNode {
                    text: "Goodbye".into(),
                    dialogue_node: None,
                },
            ],
        };
        Self { root: dialogue }
    }
}
