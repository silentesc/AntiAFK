use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Duration;
use sysinfo::System;

pub fn start_health_monitor() {
    const SLEEP_SECONDS: u64 = 3;
    let mut system = System::new_all();

    // Initialize terminal and execute updates
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0),).unwrap();

    // Define content for the multiple lines
    let mut lines: Vec<String> = Vec::new();

    loop {
        // Update the system information
        system.refresh_all();

        lines.clear();

        // Print the system information
        lines.push(format!("Processes running: {}", system.processes().len()));

        lines.push(format!("CPU:"));
        lines.push(format!("  - Physical Cores amount: {}", system.physical_core_count().unwrap_or(0)));
        lines.push(format!("  - Total CPU Usage: {}%", system.global_cpu_info().cpu_usage()));
        for (i, cpu) in system.cpus().iter().enumerate() {
            lines.push(format!("  - CPU {}: {}%", i, cpu.cpu_usage()));
        }

        lines.push(format!("Memory:"));
        lines.push(format!("  - Total Memory: {}MB", system.total_memory() / 1000 / 1000));
        lines.push(format!("  - Used Memory: {}MB", system.used_memory() / 1000 / 1000));
        lines.push(format!("  - Free Memory: {}MB", system.free_memory() / 1000 / 1000));

        lines.push(format!("Swap:"));
        lines.push(format!("  - Total Swap: {}MB", system.total_swap() / 1000 / 1000));
        lines.push(format!("  - Used Swap: {}MB", system.used_swap() / 1000 / 1000));
        lines.push(format!("  - Free Swap: {}MB", system.free_swap() / 1000 / 1000));

        // 

        // Print each line with a delay in between
        for (i, line) in lines.iter().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(0, i as u16),
                Print(line),
                Clear(ClearType::UntilNewLine),
                cursor::MoveToNextLine(1),
            )
            .unwrap();
            stdout.flush().unwrap();
        }

        // Move cursor to the bottom after updating lines
        execute!(stdout, cursor::MoveTo(0, lines.len() as u16),).unwrap();

        stdout.flush().unwrap();

        // Sleep before next update
        std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
    }
}
