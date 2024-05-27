use num::Num;
use num_bigint::BigUint;

slint::include_modules!();

const VALID_KEYS: &'static [&'static str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();
    ui.on_update_screen(move |s| {
        let ui = ui_weak.unwrap();
        let s = s.to_uppercase();
        let mut screen = ui.get_screen();
        if screen == "0" || ui.get_dirty() {
            ui.set_screen(s.into());
            ui.set_dirty(false);
        } else {
            screen.push_str(&s);
            ui.set_screen(screen);
        }
    });

    let ui_weak = ui.as_weak();
    ui.on_update_radix_boxes(move || {
        let ui = ui_weak.unwrap();
        let screen = ui.get_screen();
        let value = match ui.get_mode().as_str() {
            "Hex" => BigUint::from_str_radix(&screen, 16).unwrap(),
            "Oct" => BigUint::from_str_radix(&screen, 8).unwrap(),
            "Bin" => BigUint::from_str_radix(&screen, 2).unwrap(),
            _ => screen.parse::<BigUint>().unwrap(),
        };
        ui.set_dec(format!("{}", value).into());
        ui.set_hex(format!("{:X}", value).into());
        ui.set_oct(format!("{:o}", value).into());
        ui.set_bin(format!("{:b}", value).into());
    });

    let ui_weak = ui.as_weak();
    ui.on_backspace(move || {
        let ui = ui_weak.unwrap();
        let screen = ui.get_screen();
        if screen.len() == 1 {
            ui.set_screen(String::from("0").into());
        } else {
            let new_screen = String::from(&screen[0..screen.len() - 1]);
            ui.set_screen(new_screen.into());
        }
    });

    let ui_weak = ui.as_weak();
    ui.on_check_valid_keypress(move |k| {
        let ui = ui_weak.unwrap();
        let k = k.to_uppercase();
        let mode = ui.get_mode();
        let valid_keys = valid_keys(mode.as_str());
        valid_keys.contains(&k.as_str())
    });

    ui.run()
}

fn valid_keys(mode: &str) -> &[&str] {
    match mode {
        "Bin" => &VALID_KEYS[0..2],
        "Hex" => &VALID_KEYS[..],
        "Oct" => &VALID_KEYS[0..8],
        _ => &VALID_KEYS[0..9],
    }
}
