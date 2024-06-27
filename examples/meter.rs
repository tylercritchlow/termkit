use term_kit::meter::Meter;
use crossterm::{
    cursor,
    event::{read, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut meter = Meter::new(0.0, 100.0, "CPU Usage".to_string());
    let mut system = System::new_all();

    loop {
        system.refresh_all();
        let cpu_usage = system.global_cpu_info().cpu_usage();
        meter.value = cpu_usage as f64; // Update the meter's value directly
        meter.render();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}
