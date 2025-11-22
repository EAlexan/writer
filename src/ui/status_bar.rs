use eframe::egui;
use eframe::egui::Align;

/// Render the bottom status bar showing filename and status
pub fn render_status_bar(
    ui: &mut egui::Ui,
    filename: &Option<String>,
    is_dirty: bool,
) {
    ui.horizontal(|ui| {
        let display_name = filename.as_deref().unwrap_or("untitled");
        let dirty_indicator = if is_dirty { "*" } else { "" };
        ui.label(format!("{}{}", display_name, dirty_indicator));
        ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui| {
            ui.label("Ready");
        });
    });
}
