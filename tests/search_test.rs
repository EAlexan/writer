use writer::search::SearchState;

#[test]
fn test_update_results() {
    let mut search = SearchState::default();
    search.query = "test".to_string();
    
    let text = "this is a test string with test in it";
    search.update_results(text);
    
    assert_eq!(search.results.len(), 2);
    assert_eq!(search.results, vec![10, 27]);
    assert_eq!(search.current_match_index, Some(0));
}

#[test]
fn test_update_results_no_match() {
    let mut search = SearchState::default();
    search.query = "notfound".to_string();
    
    let text = "this is a test string";
    search.update_results(text);
    
    assert!(search.results.is_empty());
    assert_eq!(search.current_match_index, None);
}

#[test]
fn test_update_results_empty_query() {
    let mut search = SearchState::default();
    search.query = "".to_string();
    
    let text = "this is a test string";
    search.update_results(text);
    
    assert!(search.results.is_empty());
    assert_eq!(search.current_match_index, None);
}

#[test]
fn test_find_next() {
    let mut search = SearchState::default();
    search.results = vec![10, 27, 40];
    search.current_match_index = Some(0);
    
    search.find_next();
    assert_eq!(search.current_match_index, Some(1));
    
    search.find_next();
    assert_eq!(search.current_match_index, Some(2));
    
    search.find_next();
    assert_eq!(search.current_match_index, Some(0)); // Wrap around
}

#[test]
fn test_find_previous() {
    let mut search = SearchState::default();
    search.results = vec![10, 27, 40];
    search.current_match_index = Some(0);
    
    search.find_previous();
    assert_eq!(search.current_match_index, Some(2)); // Wrap around
    
    search.find_previous();
    assert_eq!(search.current_match_index, Some(1));
    
    search.find_previous();
    assert_eq!(search.current_match_index, Some(0));
}
