use fk::progressbar::ProgressBar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut progress_bar = ProgressBar::new(0, 100, 20, String::from("Loading..."));

    for i in 0..=100 {
        progress_bar.value = i;
        progress_bar.render()?;

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    Ok(())
}
