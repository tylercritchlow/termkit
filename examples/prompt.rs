use term_kit::prompt::Prompt;

fn main() {
    let options = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ];

    let mut prompt = Prompt::new("Choose an option".to_string(), options);

    if let Some(selected_option) = prompt.run().unwrap() {
        println!("You selected: {}", selected_option);
    }
}
