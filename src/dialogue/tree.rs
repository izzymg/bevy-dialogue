pub struct DialogueNode {
    pub text: std::string::String,
    pub responses: Vec<ResponseNode>,
}

pub struct ResponseNode {
    pub text: std::string::String,
    pub dialogue_node: Option<DialogueNode>,
}
