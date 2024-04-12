use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.inmert.tictactoe";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tic Tac Toe")
        .build();

    // Present window
    window.present();
}