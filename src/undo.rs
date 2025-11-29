use std::collections::VecDeque;

/// Manages undo/redo history for text editing operations
/// 
/// Uses a snapshot-based approach where each state stores the complete text.
/// The undo and redo stacks are implemented using VecDeque for efficient
/// operations at both ends.
pub struct UndoHistory {
    undo_stack: VecDeque<String>,
    redo_stack: VecDeque<String>,
    max_capacity: usize,
}

impl UndoHistory {
    /// Create a new UndoHistory with the specified maximum capacity
    /// 
    /// # Arguments
    /// * `max_capacity` - Maximum number of undo states to keep in memory
    /// 
    /// # Example
    /// ```no_run
    /// # use writer::undo::UndoHistory;
    /// let history = UndoHistory::new(100);
    /// ```
    pub fn new(max_capacity: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_capacity),
            redo_stack: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    /// Push a new state onto the undo stack
    /// 
    /// This clears the redo stack since making a new change invalidates
    /// any previously undone states. If the undo stack is at capacity,
    /// the oldest state is removed.
    /// 
    /// # Arguments
    /// * `text` - The text state to save
    pub fn push(&mut self, text: String) {
        // Clear redo stack when new change is made
        self.redo_stack.clear();

        // If at capacity, remove oldest entry
        if self.undo_stack.len() >= self.max_capacity {
            self.undo_stack.pop_front();
        }

        self.undo_stack.push_back(text);
    }

    /// Undo the last change
    /// 
    /// Moves the current state to the redo stack and returns the previous state
    /// from the undo stack.
    /// 
    /// # Arguments
    /// * `current` - The current text state
    /// 
    /// # Returns
    /// * `Some(String)` - The previous text state if undo is available
    /// * `None` - If there are no states to undo
    pub fn undo(&mut self, current: String) -> Option<String> {
        if let Some(previous) = self.undo_stack.pop_back() {
            // Push current state to redo stack
            if self.redo_stack.len() >= self.max_capacity {
                self.redo_stack.pop_front();
            }
            self.redo_stack.push_back(current);
            Some(previous)
        } else {
            None
        }
    }

    /// Redo the last undone change
    /// 
    /// Moves the current state to the undo stack and returns the next state
    /// from the redo stack.
    /// 
    /// # Arguments
    /// * `current` - The current text state
    /// 
    /// # Returns
    /// * `Some(String)` - The next text state if redo is available
    /// * `None` - If there are no states to redo
    pub fn redo(&mut self, current: String) -> Option<String> {
        if let Some(next) = self.redo_stack.pop_back() {
            // Push current state to undo stack
            if self.undo_stack.len() >= self.max_capacity {
                self.undo_stack.pop_front();
            }
            self.undo_stack.push_back(current);
            Some(next)
        } else {
            None
        }
    }

    /// Check if undo is available
    /// 
    /// # Returns
    /// * `true` if there are states in the undo stack
    /// * `false` otherwise
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    /// 
    /// # Returns
    /// * `true` if there are states in the redo stack
    /// * `false` otherwise
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Peek at the top of the undo stack
    pub fn peek_undo(&self) -> Option<&String> {
        self.undo_stack.back()
    }

    /// Clear all undo and redo history
    /// 
    /// This is useful when opening a new file or when you want to
    /// reset the history state.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self::new(100)
    }
}
