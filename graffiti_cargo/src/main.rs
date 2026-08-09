use adw::prelude::*;
use id3::{Tag, TagLike};

use adw::glib;
use adw::gtk::{Box, ListBox, Orientation, SelectionMode, Button, FileDialog};
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn file_demo() -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    let tag = Tag::read_from_path("/home/fabianstrandqvist/Music/Kall_Me_Soon_prod._Goyxrd_x_Telxry.mp3")?;
    return Ok(tag.title().unwrap_or("Unknown").to_string());
}

fn build_ui(app: &Application) {
    let filedialog = FileDialog::builder()
        .title("Select a file")
        .accept_label("Open")
        .modal(true)
        .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        let dialog = filedialog.clone();
        let window = button.root().and_downcast::<adw::gtk::Window>();
        glib::spawn_future_local(async move {
            match dialog.open_future(window.as_ref()).await {
                Ok(file) => {
                    println!("Selected file: {:?}", file);
                    match file_demo() {
                        Ok(title) => {
                            println!("Title: {}", title);
                        }
                        Err(err) => {
                            eprintln!("Error reading file: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error selecting file: {:?}", err);
                }
            }
        });
    });


    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&button);


    let window = ApplicationWindow::builder()
        .application(app)
        .title("Graffiti")
        .default_width(350)
        // add content to window
        .content(&content)
        .build();

    // Present window
    window.present();
}
