use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Open file")
        .width_request(100) // Make the button small
        .height_request(40)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Center the button using a vertical Box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_valign(gtk::Align::Center);
    vbox.set_halign(gtk::Align::Center);
    vbox.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Image Viewer")
        .default_width(600)
        .default_height(400)
        .child(&vbox)
        .build();

    // Present window
    window.present();
}
