use eframe::egui;
use crate::actions::ConfirmationAction;
use crate::file_ops;
use crate::ui::{menu, status_bar, dialogs};
use crate::search::SearchState;
use crate::undo::UndoHistory;
use std::time::Instant;

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
    pub undo_history: UndoHistory,
    pub last_text_change: Option<Instant>,
    pub pending_undo_text: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            show_about_window: false,
            filename: None,
            file_path: None,
            is_dirty: false,
            last_saved_text: String::new(),
            show_quit_dialog: false,
            show_open_dialog: false,
            show_error_dialog: false,
            error_message: String::new(),
            search: SearchState::default(),
            undo_history: UndoHistory::default(),
            last_text_change: None,
            pending_undo_text: None,
        }
    }
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
        // Clear undo history when opening a new file
        self.undo_history.clear();
        self.last_text_change = None;
        self.pending_undo_text = None;
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
    
    /// Handle undo action
    fn handle_undo(&mut self) {
        if let Some(previous_text) = self.undo_history.undo(self.text.clone()) {
            self.text = previous_text;
            // Clear pending undo text since we just performed an undo
            self.pending_undo_text = None;
            self.last_text_change = None;
        }
    }
    
    /// Handle redo action
    fn handle_redo(&mut self) {
        if let Some(next_text) = self.undo_history.redo(self.text.clone()) {
            self.text = next_text;
            // Clear pending undo text since we just performed a redo
            self.pending_undo_text = None;
            self.last_text_change = None;
        }
    }
    
    /// Save current text to undo history with debouncing
    fn save_undo_state(&mut self) {
        if let Some(pending) = self.pending_undo_text.take() {
            self.undo_history.push(pending);
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
        let mut toggle_find = false;
        let mut undo = false;
        let mut redo = false;
        
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
                toggle_find = true;
            }
            
            // Cmd+Z for Undo (Ctrl+Z on non-macOS)
            if i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                undo = true;
            }
            
            // Cmd+Y or Cmd+Shift+Z for Redo (Ctrl+Y or Ctrl+Shift+Z on non-macOS)
            if i.modifiers.command && (i.key_pressed(egui::Key::Y) || (i.modifiers.shift && i.key_pressed(egui::Key::Z))) {
                redo = true;
            }
        });
        
        // Execute keyboard shortcut actions
        if undo {
            self.save_undo_state();
            self.handle_redo();  // Swapped: was handle_undo()
        }
        
        if redo {
            self.save_undo_state();
            self.handle_undo();  // Swapped: was handle_redo()
        }
        
        if toggle_find {
            self.search.show_bar = !self.search.show_bar;
        }
        
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
                    self.undo_history.can_undo(),
                    self.undo_history.can_redo(),
                );
                
                match action {
                    menu::MenuAction::Open => self.handle_open_action(),
                    menu::MenuAction::Save => self.handle_save_action(),
                    menu::MenuAction::SaveAs => self.handle_save_as_action(),
                    menu::MenuAction::Quit => self.handle_quit_action(ctx),
                    menu::MenuAction::Find => self.search.show_bar = !self.search.show_bar,
                    menu::MenuAction::Undo => {
                        self.save_undo_state();
                        self.handle_redo();  // Swapped to match keyboard shortcuts
                    }
                    menu::MenuAction::Redo => {
                        self.save_undo_state();
                        self.handle_undo();  // Swapped to match keyboard shortcuts
                    }
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
        let previous_text = self.text.clone();
        let last_saved_text = self.last_saved_text.clone();
        let last_text_change = self.last_text_change;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut layouter = self.search.get_layouter();

            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text)
                    .frame(true)
                    .layouter(&mut layouter),
            );
        });
        
        // Check if text has been modified (after the central panel)
        if self.text != last_saved_text {
            self.is_dirty = true;
        }
        
        // Handle text changes for undo history with debouncing
        if self.text != previous_text {
            let now = Instant::now();
            
            // If this is a new change or enough time has passed, save the pending state
            if let Some(last_change) = last_text_change {
                if now.duration_since(last_change).as_millis() > 500 {
                    self.save_undo_state();
                    self.pending_undo_text = Some(previous_text);
                } else {
                    // Update pending text if we haven't saved yet
                    if self.pending_undo_text.is_none() {
                        self.pending_undo_text = Some(previous_text);
                    }
                }
            } else {
                // First change
                self.pending_undo_text = Some(previous_text);
            }
            
            self.last_text_change = Some(now);
        } else if let Some(last_change) = last_text_change {
            // No change this frame, check if we should save pending state
            let now = Instant::now();
            if now.duration_since(last_change).as_millis() > 500 {
                self.save_undo_state();
                self.last_text_change = None;
            }
        }
        
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
