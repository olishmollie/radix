use num_bigint::BigUint;
use num::Num;
use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();
    ui.on_update(move ||{
        let ui = ui_weak.unwrap();

        let label = ui.get_label();
        let value = match ui.get_mode().as_str() {
            "Hex" => BigUint::from_str_radix(&label, 16).unwrap(),
            "Oct" => BigUint::from_str_radix(&label, 8).unwrap(),
            "Bin" => BigUint::from_str_radix(&label, 2).unwrap(),
            _ => label.parse::<BigUint>().unwrap(),
        };


        ui.set_dec(format!("{}", value).into());
        ui.set_hex(format!("{:X}", value).into());
        ui.set_oct(format!("{:o}", value).into());
        ui.set_bin(format!("{:b}", value).into());
    });

    let ui_weak = ui.as_weak();
    ui.on_backspace(move || {
        let ui = ui_weak.unwrap();
        let label = ui.get_label();
        if label.len() == 1 {
            ui.set_label(SharedString::from("0"));
        } else {
            let new_label = SharedString::from(&label[0..label.len() - 1]);
            ui.set_label(new_label);
        }
    });

    ui.run()
}
