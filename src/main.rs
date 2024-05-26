slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();
    ui.on_changed(move |n|{
        let ui = ui_weak.unwrap();
        if ui.get_label() == "0" {
            ui.set_label(n.to_string().into());
        } else {
            ui.set_label(ui.get_label() + &n.to_string());
        }

        let value = ui.get_label().parse::<u64>().unwrap();
        ui.set_dec(format!("{}", value).into());
        ui.set_hex(format!("{:X}", value).into());
        ui.set_oct(format!("{:o}", value).into());
        ui.set_bin(format!("{:b}", value).into());
    });

    ui.run()
}
