use eframe::egui;
use eframe::egui::Align;

enum QuitAction {
    None,
    Save,
    DontSave,
    Cancel,
}

enum OpenAction {
    None,
    Save,
    DontSave,
    Cancel,
}


#[derive(Default)]
struct MyApp {
    text: String,
    show_about_window: bool,
    filename: Option<String>,
    file_path: Option<std::path::PathBuf>,
    is_dirty: bool,
    last_saved_text: String,
    show_quit_dialog: bool,
    show_open_dialog: bool,
    pending_open_path: Option<std::path::PathBuf>,
}

impl MyApp {
    /// Open a file and load its contents into the editor
    fn open_file(&mut self, path: std::path::PathBuf) -> Result<(), std::io::Error> {
        let contents = std::fs::read_to_string(&path)?;
        self.text = contents.clone();
        self.last_saved_text = contents;
        self.filename = path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());
        self.file_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }
    
    /// Save the current text to the existing file path
    fn save_file(&mut self) -> Result<(), std::io::Error> {
        if let Some(path) = &self.file_path {
            std::fs::write(path, &self.text)?;
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
    fn save_file_as(&mut self, path: std::path::PathBuf) -> Result<(), std::io::Error> {
        std::fs::write(&path, &self.text)?;
        self.last_saved_text = self.text.clone();
        self.filename = path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());
        self.file_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle window close button (X)
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.is_dirty {
                // Prevent immediate close and show dialog
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_quit_dialog = true;
            }
            // If not dirty, allow the close to proceed naturally
        }
        
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
        });
        
        // Execute keyboard shortcut actions
        if open_file {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                if self.is_dirty {
                    // Show confirmation dialog
                    self.pending_open_path = Some(path);
                    self.show_open_dialog = true;
                } else {
                    // No unsaved changes, open directly
                    if let Err(e) = self.open_file(path) {
                        eprintln!("Failed to open file: {}", e);
                    }
                }
            }
        }
        
        if save_file {
            if let Err(_) = self.save_file() {
                // No file path, prompt for Save As
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    if let Err(e) = self.save_file_as(path) {
                        eprintln!("Failed to save file: {}", e);
                    }
                }
            }
        }
        
        if quit_app {
            if self.is_dirty {
                self.show_quit_dialog = true;
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        
         // Menubar at the top
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Creates the horizontal menu bar
            egui::MenuBar::new().ui(ui, |ui| {
                // Adds a menu button named "File"
                ui.menu_button("File", |ui| {
                    if ui.button("Open").on_hover_text("Cmd+O").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if self.is_dirty {
                                // Show confirmation dialog
                                self.pending_open_path = Some(path);
                                self.show_open_dialog = true;
                            } else {
                                // No unsaved changes, open directly
                                if let Err(e) = self.open_file(path) {
                                    eprintln!("Failed to open file: {}", e);
                                }
                            }
                        }
                    }
                    if ui.button("Save").on_hover_text("Cmd+S").clicked() {
                        if let Err(_) = self.save_file() {
                            // No file path, prompt for Save As
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                if let Err(e) = self.save_file_as(path) {
                                    eprintln!("Failed to save file: {}", e);
                                }
                            }
                        }
                    }
                    if ui.button("Save As").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(e) = self.save_file_as(path) {
                                eprintln!("Failed to save file: {}", e);
                            }
                        }
                    }
                    ui.separator();
                    // Adds a button in the dropdown menu
                    if ui.button("Quit").on_hover_text("Cmd+Q").clicked() {
                        // Check if there are unsaved changes
                        if self.is_dirty {
                            self.show_quit_dialog = true;
                        } else {
                            // No unsaved changes, quit immediately
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                // Add menu button named "Edit"
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        //To do: Implement undo functionality
                    }
                });
                // Add menu button named "Search"
                ui.menu_button("Search", |ui| {
                    if ui.button("Find").clicked() {
                        //To do: Implement find functionality
                    }
                });
                // Add menu button named "Help"
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about_window = true;
                    }
                });
            });
        });
        // status bar at the bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let display_name = self.filename.as_deref().unwrap_or("untitled");
                let dirty_indicator = if self.is_dirty { "*" } else { "" };
                ui.label(format!("{}{}", display_name, dirty_indicator));
                ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui| {
                    ui.label("Ready");
                });
            });
        });

        // central area: text edit filling the remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text).frame(true));
            
            // Check if text has been modified
            if self.text != self.last_saved_text {
                self.is_dirty = true;
            }
        });
        // About window
        if self.show_about_window {
            let mut close_requested = false;
            egui::Window::new("About the App")
                // Link the open state to the show_about_window field
                .open(&mut self.show_about_window)
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
                self.show_about_window = false;
            }
        }
        
        // Quit confirmation dialog
        if self.show_quit_dialog {
            let mut action = QuitAction::None;
            egui::Window::new("Unsaved Changes")
                .open(&mut self.show_quit_dialog)
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
            
            match action {
                QuitAction::Save => {
                    // Save the file
                    if let Err(_) = self.save_file() {
                        // No file path, prompt for Save As
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(e) = self.save_file_as(path) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    } else {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    self.show_quit_dialog = false;
                }
                QuitAction::DontSave => {
                    self.show_quit_dialog = false;
                    self.is_dirty = false; // Clear dirty flag so close can proceed
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                QuitAction::Cancel => {
                    self.show_quit_dialog = false;
                }
                QuitAction::None => {}
            }
        }
        
        // Open file confirmation dialog
        if self.show_open_dialog {
            let mut action = OpenAction::None;
            egui::Window::new("Unsaved Changes")
                .open(&mut self.show_open_dialog)
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
            
            match action {
                OpenAction::Save => {
                    // Save the current file first
                    if let Err(_) = self.save_file() {
                        // No file path, prompt for Save As
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(e) = self.save_file_as(path) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                // Now open the pending file
                                if let Some(pending_path) = self.pending_open_path.take() {
                                    if let Err(e) = self.open_file(pending_path) {
                                        eprintln!("Failed to open file: {}", e);
                                    }
                                }
                            }
                        }
                    } else {
                        // Save succeeded, now open the pending file
                        if let Some(pending_path) = self.pending_open_path.take() {
                            if let Err(e) = self.open_file(pending_path) {
                                eprintln!("Failed to open file: {}", e);
                            }
                        }
                    }
                    self.show_open_dialog = false;
                }
                OpenAction::DontSave => {
                    // Discard changes and open the pending file
                    if let Some(pending_path) = self.pending_open_path.take() {
                        if let Err(e) = self.open_file(pending_path) {
                            eprintln!("Failed to open file: {}", e);
                        }
                    }
                    self.show_open_dialog = false;
                }
                OpenAction::Cancel => {
                    // Cancel the open operation
                    self.pending_open_path = None;
                    self.show_open_dialog = false;
                }
                OpenAction::None => {}
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Windows Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}