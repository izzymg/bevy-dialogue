use bevy::prelude::*;

pub struct DialogueStage {
    pub stage: std::string::String,
    timer: Timer,
}

impl FromWorld for DialogueStage {
    fn from_world(_: &mut World) -> DialogueStage {
        Self {
            stage: "default".into(),
            timer: Timer::from_seconds(30.0, false),
        }
    }
}

pub fn progress_stages(mut stage: ResMut<DialogueStage>, time: Res<Time>) {
    if stage.timer.tick(time.delta()).just_finished() {
        stage.stage = "stage_too_long".to_string();
    }
}
