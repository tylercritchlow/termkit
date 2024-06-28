use term_kit::{ meter::Meter, color::Color, keyboard::{ is_key_pressed, KeyCode } };
use sysinfo::System;

fn main() {
    let mut cpu_meter = Meter::new("CPU Usage:".to_string(), 100.0, Some(Color::Red)); 
    let mut sys = System::new_all();
    println!("ctrl+c to quit.");
    
    loop {
        sys.refresh_cpu();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        
        cpu_meter.refresh(cpu_usage.into(), 1000); 
    }
}
