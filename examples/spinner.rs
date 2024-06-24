use termkit::spinner;

fn main() {
    let mut spinner = spinner::Spinner::new();
    spinner.start();
    std::thread::sleep(std::time::Duration::from_secs(5));
    spinner.stop();
}