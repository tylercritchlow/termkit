use term_kit::prompt::Prompt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt1 = Prompt::new(
        "What would you like to build?".to_string(),
        vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ],
    );

    let _selected = prompt1.run()?; // Run the prompt, and get the selected option

    Ok(())
}
// ? What would you like to build? ❯
// ♦ Blah (selected index)
//   Blah
//   Blah
