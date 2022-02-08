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
