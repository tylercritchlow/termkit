use sysinfo::System;
use term_kit::{color::Color, meter::Meter};

fn main() {
    let mut cpu_meter = Meter::new("CPU Usage:".to_string(), 100.0, Some(Color::Red));
    let mut sys = System::new_all();
    println!("ctrl+c to quit.");

    loop {
        sys.refresh_cpu_all();

        let cpu_usage = sys.global_cpu_usage();

        cpu_meter.refresh(cpu_usage.into(), 1000);
    }
}
