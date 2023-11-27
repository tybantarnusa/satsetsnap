use std::time::{UNIX_EPOCH, SystemTime};

use clipboard_win::{set_clipboard, formats};
use inputbot::KeybdKey;
use screenshots::Screen;
use image::{codecs::bmp::BmpEncoder, ColorType};

fn main() {
    println!("SatSetSnap is running...");
    let screens = Screen::all().expect("ERROR: Cannot get screens.");
    let screen = screens[0];

    KeybdKey::RControlKey.bind(move || {
        if let Ok(img) = screen.capture() {
            let name = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_millis(),
                Err(_) => 0,
            };

            if img.save(format!("{name}.bmp")).is_err() {
                eprintln!("ERROR: Cannot save image.");
            };

            let mut scr_img = Vec::new();
            for p in img.pixels() {
                scr_img.push(p[0]);
                scr_img.push(p[1]);
                scr_img.push(p[2]);
            }

            let mut buf = Vec::new();
            let mut encoder = BmpEncoder::new(&mut buf);
            if let Err(e) = encoder.encode(&scr_img, img.width(), img.height(), ColorType::Rgb8) {
                eprintln!("ERROR: Cannot encode to BMP. Detail: {e}");
            }

            let _ = set_clipboard(formats::Bitmap, buf).map_err(|e| {
                eprintln!("ERROR: Cannot write screenshot to clipboard. Detail: {e}");
            });
        } else {
            eprintln!("ERROR: Cannot capture screen.");
        };
        println!("Screenshot!");
    });

    inputbot::handle_input_events();
}
