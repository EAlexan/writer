use std::io;
use std::path::PathBuf;

/// Open a file and return its contents along with metadata
pub fn open_file(path: PathBuf) -> io::Result<(String, String, PathBuf)> {
    let contents = std::fs::read_to_string(&path)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "untitled".to_string());
    
    Ok((contents.clone(), filename, path))
}

/// Save text to an existing file path
pub fn save_file(path: &PathBuf, text: &str) -> io::Result<()> {
    std::fs::write(path, text)
}

/// Save text to a new file path and return the filename
pub fn save_file_as(path: PathBuf, text: &str) -> io::Result<(String, PathBuf)> {
    std::fs::write(&path, text)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "untitled".to_string());
    
    Ok((filename, path))
}
