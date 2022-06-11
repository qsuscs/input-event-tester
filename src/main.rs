use evdev;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button,
	  ComboBoxText};

fn main() {
    let app = Application::builder()
        .application_id("re.qsx.input-event-tester")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn update_devices(cbt: &ComboBoxText) {
    cbt.remove_all();
    for dev in evdev::enumerate() {
	let text = dev.name().unwrap_or("(none)");
	println!("{}", text);
	cbt.append_text(text);
    }
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    let cbt: ComboBoxText = builder
	.object("cbt-device")
	.expect("Could not get object `cbt-device` from builder.");
    // for dev in evdev::enumerate() {
    //     println!(
    //         "{}Physical path: {}\nInput ID: {:?}",
    //         dev,
    //         dev.physical_path().unwrap_or("(none)"),
    //         dev.input_id()
    //     );
    //     let text = dev.name().unwrap_or("(none)");
    //     cbt.append_text(text);
    // }
    update_devices(&cbt);

    let refresh: Button = builder
	.object("refresh")
	.expect("Could not get object `refresh` from builder.");
    refresh.connect_clicked(move |_| {
	update_devices(&cbt);
    });

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("Gamepad tester")
    //     .child(&cbt)
    //     .build();
    let window: ApplicationWindow = builder
	.object("window")
	.expect("Could not get object `window` from builder.");
    window.set_application(Some(app));

    window.present();
}
