#[cfg(test)]
mod tests {
    use term_kit::{color::Color, infobox::InfoBox};
    #[test]
    fn test_infobox_new() {
        let infobox = InfoBox::new(
            "Title".to_string(),
            "Message".to_string(),
            50,
            Some(Color::Blue),
            Some(Color::Green),
            Some(Color::Red),
        );
        assert_eq!(infobox.title, "Title");
        assert_eq!(infobox.message, "Message");
        assert_eq!(infobox.width, 50);
    }

    #[test]
    fn test_infobox_with_padding() {
        let infobox = InfoBox::new(
            "Title".to_string(),
            "Message".to_string(),
            50,
            Some(Color::Blue),
            Some(Color::Green),
            Some(Color::Red),
        )
        .with_padding(10);
        assert_eq!(infobox.padding, 10);
    }

    #[test]
    fn test_infobox_render() {
        let mut buffer = Vec::new();
        let mut infobox = InfoBox::new(
            "Title".to_string(),
            "Message".to_string(),
            50,
            Some(Color::Blue),
            Some(Color::Green),
            Some(Color::Red),
        )
        .with_custom_writer(&mut buffer);

        // Call render()
        let result = infobox.render();


        assert!(result.is_ok());
    }
}
