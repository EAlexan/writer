use writer::undo::UndoHistory;

#[test]
fn test_new_history() {
    let history = UndoHistory::new(10);
    assert!(!history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_push_and_undo() {
    let mut history = UndoHistory::new(10);
    
    // Push initial state
    history.push("state1".to_string());
    assert!(history.can_undo());
    assert!(!history.can_redo());

    // Undo should return the previous state
    let result = history.undo("state2".to_string());
    assert_eq!(result, Some("state1".to_string()));
    assert!(!history.can_undo());
    assert!(history.can_redo());
}

#[test]
fn test_undo_redo_sequence() {
    let mut history = UndoHistory::new(10);
    
    history.push("state1".to_string());
    history.push("state2".to_string());
    
    // Undo to state2
    let result = history.undo("state3".to_string());
    assert_eq!(result, Some("state2".to_string()));
    
    // Undo to state1
    let result = history.undo("state2".to_string());
    assert_eq!(result, Some("state1".to_string()));
    
    // Redo to state2
    let result = history.redo("state1".to_string());
    assert_eq!(result, Some("state2".to_string()));
    
    // Redo to state3
    let result = history.redo("state2".to_string());
    assert_eq!(result, Some("state3".to_string()));
}

#[test]
fn test_push_clears_redo() {
    let mut history = UndoHistory::new(10);
    
    history.push("state1".to_string());
    history.push("state2".to_string());
    history.undo("state3".to_string());
    
    assert!(history.can_redo());
    
    // Making a new change should clear redo stack
    history.push("state4".to_string());
    assert!(!history.can_redo());
}

#[test]
fn test_capacity_limit() {
    let mut history = UndoHistory::new(3);
    
    history.push("state1".to_string());
    history.push("state2".to_string());
    history.push("state3".to_string());
    history.push("state4".to_string()); // Should remove state1
    
    // Stack now has: [state2, state3, state4]
    // Undo with current state5 should give us state4
    let result = history.undo("state5".to_string());
    assert_eq!(result, Some("state4".to_string()));
    
    // Stack: [state2, state3], Redo: [state5]
    // Undo with current state4 should give us state3
    let result = history.undo("state4".to_string());
    assert_eq!(result, Some("state3".to_string()));
    
    // Stack: [state2], Redo: [state5, state4]
    // Undo with current state3 should give us state2
    let result = history.undo("state3".to_string());
    assert_eq!(result, Some("state2".to_string()));
    
    // Stack: [], Redo: [state5, state4, state3]
    // No more undo available
    let result = history.undo("state2".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_clear() {
    let mut history = UndoHistory::new(10);
    
    history.push("state1".to_string());
    history.push("state2".to_string());
    history.undo("state3".to_string());
    
    assert!(history.can_undo());
    assert!(history.can_redo());
    
    history.clear();
    
    assert!(!history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_undo_empty_stack() {
    let mut history = UndoHistory::new(10);
    let result = history.undo("current".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_redo_empty_stack() {
    let mut history = UndoHistory::new(10);
    let result = history.redo("current".to_string());
    assert_eq!(result, None);
}
