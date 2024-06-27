use term_kit::listselector::ListSelector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = vec![
        "Red".to_string(),
        "Green".to_string(),
        "Blue".to_string(),
        "Yellow".to_string(),
        "Orange".to_string(),
        "Purple".to_string(),
        "Pink".to_string(),
        "Brown".to_string(),
        "Black".to_string(),
        "White".to_string(),
        "Gray".to_string(),
        "Cyan".to_string(),
    ];
    let mut selector = ListSelector::new(options);

    if let Some(selected_color) = selector.run()? {
        println!("You selected: {}", selected_color);
    }

    Ok(())
}