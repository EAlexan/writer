use eframe::egui;
use crate::app::MyApp;

pub struct Shortcuts {
    pub undo: bool,
    pub redo: bool,
    pub toggle_find: bool,
    pub new_file: bool,
    pub open_file: bool,
    pub save_file: bool,
    pub quit_app: bool,
    pub go_to_line: bool,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            undo: false,
            redo: false,
            toggle_find: false,
            new_file: false,
            open_file: false,
            save_file: false,
            quit_app: false,
            go_to_line: false,
        }
    }
}

pub fn handle_shortcuts(ctx: &egui::Context, app: &mut MyApp) {
    let mut shortcuts = Shortcuts::default();

    ctx.input(|i| {
        // Ctrl+N - New file
        if i.modifiers.command && i.key_pressed(egui::Key::N) {
            shortcuts.new_file = true;
        }
        
        // Ctrl+O - Open file
        if i.modifiers.command && i.key_pressed(egui::Key::O) {
            shortcuts.open_file = true;
        }
        
        // Ctrl+S - Save file
        if i.modifiers.command && i.key_pressed(egui::Key::S) {
            shortcuts.save_file = true;
        }
        
        // Ctrl+Q - Quit
        if i.modifiers.command && i.key_pressed(egui::Key::Q) {
            shortcuts.quit_app = true;
        }

        // Ctrl+F - Find
        if i.modifiers.command && i.key_pressed(egui::Key::F) {
            shortcuts.toggle_find = true;
        }
        
        // Ctrl+Z - Undo
        if i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
            shortcuts.undo = true;
        }
        
        // Ctrl+Y or Ctrl+Shift+Z - Redo
        if i.modifiers.command && (i.key_pressed(egui::Key::Y) || (i.modifiers.shift && i.key_pressed(egui::Key::Z))) {
            shortcuts.redo = true;
        }

        // Ctrl+G - Go to Line
        if i.modifiers.command && i.key_pressed(egui::Key::G) {
            shortcuts.go_to_line = true;
        }
    });
    
    // Execute actions
    // Execute actions
    if shortcuts.undo {
        app.save_undo_state();
        app.handle_redo();
    }
    
    if shortcuts.redo {
        app.save_undo_state();
        app.handle_undo();
    }
    
    if shortcuts.toggle_find {
        app.search.show_bar = !app.search.show_bar;
    }
    
    if shortcuts.new_file {
        app.handle_new_action();
    }
    
    if shortcuts.open_file {
        app.handle_open_action();
    }
    
    if shortcuts.save_file {
        app.handle_save_action();
    }
    
    if shortcuts.quit_app {
        app.handle_quit_action(ctx);
    }

    if shortcuts.go_to_line {
        app.show_goto_line_dialog = true;
        app.goto_line_input.clear();
    }
}
