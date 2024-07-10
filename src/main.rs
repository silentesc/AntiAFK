use std::time::Duration;

use enigo::{Enigo, Keyboard, Settings};

fn main() {
    // Print a message to the console
    println!("");
    println!("    _          _   _      _    _____ _  __         ___   _   ___  ");
    println!("   / \\   _ __ | |_(_)    / \\  |  ___| |/ / __   __/ _ \\ / | / _ \\ ");
    println!("  / _ \\ | '_ \\| __| |   / _ \\ | |_  | ' /  \\ \\ / / | | || || | | |");
    println!(" / ___ \\| | | | |_| |  / ___ \\|  _| | . \\   \\ V /| |_| || || |_| |");
    println!("/_/   \\_\\_| |_|\\__|_| /_/   \\_\\_|   |_|\\_\\   \\_/  \\___(_)_(_)___/ ");
    println!("");

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    loop {
        // Simulate a key press
        enigo.key(enigo::Key::LWin, enigo::Direction::Click).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        enigo.key(enigo::Key::LWin, enigo::Direction::Click).unwrap();

        // Sleep for 3 seconds
        std::thread::sleep(Duration::from_secs(5));
    }
}
