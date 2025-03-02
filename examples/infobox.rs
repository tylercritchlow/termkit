use term_kit::color::Color;
use term_kit::infobox::InfoBox;

fn main() {
    let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let infobox = InfoBox::new(String::from("Title"), String::from(message), 57)
        .with_padding(3)
        .with_border_color(Color::Cyan)
        .with_title_color(Color::Yellow)
        .with_message_color(Color::Green);

    infobox.render().unwrap();
}
