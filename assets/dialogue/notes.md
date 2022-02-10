raw dialogue is stored in a yaml file

when we move stages or load in a character, we load the dialogue from the yaml file for that stage

each response node can have an id marking whether or not it can only be visited once

tree::response_is_dead(self, &response_node) -> bool
	if !response_node.dialogue
		true
	else for response in response_node.dialogue.responses
		if self.visited.has(response.visited_id)
		    false
	true


when entering a new stage:
    load stage dialogue from yaml file
    set tree root to dialogue root from yaml file
    set visited to empty vec

when entering dialogue:
    // nothing more to say, move back to default stage (updates tree)
    if for all tree.root responses response_is_dead(response) true
        send_update_stage_event(default)
    else
        send_update_dialogue_event()

when updating dialogue (new response etc)
    take each response to the current root
