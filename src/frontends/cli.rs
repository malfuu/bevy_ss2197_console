use bevy::prelude::*;

pub struct CliConsolePlugin;

impl Plugin for CliConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CliConsoleSettings>()
            .add_systems(Update, render_frontend);
    }
}

#[derive(Resource)]
pub struct CliConsoleSettings {
    pub prompt: String,
}

impl Default for CliConsoleSettings {
    fn default() -> Self {
        Self {
            prompt: String::from("> "),
        }
    }
}

fn render_frontend(mut _commands: Commands, _settings: Res<CliConsoleSettings>) {
    todo!()
}
