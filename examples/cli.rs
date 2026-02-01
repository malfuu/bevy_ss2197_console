use bevy::prelude::*;
use bevy_ss2197_console::{ConsolePlugin, frontends::cli::CliConsolePlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: bevy::window::ExitCondition::DontExit,
        ..default()
    }))
    .add_plugins(ConsolePlugin::default())
    .add_plugins(CliConsolePlugin);

    app.run();
}
