use std::time::{UNIX_EPOCH, SystemTime};

use inputbot::KeybdKey;
use screenshots::Screen;

fn main() {
    println!("SatSetSnap is running...");
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    KeybdKey::RControlKey.bind(move || {
        println!("Screenshot!");
        let image = screen.capture().unwrap();
        let name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        image.save(format!("{name}.png")).unwrap();
    });

    inputbot::handle_input_events();
}
