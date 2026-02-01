use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, RichText, Ui},
};

use crate::{Console, ConsoleInput, ConsolePlugin};

#[derive(Default)]
pub struct EguiConsolePlugin {
    start_enabled: bool,
}

impl Plugin for EguiConsolePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<EguiPlugin>(), "EguiPlugin required!");

        assert!(
            app.is_plugin_added::<ConsolePlugin>(),
            "ConsolePlugin required for the frontend!"
        );

        app.insert_resource(EguiConsoleFrontend::new(self.start_enabled))
            .insert_resource(EguiConsoleSettings::default())
            .add_systems(Update, toggle_console)
            .add_systems(EguiPrimaryContextPass, ui_console);
    }
}

#[derive(Resource)]
pub struct EguiConsoleSettings {
    pub toggle_key: KeyCode,
}

impl Default for EguiConsoleSettings {
    fn default() -> Self {
        Self {
            toggle_key: KeyCode::Backquote,
        }
    }
}

#[derive(Resource, Default)]
struct EguiConsoleFrontend {
    visible: bool,
    input: String,
}

impl EguiConsoleFrontend {
    fn new(visible: bool) -> Self {
        Self {
            visible,
            input: String::new(),
        }
    }
}

fn toggle_console(
    keys: Res<ButtonInput<KeyCode>>,
    settings: ResMut<EguiConsoleSettings>,
    mut frontend: ResMut<EguiConsoleFrontend>,
) {
    if keys.just_pressed(settings.toggle_key) {
        frontend.visible = !frontend.visible;
    }
}

fn ui_console(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut res: ResMut<EguiConsoleFrontend>,
    console: Res<Console>,
) -> Result {
    if !res.visible {
        return Ok(());
    }

    egui::Window::new("Console")
        .default_size([500.0, 300.0])
        .show(contexts.ctx_mut()?, |ui| {
            ui.vertical(|ui| {
                ui_console_text(ui, &console);
                ui.separator();
                ui_console_input(ui, &mut commands, &mut res);
            });
        });

    Ok(())
}

#[inline]
fn ui_console_text(ui: &mut Ui, console: &Console) {
    egui::ScrollArea::vertical()
        .max_height(300.0)
        .min_scrolled_height(300.0)
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            for line in &console.history {
                let rich_text: RichText = line.into();

                ui.label(rich_text.monospace());
            }
        });
}

#[inline]
fn ui_console_input(ui: &mut Ui, commands: &mut Commands, res: &mut EguiConsoleFrontend) {
    let mut submit = false;

    ui.horizontal(|ui| {
        let text_edit = egui::TextEdit::singleline(&mut res.input)
            .desired_width(ui.available_width() - 100.0)
            .font(egui::TextStyle::Monospace);

        let response = ui.add(text_edit);

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            submit = true;
        }

        if ui.button("Enter").clicked() {
            submit = true;
        }
    });

    if submit && !res.input.is_empty() {
        let input = std::mem::take(&mut res.input);
        commands.trigger(ConsoleInput(input));
    }
}
