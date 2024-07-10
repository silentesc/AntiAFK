use std::time::Duration;

use sysinfo::System;

pub mod antiafk;

fn main() {
    // Start the anti-afk thread
    antiafk::start_antiafk();

    // Print a multiline ASCII art
    println!(
        "
        .·:'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''':·.
        : :  ____            _                   _                _ _   _      : :
        : : / ___| _   _ ___| |_ ___ _ __ ___   | |__   ___  __ _| | |_| |__   : :
        : : \\___ \\| | | / __| __/ _ \\ '_ ` _ \\  | '_ \\ / _ \\/ _` | | __| '_ \\  : :
        : :  ___) | |_| \\__ \\ ||  __/ | | | | | | | | |  __/ (_| | | |_| | | | : :
        : : |____/ \\__, |___/\\__\\___|_| |_| |_| |_| |_|\\___|\\__,_|_|\\__|_| |_| : :
        : :        |___/           _ _                                         : :
        : :  _ __ ___   ___  _ __ (_) |_ ___  _ __                             : :
        : : | '_ ` _ \\ / _ \\| '_ \\| | __/ _ \\| '__|                            : :
        : : | | | | | | (_) | | | | | || (_) | |                               : :
        : : |_| |_| |_|\\___/|_| |_|_|\\__\\___/|_|                               : :
        '·:....................................................................:·'
    "
    );

    const SLEEP_SECONDS: u64 = 1;
    let mut system = System::new_all();

    loop {
        // Update the system information
        system.refresh_all();

        // Print the system information
        println!("Processes running: {}", system.processes().len());

        println!("CPU:");
        println!("  - CPU Usage: {}%", system.global_cpu_info().cpu_usage());
        println!("  - CPU Frequency: {} MHz", system.global_cpu_info().frequency());
        println!("  - Physical Cores: {}", system.physical_core_count().unwrap_or(0));

        println!("Memory:");
        println!("  - Total Memory: {}MB", system.total_memory() / 1000 / 1000);
        println!("  - Used Memory: {}MB", system.used_memory() / 1000 / 1000);
        println!("  - Free Memory: {}MB", system.free_memory() / 1000 / 1000);

        println!("Swap:");
        println!("  - Total Swap: {}MB", system.total_swap() / 1000 / 1000);
        println!("  - Used Swap: {}MB", system.used_swap() / 1000 / 1000);
        println!("  - Free Swap: {}MB", system.free_swap() / 1000 / 1000);

        // Sleep for 1 second
        std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
    }
}
