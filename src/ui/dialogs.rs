use eframe::egui;
use crate::actions::ConfirmationAction;

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

/// Render a generic confirmation dialog
pub fn render_confirmation_dialog(
    ctx: &egui::Context,
    show_dialog: &mut bool,
    title: &str,
    message: &str,
) -> ConfirmationAction {
    let mut action = ConfirmationAction::None;

    if *show_dialog {
        egui::Window::new(title)
            .open(show_dialog)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(message);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        action = ConfirmationAction::Save;
                    }
                    if ui.button("Don't Save").clicked() {
                        action = ConfirmationAction::DontSave;
                    }
                    if ui.button("Cancel").clicked() {
                        action = ConfirmationAction::Cancel;
                    }
                });
            });
    }

    action
}

/// Renders a confirmation dialog for creating a new file when there are unsaved changes
pub fn render_new_dialog(ctx: &egui::Context, show: &mut bool) -> ConfirmationAction {
    let mut action = ConfirmationAction::None;
    
    if *show {
        egui::Window::new("Unsaved Changes")
            .open(show)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Do you want to save changes before creating a new file?");
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        action = ConfirmationAction::Save;
                    }
                    if ui.button("Don't Save").clicked() {
                        action = ConfirmationAction::DontSave;
                    }
                    if ui.button("Cancel").clicked() {
                        action = ConfirmationAction::Cancel;
                    }
                });
            });
    }
    
    action
}

/// Render the quit confirmation dialog
pub fn render_quit_dialog(
    ctx: &egui::Context,
    show_quit_dialog: &mut bool,
) -> ConfirmationAction {
    render_confirmation_dialog(
        ctx,
        show_quit_dialog,
        "Unsaved Changes",
        "You have unsaved changes. Do you want to save before quitting?",
    )
}

/// Render the open file confirmation dialog
pub fn render_open_dialog(
    ctx: &egui::Context,
    show_open_dialog: &mut bool,
) -> ConfirmationAction {
    render_confirmation_dialog(
        ctx,
        show_open_dialog,
        "Unsaved Changes",
        "You have unsaved changes. Do you want to save before opening a new file?",
    )
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

/// Render the Go to Line dialog
pub fn render_goto_line_dialog(
    ctx: &egui::Context,
    show_dialog: &mut bool,
    line_input: &mut String,
) -> Option<usize> {
    let mut target_line = None;
    let mut close_requested = false;

    if *show_dialog {
        egui::Window::new("Go to Line")
            .open(show_dialog)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Line number:");
                    let response = ui.text_edit_singleline(line_input);
                    
                    // Focus the input field when dialog opens
                    response.request_focus();
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Ok(line) = line_input.trim().parse::<usize>() {
                            target_line = Some(line);
                            close_requested = true;
                        }
                    }
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Go").clicked() {
                        if let Ok(line) = line_input.trim().parse::<usize>() {
                            target_line = Some(line);
                            close_requested = true;
                        }
                    }
                    
                    if ui.button("Cancel").clicked() {
                        close_requested = true;
                    }
                });
            });
            
        if close_requested {
            *show_dialog = false;
        }
    }

    target_line
}
