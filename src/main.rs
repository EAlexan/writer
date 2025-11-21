use eframe::egui;
use eframe::egui::Align;

#[derive(Default)]
struct MyApp {
    text: String,
    show_about_window: bool,
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
                            if let Ok(contents) = std::fs::read_to_string(path) {
                                self.text = contents;
                            }
                        }
                    }
                    if ui.button("Save").clicked()
                    {
                        //To do: Implement save functionality
                    }
                    if ui.button("Save As").clicked()
                    {
                        //To do: Implement save as functionality
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
                ui.label(format!("{} characters", self.text.len()));
                ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui| {
                    ui.label("Ready");
                });
            });
        });

        // central area: text edit filling the remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text).frame(true));
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