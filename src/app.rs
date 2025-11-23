use eframe::egui;
use crate::actions::ConfirmationAction;
use crate::file_ops;
use crate::ui::{menu, status_bar, dialogs};
use crate::search::SearchState;

#[derive(Default)]
pub struct MyApp {
    pub text: String,
    pub show_about_window: bool,
    pub filename: Option<String>,
    pub file_path: Option<std::path::PathBuf>,
    pub is_dirty: bool,
    pub last_saved_text: String,
    pub show_quit_dialog: bool,
    pub show_open_dialog: bool,
    pub show_error_dialog: bool,
    pub error_message: String,
    pub search: SearchState,
}

impl MyApp {
    /// Open a file and load its contents into the editor
    pub fn open_file(&mut self, path: std::path::PathBuf) -> Result<(), std::io::Error> {
        let (contents, filename, path) = file_ops::open_file(path)?;
        self.text = contents.clone();
        self.last_saved_text = contents;
        self.filename = Some(filename);
        self.file_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }
    
    /// Save the current text to the existing file path
    pub fn save_file(&mut self) -> Result<(), std::io::Error> {
        if let Some(path) = &self.file_path {
            file_ops::save_file(path, &self.text)?;
            self.last_saved_text = self.text.clone();
            self.is_dirty = false;
            Ok(())
        } else {
            // No file path exists, need to use save_as
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path set"
            ))
        }
    }
    
    /// Save the current text to a new file path
    pub fn save_file_as(&mut self, path: std::path::PathBuf) -> Result<(), std::io::Error> {
        let (filename, path) = file_ops::save_file_as(path, &self.text)?;
        self.last_saved_text = self.text.clone();
        self.filename = Some(filename);
        self.file_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }
    
    /// Show an error message to the user in a dialog
    pub fn show_error(&mut self, message: String) {
        self.error_message = message;
        self.show_error_dialog = true;
    }
    
    /// Handle file open action with proper save confirmation
    fn handle_open_action(&mut self) {
        if self.is_dirty {
            self.show_open_dialog = true;
        } else if let Some(path) = rfd::FileDialog::new().pick_file()
        && let Err(e) = self.open_file(path) {
            self.show_error(format!("Failed to open file: {}", e));
        }
    }
    
    /// Handle file save action with save-as fallback
    fn handle_save_action(&mut self) {
        if self.save_file().is_err() {
            // No file path, prompt for Save As
            if let Some(path) = rfd::FileDialog::new().save_file()
                && let Err(e) = self.save_file_as(path) {
                    self.show_error(format!("Failed to save file: {}", e));
                }
        }
    }
    
    /// Handle save-as action
    fn handle_save_as_action(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file()
            && let Err(e) = self.save_file_as(path) {
                self.show_error(format!("Failed to save file: {}", e));
            }
    }
    
    /// Handle quit action with proper save confirmation
    fn handle_quit_action(&mut self, ctx: &egui::Context) {
        if self.is_dirty {
            self.show_quit_dialog = true;
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle window close button (X)
        if ctx.input(|i| i.viewport().close_requested())
            && self.is_dirty {
                // Prevent immediate close and show dialog
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_quit_dialog = true;
            }
            // If not dirty, allow the close to proceed naturally
        
        // Handle keyboard shortcuts
        let mut open_file = false;
        let mut save_file = false;
        let mut quit_app = false;
        
        ctx.input(|i| {
            // Cmd+O for Open (Ctrl+O on non-macOS)
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                open_file = true;
            }
            
            // Cmd+S for Save (Ctrl+S on non-macOS)
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                save_file = true;
            }
            
            // Cmd+Q for Quit (Ctrl+Q on non-macOS)
            if i.modifiers.command && i.key_pressed(egui::Key::Q) {
                quit_app = true;
            }

            // Cmd+F for Find (Ctrl+F on non-macOS)
            if i.modifiers.command && i.key_pressed(egui::Key::F) {
                self.search.show_bar = !self.search.show_bar;
            }
        });
        
        // Execute keyboard shortcut actions
        if open_file {
            self.handle_open_action();
        }
        
        if save_file {
            self.handle_save_action();
        }
        
        if quit_app {
            self.handle_quit_action(ctx);
        }
        
        // Menubar at the top
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let action = menu::render_menu(
                    ui,
                    &mut self.show_about_window,
                );
                
                match action {
                    menu::MenuAction::Open => self.handle_open_action(),
                    menu::MenuAction::Save => self.handle_save_action(),
                    menu::MenuAction::SaveAs => self.handle_save_as_action(),
                    menu::MenuAction::Quit => self.handle_quit_action(ctx),
                    menu::MenuAction::Find => self.search.show_bar = !self.search.show_bar,
                    menu::MenuAction::None => {}
                }
            });
        });

        // Find Bar
        if self.search.show_bar {
            egui::TopBottomPanel::top("find_panel").show(ctx, |ui| {
                self.search.render_bar(ui, &self.text);
            });
        }

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            status_bar::render_status_bar(ui, &self.filename, self.is_dirty);
        });

        // Central area: text edit filling the remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut layouter = self.search.get_layouter();

            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text)
                    .frame(true)
                    .layouter(&mut layouter),
            );
            
            // Check if text has been modified
            if self.text != self.last_saved_text {
                self.is_dirty = true;
            }
        });
        
        // Render all dialogs
        dialogs::render_about_dialog(ctx, &mut self.show_about_window);
        
        // Quit confirmation dialog
        let quit_action = dialogs::render_quit_dialog(ctx, &mut self.show_quit_dialog);
        match quit_action {
            ConfirmationAction::Save => {
                if self.save_file().is_err() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        if let Err(e) = self.save_file_as(path) {
                            self.show_error(format!("Failed to save file: {}", e));
                        } else {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                self.show_quit_dialog = false;
            }
            ConfirmationAction::DontSave => {
                self.show_quit_dialog = false;
                self.is_dirty = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ConfirmationAction::Cancel => {
                self.show_quit_dialog = false;
            }
            ConfirmationAction::None => {}
        }
        
        // Open file confirmation dialog
        let open_action = dialogs::render_open_dialog(ctx, &mut self.show_open_dialog);
        match open_action {
            ConfirmationAction::Save => {
                if self.save_file().is_err() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        if let Err(e) = self.save_file_as(path) {
                            self.show_error(format!("Failed to save file: {}", e));
                        } else if let Some(path) = rfd::FileDialog::new().pick_file()
                        && let Err(e) = self.open_file(path) {
                            self.show_error(format!("Failed to open file: {}", e));
                        }
                    }
                } else if let Some(path) = rfd::FileDialog::new().pick_file()
                && let Err(e) = self.open_file(path) {
                    self.show_error(format!("Failed to open file: {}", e));
                }
                self.show_open_dialog = false;
            }
            ConfirmationAction::DontSave => {
                if let Some(path) = rfd::FileDialog::new().pick_file()
                    && let Err(e) = self.open_file(path) {
                        self.show_error(format!("Failed to open file: {}", e));
                    }
                self.show_open_dialog = false;
            }
            ConfirmationAction::Cancel => {
                self.show_open_dialog = false;
            }
            ConfirmationAction::None => {}
        }
        
        // Error dialog
        dialogs::render_error_dialog(ctx, &mut self.show_error_dialog, &self.error_message);
    }
}
