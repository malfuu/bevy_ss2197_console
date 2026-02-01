use core::fmt;

use bevy::ecs::system::Commands;

use crate::parser::RawCommand;

// TODO: change to pub(crate)
pub enum BuildError {
    /// A required argument was not provided.
    MissingArgument,
    /// An argument was provided but failed to parse into the expected type.
    ParseError,
    /// Catch all type
    Other(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::MissingArgument => {
                write!(f, "missing required argument")
            }
            BuildError::ParseError => {
                write!(f, "failed to parse argument")
            }
            BuildError::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

pub trait ConsoleCommand {
    /// Name of the command, when called upon.
    fn name() -> &'static str;
    /// Transform the tokens into typed command.
    fn build(command: RawCommand) -> Result<Box<Self>, BuildError>;
    /// Trigger the command
    fn trigger(self, commands: &mut Commands);
}

pub(crate) type CommandHandler = Box<dyn Fn(RawCommand, &mut Commands) + Send + Sync>;

pub use space_console_derive::ConsoleCommand;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[derive(Event, ConsoleCommand)]
    #[console(name = "foo")]
    struct _Foo {
        arg1: String,
        arg2: i32,
    }

    // TODO
    // #[derive(ConsoleCommand)]
    // #[console(name = "bar")]
    // struct Bar (f32, bool);

    // TODO
    // #[derive(ConsoleCommand)]
    // #[console(name = "abc")]
    // enum Abc {
    //     Opt1 { x: i32 },
    //     Opt2 { x: i32, y: String },
    // }
}
