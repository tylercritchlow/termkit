use term_kit::color::Color;
use term_kit::infobox::InfoBox;

fn main() {
    let mut infobox = InfoBox::new(
        String::from("Title"),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
        57,
        Some(Color::Yellow),
        Some(Color::Green),
        Some(Color::Red),
    ).with_padding(3);

    infobox.render().unwrap();
}