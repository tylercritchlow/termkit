use std::thread;
use term_kit::spinner::Spinner;
fn main() {
    let spinner = Spinner::new();
    spinner.render();

    thread::sleep(std::time::Duration::from_secs(5)); // Simulate some work

    spinner.stop();
    println!("Work completed!");
}
