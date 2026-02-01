//! Buffering tracing output, and sending it to the console
use std::{
    io::{BufRead, Write},
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use tracing_subscriber::Registry;

use crate::Console;

type ArcCursor = Arc<Mutex<std::io::Cursor<Vec<u8>>>>;

#[derive(Resource)]
struct BevyLogBuffer(ArcCursor);

/// Writer implementation which writes into a buffer resource inside the bevy world
struct BevyLogBufferWriter(ArcCursor);

impl Write for BevyLogBufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut lock = self
            .0
            .lock()
            .map_err(|e| std::io::Error::other(format!("Failed to lock buffer: {}", e)))?;
        lock.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut lock = self
            .0
            .lock()
            .map_err(|e| std::io::Error::other(format!("Failed to lock buffer: {}", e)))?;
        lock.flush()
    }
}

pub fn create_layer(
    app: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> {
    let buffer = Arc::new(Mutex::new(std::io::Cursor::new(Vec::new())));

    app.insert_resource(BevyLogBuffer(buffer.clone()))
        .add_systems(Update, buffer_output);

    let layer = tracing_subscriber::fmt::Layer::<Registry>::new()
        .with_target(false)
        .with_ansi(false)
        .with_writer(move || BevyLogBufferWriter(buffer.clone()));

    Some(Box::new(layer))
}

fn buffer_output(buffer: ResMut<BevyLogBuffer>, mut console: ResMut<Console>) {
    let mut buffer = buffer.0.lock().unwrap();

    let buffer = buffer.get_mut();
    for line in buffer.lines().map_while(Result::ok) {
        console.history.push(line);
    }

    buffer.clear();
}
