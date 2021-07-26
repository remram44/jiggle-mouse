use enigo::{Enigo, MouseControllable};
use std::thread::sleep;
use std::time::Duration;

const PIXELS: i32 = 5;
const INTERVAL: u64 = 50;
const DURATION_MS: u64 = 100;

fn main() {
    let mut enigo = Enigo::new();
    println!("Moving mouse by {} pixels every {} seconds,", PIXELS, INTERVAL);
    println!("until you close the window.");
    loop {
        sleep(Duration::from_secs(INTERVAL));
        enigo.mouse_move_relative(PIXELS, 0);
        sleep(Duration::from_millis(DURATION_MS));
        enigo.mouse_move_relative(-PIXELS, 0);
    }
}
