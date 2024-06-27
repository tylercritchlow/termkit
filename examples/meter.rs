use term_kit::meter::Meter;
use term_kit::color::Color;
use sysinfo::System;

fn main() {
    let mut cpu_meter = Meter::new("CPU Usage:".to_string(), 100.0, Some(Color::Red)); 
    let mut sys = System::new_all();

    loop {
        sys.refresh_cpu();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        
        cpu_meter.refresh(cpu_usage.into(), 1000); 
    }
}
