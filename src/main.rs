use eframe::egui;

#[derive(Default)]
struct MyApp {

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top bar with toggles
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {});
        });


        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui windows example");
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