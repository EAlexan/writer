use eframe::egui;
use crate::actions::ConfirmationAction;
use crate::file_ops;
use crate::ui::{menu, status_bar, dialogs};

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
    pub show_find_bar: bool,
    pub find_query: String,
    pub find_results: Vec<usize>,
    pub current_match_index: Option<usize>,
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

    /// Update search results based on current query
    fn update_search_results(&mut self) {
        self.find_results.clear();
        self.current_match_index = None;
        
        if self.find_query.is_empty() {
            return;
        }
        
        self.find_results = self.text.match_indices(&self.find_query).map(|(i, _)| i).collect();
        
        if !self.find_results.is_empty() {
            self.current_match_index = Some(0);
        }
    }
    
    /// Move to the next search result
    fn find_next(&mut self) {
        if self.find_results.is_empty() {
            return;
        }
        
        if let Some(current) = self.current_match_index {
            self.current_match_index = Some((current + 1) % self.find_results.len());
        } else {
            self.current_match_index = Some(0);
        }
    }
    
    /// Move to the previous search result
    fn find_previous(&mut self) {
        if self.find_results.is_empty() {
            return;
        }
        
        if let Some(current) = self.current_match_index {
            if current == 0 {
                self.current_match_index = Some(self.find_results.len() - 1);
            } else {
                self.current_match_index = Some(current - 1);
            }
        } else {
            self.current_match_index = Some(self.find_results.len() - 1);
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
                self.show_find_bar = !self.show_find_bar;
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
                    menu::MenuAction::Find => self.show_find_bar = !self.show_find_bar,
                    menu::MenuAction::None => {}
                }
            });
        });

        // Find Bar
        if self.show_find_bar {
            egui::TopBottomPanel::top("find_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    let response = ui.text_edit_singleline(&mut self.find_query);
                    if response.changed() {
                        self.update_search_results();
                    }
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.find_next();
                    }
                    
                    if ui.button("Next").clicked() {
                        self.find_next();
                    }
                    
                    if ui.button("Previous").clicked() {
                        self.find_previous();
                    }
                    
                    if let Some(index) = self.current_match_index {
                        ui.label(format!("Match {} of {}", index + 1, self.find_results.len()));
                    } else if !self.find_query.is_empty() && self.find_results.is_empty() {
                        ui.label("No matches found");
                    }
                    
                    if ui.button("Close").clicked() {
                        self.show_find_bar = false;
                        self.find_query.clear();
                        self.find_results.clear();
                        self.current_match_index = None;
                    }
                });
            });
        }
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            status_bar::render_status_bar(ui, &self.filename, self.is_dirty);
        });

        // Central area: text edit filling the remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            let find_results = self.find_results.clone();
            let match_len = self.find_query.len();
            let current_match_index = self.current_match_index;

            let mut layouter = move |ui: &egui::Ui, string: &dyn egui::TextBuffer, wrap_width: f32| {
                let string = string.as_str();
                let mut layout_job = egui::text::LayoutJob::default();
                
                if find_results.is_empty() || match_len == 0 {
                    layout_job.append(
                        string,
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::monospace(14.0),
                            ..Default::default()
                        },
                    );
                } else {
                    let mut last_index = 0;
                    for (i, &index) in find_results.iter().enumerate() {
                        if index > last_index {
                            layout_job.append(
                                &string[last_index..index],
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::monospace(14.0),
                                    ..Default::default()
                                },
                            );
                        }
                        
                        let bg_color = if Some(i) == current_match_index {
                            egui::Color32::from_rgb(255, 165, 0) // Orange for current
                        } else {
                            egui::Color32::YELLOW // Yellow for others
                        };
                        
                        layout_job.append(
                            &string[index..index + match_len],
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::monospace(14.0),
                                background: bg_color,
                                color: egui::Color32::BLACK,
                                ..Default::default()
                            },
                        );
                        
                        last_index = index + match_len;
                    }
                    
                    if last_index < string.len() {
                        layout_job.append(
                            &string[last_index..],
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::monospace(14.0),
                                ..Default::default()
                            },
                        );
                    }
                }
                
                layout_job.wrap.max_width = wrap_width;
                ui.painter().layout_job(layout_job)
            };

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
