use eframe::egui;

/// Render the top menu bar with File, Edit, Search, and Help menus
pub fn render_menu(
    ui: &mut egui::Ui,
    show_about_window: &mut bool,
) -> MenuAction {
    let mut action = MenuAction::None;
    
    // Adds a menu button named "File"
    ui.menu_button("File", |ui| {
        if ui.button("Open").on_hover_text("Cmd+O").clicked() {
            action = MenuAction::Open;
        }
        if ui.button("Save").on_hover_text("Cmd+S").clicked() {
            action = MenuAction::Save;
        }
        if ui.button("Save As").clicked() {
            action = MenuAction::SaveAs;
        }
        ui.separator();
        if ui.button("Quit").on_hover_text("Cmd+Q").clicked() {
            action = MenuAction::Quit;
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
            *show_about_window = true;
        }
    });
    
    action
}

/// Actions that can be triggered from the menu
pub enum MenuAction {
    None,
    Open,
    Save,
    SaveAs,
    Quit,
}
