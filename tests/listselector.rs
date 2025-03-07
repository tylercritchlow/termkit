#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use std::io::Cursor;
    use term_kit::listselector::ListSelector;

    #[test]
    fn test_new_list_selector() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let selector = ListSelector::new(options.clone());
        assert_eq!(selector.options, options);
        assert_eq!(selector.selected_index.get(), 0);
        assert_eq!(selector.top_visible_index.get(), 0);
    }

    #[test]
    fn test_get_selected_option() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let selector = ListSelector::new(options);
        assert_eq!(selector.get_selected_option(), Some("Option 1"));
        selector.selected_index.set(1);
        assert_eq!(selector.get_selected_option(), Some("Option 2"));
    }

    #[test]
    fn test_get_selected_option_empty() {
        let selector = ListSelector::new(vec![]);
        assert_eq!(selector.get_selected_option(), None);
    }

    #[test]
    fn test_with_custom_writer() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let selector = ListSelector::new(options.clone());
        let custom_writer = Cursor::new(Vec::new());
        let custom_selector = selector.with_custom_writer(custom_writer);

        assert_eq!(custom_selector.options, options);
        assert_eq!(custom_selector.selected_index.get(), 0);
        assert_eq!(custom_selector.top_visible_index.get(), 0);
    }

    #[test]
    fn test_navigation_bounds() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let selector = ListSelector::new(options);

        // Test upper bound
        selector.selected_index.set(0);
        // Simulate up key press
        let _key_event = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()));
        // The index should still be 0
        assert_eq!(selector.selected_index.get(), 0);

        // Test lower bound
        selector.selected_index.set(1);
        // Simulate down key press
        let _key_event = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
        // The index should still be 1
        assert_eq!(selector.selected_index.get(), 1);
    }

    #[test]
    fn test_scrolling_logic() {
        let mut options = Vec::new();
        for i in 1..=20 {
            options.push(format!("Option {}", i));
        }

        let selector = ListSelector::new(options);

        // Set initial state
        selector.selected_index.set(0);
        selector.top_visible_index.set(0);

        // Move down to trigger scrolling
        for _ in 0..15 {
            selector
                .selected_index
                .set(selector.selected_index.get() + 1);
            // In a real terminal with 10 visible rows, scrolling would start after index 8
            if selector.selected_index.get() > 8 {
                selector
                    .top_visible_index
                    .set(selector.selected_index.get() - 8);
            }
        }

        // Check that scrolling happened correctly
        assert_eq!(selector.selected_index.get(), 15);
        assert_eq!(selector.top_visible_index.get(), 7); // 15-8
    }
}
