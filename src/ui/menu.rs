use eframe::egui;

/// Render the top menu bar with File, Edit, View, Search, and Help menus
pub fn render_menu(
    ui: &mut egui::Ui,
    show_about_window: &mut bool,
    can_undo: bool,
    can_redo: bool,
    show_line_numbers: bool,
    syntax_highlighting: bool,
) -> MenuAction {
    let mut action = MenuAction::None;
    
    // Adds a menu button named "File"
    ui.menu_button("File", |ui| {
        if ui.button("New").on_hover_text("Ctrl+N").clicked() {
            action = MenuAction::New;
        }
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
        ui.add_enabled_ui(can_undo, |ui| {
            if ui.button("Undo").on_hover_text("Ctrl+Z").clicked() {
                action = MenuAction::Undo;
            }
        });
        ui.add_enabled_ui(can_redo, |ui| {
            if ui.button("Redo").on_hover_text("Ctrl+Y").clicked() {
                action = MenuAction::Redo;
            }
        });
    });
    
    // Add menu button named "View"
    ui.menu_button("View", |ui| {
        if ui.checkbox(&mut show_line_numbers.clone(), "Line Numbers").clicked() {
            action = MenuAction::ToggleLineNumbers;
        }
        if ui.checkbox(&mut syntax_highlighting.clone(), "Syntax Highlighting").clicked() {
            action = MenuAction::ToggleSyntaxHighlighting;
        }
    });
    
    // Add menu button named "Search"
    ui.menu_button("Search", |ui| {
        if ui.button("Find").on_hover_text("Cmd+F").clicked() {
            action = MenuAction::Find;
        }
        if ui.button("Go to Line").on_hover_text("Cmd+G").clicked() {
            action = MenuAction::GoToLine;
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
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Find,
    Undo,
    Redo,
    ToggleLineNumbers,
    ToggleSyntaxHighlighting,
    GoToLine,
}
