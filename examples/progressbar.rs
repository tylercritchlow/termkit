use rand::{rng, Rng};
use term_kit::progressbar::ProgressBar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut progress_bar = ProgressBar::new("Loading...", 100, 20);

    for i in 0..=100 {
        progress_bar.update(i);
        progress_bar.render()?;

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(rng().random_range(0..120)));
    }
    Ok(())
}
