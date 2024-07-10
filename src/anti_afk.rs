use std::{io::Write, time::Duration};

use enigo::{Enigo, Keyboard, Settings};

pub fn start_anti_afk() {
    // Print a message to the console
    println!(
        "
.·:'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''':·.
: :     _          _   _    _    _____ _  __         ___   _   ___   : :
: :    / \\   _ __ | |_(_)  / \\  |  ___| |/ / __   __/ _ \\ / | / _ \\  : :
: :   / _ \\ | '_ \\| __| | / _ \\ | |_  | ' /  \\ \\ / / | | || || | | | : :
: :  / ___ \\| | | | |_| |/ ___ \\|  _| | . \\   \\ V /| |_| || || |_| | : :
: : /_/   \\_\\_| |_|\\__|_/_/   \\_\\_|   |_|\\_\\   \\_/  \\___(_)_(_)___/  : :
'·:..................................................................:·'
    "
    );

    const SLEEP_SECONDS: u64 = 60 * 3;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut counter = 1;

    println!(
        "This program will simulate a key press every {} minutes to prevent AFK.",
        SLEEP_SECONDS / 60
    );
    println!("");

    loop {
        let current_time = chrono::Local::now().time().format("%H:%M:%S").to_string();

        // Simulate a key release
        match enigo.key(enigo::Key::LWin, enigo::Direction::Release) {
            Ok(_) => (),
            Err(e) => eprintln!("\r[{}] Error releasing key: {}", current_time, e),
        }

        // Print info
        print!(
            "\rTriggered anti afk {} times (Last trigger: {})",
            counter, current_time
        );
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(e) => eprintln!("[{}] Error flushing stdout: {}", current_time, e),
        }

        counter += 1;

        // Sleep before next key press
        std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
    }
}
