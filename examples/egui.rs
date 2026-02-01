use bevy::{
    log::{LogPlugin},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_ss2197_console::{
    ConsoleAppExt, ConsolePlugin,
    command::{BuildError, ConsoleCommand},
    frontends::egui::{EguiConsolePlugin, EguiConsoleSettings},
    logging::create_layer,
    parser::RawCommand,
};

#[derive(Event, ConsoleCommand, Debug)]
struct Foo {
    first: i32,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(LogPlugin {
        // level: log::Level::INFO,
        custom_layer: create_layer,
        ..default()
    }))
    .add_plugins(ConsolePlugin::default())
    .add_plugins(EguiPlugin::default())
    .add_plugins(EguiConsolePlugin::default())
    .add_systems(Startup, setup_example)
    .add_observer(on_foo)
    .register_command::<Foo>();

    app.run();
}

fn on_foo(cmd: On<Foo>) {
    info!("Received foo! {}", cmd.first);
}

fn setup_example(mut commands: Commands, settings: Res<EguiConsoleSettings>) {
    commands.spawn((Camera::default(), Camera2d));

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        Text::new(format!(
            "Press {:?} to toggle the console!",
            settings.toggle_key
        )),
    ));
}
