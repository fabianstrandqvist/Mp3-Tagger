use adw::prelude::*;
use id3::{Tag, TagLike, Version, ErrorKind};

use std::path::Path;
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

fn set_artist(path: &Path, artist: &str) -> Result<(), id3::Error> {
    let mut tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(e) if matches!(e.kind, ErrorKind::NoTag) => Tag::new(),
        Err(e) => return Err(e),
    };

    tag.set_artist(artist);
    tag.write_to_path(path, Version::Id3v24)?;
    Ok(())
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
                    let Some(path) = file.path() else {
                        eprintln!("not a local file");
                        return;
                    };

                    // println!("Selected file: {:?}", file);
                    match set_artist(&path, "Artist") {
                        Ok(()) => {
                            println!("Tags updated successfully");
                        }
                        Err(err) => {
                            eprintln!("Error updating tag: {:?}", err);
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
