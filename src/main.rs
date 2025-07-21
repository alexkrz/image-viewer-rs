use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileDialog, Picture, gio, glib};
const APP_ID: &str = "org.gtk_rs.ImageViewer";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Simple Image Viewer")
        .default_width(600)
        .default_height(400)
        .build();

    let picture = Picture::new();

    let button = Button::with_label("Open Image");
    button.connect_clicked(clone!(
        #[weak]
        window,
        #[weak]
        picture,
        move |_| {
            let dialog = FileDialog::builder()
                .title("Open File")
                .accept_label("Open")
                .build();

            dialog.open(Some(&window), gio::Cancellable::NONE, move |file| {
                if let Ok(file) = file {
                    picture.set_file(Some(&file));
                }
            });
        }
    ));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.append(&button);
    vbox.append(&picture);

    window.set_child(Some(&vbox));
    window.present();
}
