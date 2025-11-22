use eframe::egui;
use eframe::egui::Align;

#[derive(Default)]
struct MyApp {
    text: String,
    show_about_window: bool,
    filename: Option<String>,
    file_path: Option<std::path::PathBuf>,
    is_dirty: bool,
    last_saved_text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
         // Menubar at the top
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Creates the horizontal menu bar
            egui::MenuBar::new().ui(ui, |ui| {
                // Adds a menu button named "File"
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(contents) = std::fs::read_to_string(&path) {
                                self.text = contents.clone();
                                self.last_saved_text = contents;
                                self.filename = path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|s| s.to_string());
                                self.file_path = Some(path);
                                self.is_dirty = false;
                            }
                        }
                    }
                    if ui.button("Save").clicked()
                    {
                        // If we have a file path, save to it; otherwise, prompt for a new file
                        if let Some(path) = &self.file_path {
                            if let Err(e) = std::fs::write(path, &self.text) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                self.last_saved_text = self.text.clone();
                                self.is_dirty = false;
                            }
                        } else {
                            // No file path, prompt for Save As
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                if let Err(e) = std::fs::write(&path, &self.text) {
                                    eprintln!("Failed to save file: {}", e);
                                } else {
                                    self.last_saved_text = self.text.clone();
                                    self.filename = path.file_name()
                                        .and_then(|n| n.to_str())
                                        .map(|s| s.to_string());
                                    self.file_path = Some(path);
                                    self.is_dirty = false;
                                }
                            }
                        }
                    }
                    if ui.button("Save As").clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(e) = std::fs::write(&path, &self.text) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                self.last_saved_text = self.text.clone();
                                self.filename = path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|s| s.to_string());
                                self.file_path = Some(path);
                                self.is_dirty = false;
                            }
                        }
                    }
                    ui.separator();
                    // Adds a button in the dropdown menu
                    if ui.button("Quit").clicked() {
                        // Command to close the application
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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