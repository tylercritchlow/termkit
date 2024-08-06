use std::thread;
use term_kit::spinner::Spinner;
fn main() {
    let mut spinner = Spinner::new("fdkzls aekfl asdjfk alds".to_string());
    spinner.render(); // Start spinning

    thread::sleep(std::time::Duration::from_secs(5)); // Simulate some work

    spinner.stop(); // Stop and clear the spinner
    println!("Work completed!");
}
