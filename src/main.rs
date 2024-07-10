use windows_sys::Win32::System::Power::SetThreadExecutionState;
use windows_sys::Win32::System::Power::ES_CONTINUOUS;
use windows_sys::Win32::System::Power::ES_DISPLAY_REQUIRED;
use windows_sys::Win32::System::Power::ES_SYSTEM_REQUIRED;

fn main() {
    // Prevent the system from going idle
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED);
    }

    // Print a message to the console
    println!("");
    println!("    _          _   _      _    _____ _  __         ___   _   ___  ");
    println!("   / \\   _ __ | |_(_)    / \\  |  ___| |/ / __   __/ _ \\ / | / _ \\ ");
    println!("  / _ \\ | '_ \\| __| |   / _ \\ | |_  | ' /  \\ \\ / / | | || || | | |");
    println!(" / ___ \\| | | | |_| |  / ___ \\|  _| | . \\   \\ V /| |_| || || |_| |");
    println!("/_/   \\_\\_| |_|\\__|_| /_/   \\_\\_|   |_|\\_\\   \\_/  \\___(_)_(_)___/ ");
    println!("");
    println!("System is prevented from going idle. Press Enter to exit.");
    
    // Simulate long-running task
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Allow the system to go idle again
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
}
