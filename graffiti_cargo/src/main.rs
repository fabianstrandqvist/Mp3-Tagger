use adw::prelude::*;
use id3::{Tag, TagLike, Version, ErrorKind};

use std::path::Path;
use std::{cell::RefCell, rc::Rc, path::PathBuf};
use adw::glib;
use adw::gio;
use adw::gtk::{Box, ListBox, Orientation, SelectionMode, Button, FileDialog, Entry, FileFilter};
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
    let selected_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

    let mp3 = FileFilter::new();
    mp3.set_name(Some("MP3 files"));
    mp3.add_mime_type("audio/mpeg");
    mp3.add_suffix("mp3");

    let filters = gio::ListStore::new::<FileFilter>();
    filters.append(&mp3);

    let filedialog = FileDialog::builder()
        .title("Select a file")
        .accept_label("Open")
        .filters(&filters)
        .modal(true)
        .build();

    let entry_artist = Entry::builder()
        .placeholder_text("Enter artist name")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Pick a file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let state = selected_path.clone();
    button.connect_clicked(move |button| {
        let state = state.clone();
        let dialog = filedialog.clone();
        let button = button.clone();
        let window = button.root().and_downcast::<adw::gtk::Window>();
        glib::spawn_future_local(async move {
            match dialog.open_future(window.as_ref()).await {
                Ok(file) => {
                    let Some(path) = file.path() else {
                        eprintln!("not a local file");
                        return;
                    };
                    button.set_label(&format!("Selected: {}", path.display()));
                    *state.borrow_mut() = Some(path);
                    
                }
                Err(err) => {
                    eprintln!("Error selecting file: {:?}", err);
                }
            }
        });
    });

    
    let button_save = Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    
    let state = selected_path.clone();
    let entry_for_closure = entry_artist.clone();
    button_save.connect_clicked(move |_| {
        let Some(path) = state.borrow().clone() else {
            eprintln!("no file selected");
            return;
        };

        let artist = entry_for_closure.text();
        match set_artist(&path, &artist) {
                        Ok(()) => {
                            println!("Tags updated successfully");
                        }
                        Err(err) => {
                            eprintln!("Error updating tag: {:?}", err);
                        }
                    }
    });


    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    
    content.append(&button);
    content.append(&entry_artist);
    content.append(&button_save);

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
