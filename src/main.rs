use std::{thread, time::Duration};

use keepawake::Builder;

fn main() {
    let _awake = Builder::default()
        .display(true)
        .idle(true)
        .sleep(true)
        .create()
        .expect("Failed to prevent sleep");

    println!("PC will stay awake until this program is closed.");

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
