use eframe::egui;
use eframe::egui::Align;

#[derive(Default)]
struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Windows Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}