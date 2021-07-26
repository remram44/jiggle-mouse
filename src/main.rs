use enigo::{Enigo, MouseControllable};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    loop {
        sleep(Duration::from_secs(58));
        enigo.mouse_move_relative(5, 0);
        sleep(Duration::from_millis(100));
        enigo.mouse_move_relative(-5, 0);
    }
}
