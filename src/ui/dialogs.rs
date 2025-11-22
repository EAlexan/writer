use eframe::egui;
use crate::actions::{QuitAction, OpenAction};

/// Render the About dialog window
pub fn render_about_dialog(
    ctx: &egui::Context,
    show_about_window: &mut bool,
) {
    if *show_about_window {
        let mut close_requested = false;
        egui::Window::new("About the App")
            .open(show_about_window)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("My great Egui-App");
                ui.label("Version: 1.0.0");
                ui.separator();
                if ui.button("OK").clicked() {
                    close_requested = true;
                }
            });
        if close_requested {
            *show_about_window = false;
        }
    }
}

/// Render the quit confirmation dialog
pub fn render_quit_dialog(
    ctx: &egui::Context,
    show_quit_dialog: &mut bool,
) -> QuitAction {
    let mut action = QuitAction::None;
    
    if *show_quit_dialog {
        egui::Window::new("Unsaved Changes")
            .open(show_quit_dialog)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("You have unsaved changes. Do you want to save before quitting?");
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        action = QuitAction::Save;
                    }
                    if ui.button("Don't Save").clicked() {
                        action = QuitAction::DontSave;
                    }
                    if ui.button("Cancel").clicked() {
                        action = QuitAction::Cancel;
                    }
                });
            });
    }
    
    action
}

/// Render the open file confirmation dialog
pub fn render_open_dialog(
    ctx: &egui::Context,
    show_open_dialog: &mut bool,
) -> OpenAction {
    let mut action = OpenAction::None;
    
    if *show_open_dialog {
        egui::Window::new("Unsaved Changes")
            .open(show_open_dialog)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("You have unsaved changes. Do you want to save before opening a new file?");
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        action = OpenAction::Save;
                    }
                    if ui.button("Don't Save").clicked() {
                        action = OpenAction::DontSave;
                    }
                    if ui.button("Cancel").clicked() {
                        action = OpenAction::Cancel;
                    }
                });
            });
    }
    
    action
}

/// Render the error dialog
pub fn render_error_dialog(
    ctx: &egui::Context,
    show_error_dialog: &mut bool,
    error_message: &str,
) {
    if *show_error_dialog {
        let mut close_requested = false;
        egui::Window::new("Error")
            .open(show_error_dialog)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(error_message);
                ui.separator();
                if ui.button("OK").clicked() {
                    close_requested = true;
                }
            });
        if close_requested {
            *show_error_dialog = false;
        }
    }
}
