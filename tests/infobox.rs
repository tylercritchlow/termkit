#[cfg(test)]
mod tests {
    use crossterm::style::Color;
    use std::io::Cursor;
    use term_kit::infobox::InfoBox;

    #[test]
    fn test_infobox_creation() {
        let infobox = InfoBox::new("Test Title", "Test Message", 20);
        assert_eq!(infobox.title, "Test Title");
        assert_eq!(infobox.message, "Test Message");
        assert_eq!(infobox.width, 20);
        assert_eq!(infobox.padding, 2);
    }

    #[test]
    fn test_infobox_with_custom_padding() {
        let infobox = InfoBox::new("Test Title", "Test Message", 20).with_padding(4);
        assert_eq!(infobox.padding, 4);
    }

    #[test]
    fn test_infobox_with_custom_writer() {
        let buffer = Cursor::new(Vec::new());
        let infobox = InfoBox::new("Test Title", "Test Message", 20).with_custom_writer(buffer);

        // We're mainly testing that it compiles correctly here
        assert_eq!(infobox.width, 20);
    }

    #[test]
    fn test_infobox_with_custom_colors() {
        let infobox = InfoBox::new("Test Title", "Test Message", 20)
            .with_title_color(Color::Red)
            .with_border_color(Color::Green)
            .with_message_color(Color::Blue);

        assert_eq!(infobox.title_color, Color::Red);
        assert_eq!(infobox.border_color, Color::Green);
        assert_eq!(infobox.message_color, Color::Blue);
    }

    #[test]
    fn test_infobox_render_output() {
        let buffer = Cursor::new(Vec::new());
        let infobox =
            InfoBox::new("Render Test", "This is a test message.", 20).with_custom_writer(buffer);

        let result = infobox.render();
        assert!(result.is_ok());

        // We could check the exact output, but it would be complex due to ANSI codes
        // This test primarily ensures rendering doesn't panic
    }
}
