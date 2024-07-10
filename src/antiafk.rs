use std::time::Duration;

use enigo::{Enigo, Keyboard, Settings};

pub fn start_antiafk() {
    std::thread::spawn(move || {
        const SLEEP_SECONDS: u64 = 60 * 3;
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        loop {
            // Simulate a key release
            match enigo.key(enigo::Key::LWin, enigo::Direction::Release) {
                Ok(_) => (),
                Err(_) => (),
            }
            // Sleep before next key press
            std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
        }
    });
}
