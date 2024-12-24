use gtk4::prelude::*; // Import GTK prelude for traits
use gtk4::{Application, ApplicationWindow, Button};

fn main() {
    // Initialize GTK application
    let app = Application::builder()
        .application_id("com.example.gtk-app")
        .build();

    app.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello, GTK in Rust!")
            .default_width(400)
            .default_height(300)
            .build();

        // Create a button
        let button = Button::builder()
            .label("Click me!")
            .build();

        // Connect button signal
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });

        // Add button to the window
        window.set_child(Some(&button));

        // Show the window
        window.set_visible(true);
    });

    // Run the application
    app.run();
}
