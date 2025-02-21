use rand::{rng, Rng};
use std::{thread, time::Duration};
use term_kit::progressbar::ProgressBarManager;

fn main() {
    println!("Initializing downloads...");
    println!("Fetching metadata...");
    println!("Preparing files...");

    let mut manager = ProgressBarManager::new();


    manager.add_bar("Downloading 1", 100, 30);
    manager.add_bar("Downloading 2", 100, 30);
    manager.add_bar("Downloading 3", 100, 30);

    let mut progresses = vec![0; 3];

    // While any of the progresses is less than 100, keep incrementing them
    while progresses.iter().any(|&p| p < 100) {
        for i in 0..3 {
            if progresses[i] < 100 {
                let increment = rng().random_range(1..=3);
                progresses[i] = (progresses[i] + increment).min(100);
                manager.update_bar(i, progresses[i]);
            }
        }
        manager
            .render_all()
            .expect("A problem rendering the progress bars occurred");
        thread::sleep(Duration::from_millis(200));
    }

    println!("Downloads complete!");
}
