use egui_code_editor::Syntax;

/// Determines the appropriate syntax highlighting based on file extension
pub fn get_syntax_for_file(filename: Option<&String>, highlighting_enabled: bool) -> Syntax {
    if !highlighting_enabled {
        return Syntax::default();
    }
    
    let Some(filename) = filename else {
        return Syntax::default();
    };
    
    match get_file_extension(filename) {
        Some("rs") => Syntax::rust(),
        Some("py") | Some("py3") | Some("pyw") => Syntax::python(),
        Some("lua") => Syntax::lua(),
        Some("sh") | Some("bash") | Some("zsh") => Syntax::shell(),
        Some("sql") => Syntax::sql(),
        Some("asm") | Some("s") => Syntax::asm(),
        Some("txt") | _ => Syntax::default(),
    }
}

/// Extracts the file extension from a filename
fn get_file_extension(filename: &str) -> Option<&str> {
    filename.rsplit('.').next()
}

/// Returns a human-readable name for the detected language
pub fn get_language_name(filename: Option<&String>) -> &'static str {
    let Some(filename) = filename else {
        return "Plain Text";
    };
    
    match get_file_extension(filename) {
        Some("rs") => "Rust",
        Some("py") | Some("py3") | Some("pyw") => "Python",
        Some("lua") => "Lua",
        Some("sh") | Some("bash") | Some("zsh") => "Shell",
        Some("sql") => "SQL",
        Some("asm") | Some("s") => "Assembly",
        _ => "Plain Text",
    }
}

