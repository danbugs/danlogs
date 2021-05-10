use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};
use std::env;

fn main() {
    let app = Application::new(
        Some("com.danlogs.pikasay_gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Failed to initialize GTK.");

    app.connect_activate(|app| {

        let window = ApplicationWindow::new(app);
        window.set_title("Pikasay");
        window.set_default_size(256, 128);

        let layout_box = Box::new(Orientation::Vertical, 0);
        let message = Label::new(Some("Pika!\n     \\\n      \\"));
        layout_box.add(&message);
        let pikachu = Image::from_file("./images/pikachu.png");
        layout_box.add(&pikachu);

        window.add(&layout_box);

        window.show_all();
    });
    app.run(&env::args().collect::<Vec<_>>());
}
