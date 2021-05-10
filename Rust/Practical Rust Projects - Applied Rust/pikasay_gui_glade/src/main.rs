use gio::prelude::*;
use gtk::prelude::*;
use std::env;

fn main() {
    let app = gtk::Application::new(
        Some("com.danlogs.pikasay_gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Failed to initialize GTK.");

    app.connect_activate(|app| {
        let glade_src = include_str!("../layout1.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window : gtk::Window = builder.get_object("applicationwindow1").unwrap();
        window.set_application(Some(app));

        // get all the UI elements.
        let message_input : gtk::Entry = builder.get_object("message_input").unwrap();
        let button : gtk::Button = builder.get_object("generate_btn").unwrap();
        let message_output: gtk::Label = builder.get_object("message_output").unwrap();
        let is_returned_switch : gtk::Switch = builder.get_object("is_returned_switch").unwrap();
        let image_output: gtk::Image = builder.get_object("image_output").unwrap();
        let image_output_clone = image_output.clone();

        button.connect_clicked(move |_| { // handler for a button click
            let is_returned = is_returned_switch.get_active();
            let message_gstring = message_input.get_text();
            let message = if message_gstring.as_str().is_empty() { "Created by danlogs!" } else { message_gstring.as_str() };
            if is_returned {
                message_output.set_text(&String::new());
                image_output_clone.set_from_file("./images/pokeball.png");
            } else {
                image_output_clone.set_from_file("./images/pikachu.png");
                message_output.set_text(&format!(
"
| {} |
\\  /
 \\/
",
                    message,
                ));
            }
            image_output_clone.show();
        });

        window.show_all();
        image_output.hide(); 
        // we hide the image after showing so that it only shows
        // after we hit the generate btn

    });
    app.run(&env::args().collect::<Vec<_>>());
}