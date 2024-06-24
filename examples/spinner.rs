use std::thread;
use termkit::spinner::Spinner;
fn main() {
    let mut spinner = Spinner::new();
    spinner.start(); // Start spinning

    thread::sleep(std::time::Duration::from_secs(5)); // Simulate some work

    spinner.stop(); // Stop and clear the spinner
    println!("Work completed!");
}
