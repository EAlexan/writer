use writer::syntax::get_language_name;

#[test]
fn test_rust_extension() {
    let filename = "main.rs".to_string();
    assert_eq!(get_language_name(Some(&filename)), "Rust");
}

#[test]
fn test_python_extensions() {
    let py = "script.py".to_string();
    let py3 = "script.py3".to_string();
    let pyw = "script.pyw".to_string();
    assert_eq!(get_language_name(Some(&py)), "Python");
    assert_eq!(get_language_name(Some(&py3)), "Python");
    assert_eq!(get_language_name(Some(&pyw)), "Python");
}

#[test]
fn test_shell_extensions() {
    let sh = "script.sh".to_string();
    let bash = "script.bash".to_string();
    let zsh = "script.zsh".to_string();
    assert_eq!(get_language_name(Some(&sh)), "Shell");
    assert_eq!(get_language_name(Some(&bash)), "Shell");
    assert_eq!(get_language_name(Some(&zsh)), "Shell");
}

#[test]
fn test_no_extension() {
    let readme = "README".to_string();
    assert_eq!(get_language_name(Some(&readme)), "Plain Text");
}

#[test]
fn test_no_filename() {
    assert_eq!(get_language_name(None), "Plain Text");
}

#[test]
fn test_unknown_extension() {
    let xyz = "file.xyz".to_string();
    assert_eq!(get_language_name(Some(&xyz)), "Plain Text");
}
