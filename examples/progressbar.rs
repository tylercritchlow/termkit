use termkit::progressbar::ProgressBar;
use rand::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut progress_bar = ProgressBar::new(0, 100, 20, String::from("Loading..."));

    for i in 0..=100 {
        progress_bar.value = i;
        progress_bar.render()?;

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(rand::thread_rng().gen_range(0..120)));
    }
    Ok(())
}
