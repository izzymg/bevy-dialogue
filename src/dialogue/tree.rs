extern crate yaml_rust;

use std::fs::{self};

use bevy::prelude::*;
use yaml_rust::{yaml, YamlLoader};

#[derive(Clone, Debug, PartialEq)]
pub struct DialogueNode {
    pub text: std::string::String,
    pub responses: Vec<ResponseNode>,
}

impl Drop for DialogueNode {
    fn drop(&mut self) {
        println!("Dropped: {}", self.text);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResponseNode {
    pub text: std::string::String,
    pub dialogue_node: Option<DialogueNode>,
}

pub fn generate_dialogue_from_yaml(
    stage_str: &std::string::String,
    yaml_path: &str,
) -> DialogueNode {
    let docs = YamlLoader::load_from_str(&fs::read_to_string(yaml_path).unwrap()).unwrap();
    let doc = &docs[0];
    parse_dialogue_yaml(&doc[stage_str.as_str()]["dialogue"])
}

pub fn parse_dialogue_yaml(yaml: &yaml::Yaml) -> DialogueNode {
    let responses = yaml["responses"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|response_yaml| ResponseNode {
            text: response_yaml["text"].as_str().unwrap().to_string(),
            dialogue_node: if response_yaml["dialogue"].is_null() {
                None
            } else {
                Some(parse_dialogue_yaml(&response_yaml["dialogue"]))
            },
        })
        .collect();
    DialogueNode {
        text: yaml["text"].as_str().unwrap().into(),
        responses,
    }
}

#[test]
pub fn test_generate_dialogue_from_yaml() {
    let node = generate_dialogue_from_yaml(
        &"default".to_string(),
        "./assets/dialogue/test_dialogue.yaml",
    );
    assert_eq!(node.text, "Hi");
    assert_eq!(node.responses[0].text, "Hello");
    assert_eq!(
        node.responses[0].dialogue_node.as_ref().unwrap().text,
        "I can't talk now."
    );
    assert_eq!(
        node.responses[0].dialogue_node.as_ref().unwrap().responses[0].text,
        "Oh.."
    );
    assert_eq!(
        node.responses[0].dialogue_node.as_ref().unwrap().responses[0].dialogue_node,
        None
    );
    assert_eq!(node.responses[1].text, "Goodbye");
}

pub struct DialogueTree {
    pub root: DialogueNode,
}

impl DialogueTree {
    pub fn load_stage(&mut self, stage: &std::string::String) {
        self.root = generate_dialogue_from_yaml(stage, "./assets/dialogue/cube_dialogue.yaml");
    }

    fn new() -> Self {
        Self {
            root: generate_dialogue_from_yaml(
                &"default".to_string(),
                "./assets/dialogue/cube_dialogue.yaml",
            ),
        }
    }
}

impl FromWorld for DialogueTree {
    fn from_world(_: &mut World) -> Self {
        DialogueTree::new()
    }
}
