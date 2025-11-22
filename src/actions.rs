/// Actions that can be taken in response to the quit confirmation dialog
pub enum QuitAction {
    None,
    Save,
    DontSave,
    Cancel,
}

/// Actions that can be taken in response to the open file confirmation dialog
pub enum OpenAction {
    None,
    Save,
    DontSave,
    Cancel,
}
