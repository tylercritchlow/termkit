use std::thread;
use term_kit::spinner::*;
fn main() {
    let spinner = Spinner::new()
        .with_label("Loading...".to_string())
        .with_label_position(After); // if not set, default is After

    spinner.start();

    thread::sleep(std::time::Duration::from_secs(5)); // Simulate some work

    spinner.stop();

    println!("Work completed!");
}
