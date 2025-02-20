use rand::{rng, Rng};
use term_kit::progressbar::ProgressBar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut progress_bar = ProgressBar::new(0, 100, 20, String::from("Loading..."));

    for i in 0..=100 {
        //FIXME: This should work whether it is 100 or not
        progress_bar.value = i;
        progress_bar.render()?;

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(rng().random_range(0..120)));
    }
    Ok(())
}
