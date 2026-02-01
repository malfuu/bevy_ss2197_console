pub mod command;
pub mod frontends;
pub mod logging;
pub mod parser;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    command::{CommandHandler, ConsoleCommand},
    parser::{RawCommand, parse_input},
};

/// Contains history and commands
#[derive(Resource)]
pub struct Console {
    history: Vec<String>,
    commands: HashMap<String, CommandHandler>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            history: default(),
            commands: default(),
        }
    }
}

impl Console {
    pub fn clear(&mut self) {
        self.history.clear();
    }
}

/// Send a string as input to a console
#[derive(Event, Deref, Debug)]
pub struct ConsoleInput(pub String);

fn on_process_input(input: On<ConsoleInput>, console: ResMut<Console>, mut commands: Commands) {
    let s = &**input;

    let raw_commands = parse_input(s);
    for cmd in raw_commands {
        if let Some(handler) = console.commands.get(&cmd.command) {
            handler(cmd, &mut commands);
        } else {
            // TODO: change not found feedback.
            warn!("Command not found: {}", cmd.command);
        }
    }
}

#[derive(Default)]
pub struct ConsolePlugin {
    // empty
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Console {
            ..Default::default()
        })
        .add_observer(on_process_input);
    }
}

pub trait ConsoleAppExt {
    fn register_command<T>(&mut self) -> &mut Self
    where
        T: ConsoleCommand;
}

impl ConsoleAppExt for App {
    fn register_command<T>(&mut self) -> &mut Self
    where
        T: ConsoleCommand,
    {
        let mut console = self.world_mut().resource_mut::<Console>();

        let name = T::name().to_string();

        let handler = Box::new(|raw: RawCommand, commands: &mut Commands| {
            let built = T::build(raw);
            match built {
                Ok(event) => event.trigger(commands),
                Err(error) => {
                    warn!("Got error on command: {}", error);
                }
            }
        });

        console.commands.insert(name, handler);

        self
    }
}
