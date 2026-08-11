use adw::prelude::*;
use id3::{Tag, TagLike, Version, ErrorKind, Frame};
use id3::frame::{Picture, PictureType};
use std::path::Path;
use std::{cell::RefCell, rc::Rc, path::PathBuf};
use adw::glib;
use adw::gio;
use adw::gtk::{Box, ListBox, Orientation, SelectionMode, Button, FileDialog, FileFilter};
use adw::{ActionRow, Application, ApplicationWindow, EntryRow, HeaderBar, PreferencesGroup};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn save_tags(
    path_music: &Path,
    path_img: Option<&Path>,      // optional — see below
    artist: &str,
    title: &str,
) -> Result<(), id3::Error> {
    let mut tag = match Tag::read_from_path(path_music) {
        Ok(tag) => tag,
        Err(e) if matches!(e.kind, ErrorKind::NoTag) => Tag::new(),
        Err(e) => return Err(e),
    };

    tag.set_artist(artist);
    tag.set_title(title);

    if let Some(img) = path_img {
        let mime = match img.extension().and_then(|e| e.to_str()) {
            Some("png") => "image/png",
            Some("jpg" | "jpeg") => "image/jpeg",
            _ => "application/octet-stream",   // decide what you want here
        };
        tag.add_frame(Picture {
            mime_type: mime.to_string(),
            picture_type: PictureType::CoverFront,
            description: String::new(),
            data: std::fs::read(img)?,
        });
    }

    tag.write_to_path(path_music, Version::Id3v24)?;
    Ok(())
}

fn build_ui(app: &Application) {
    let selected_path_music: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let selected_path_img: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

    let mp3 = FileFilter::new();
    mp3.set_name(Some("MP3 files"));
    mp3.add_mime_type("audio/mpeg");
    mp3.add_suffix("mp3");

    let filters = gio::ListStore::new::<FileFilter>();
    filters.append(&mp3);

    let img = FileFilter::new();
    img.set_name(Some("Image files"));
    img.add_mime_type("image/png");
    img.add_mime_type("image/jpeg");
    img.add_suffix("png");
    img.add_suffix("jpg");

    let filters_img = gio::ListStore::new::<FileFilter>();
    filters_img.append(&img);

    let filedialog_music = FileDialog::builder()
        .title("Select a music file")
        .accept_label("Open")
        .filters(&filters)
        .modal(true)
        .build();

    let filedialog_img = FileDialog::builder()
        .title("Select an image")
        .accept_label("Open")
        .filters(&filters_img)
        .modal(true)
        .build();

    let entry_artist = EntryRow::builder().title("Artist").build();

    let entry_title = EntryRow::builder().title("Title").build();

    let tag_group = PreferencesGroup::builder()
        .title("Tags")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    tag_group.add(&entry_artist);
    tag_group.add(&entry_title);

    let button_pick_music = Button::builder()
        .label("Pick a music file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // closure for selecting music file - refactor later to avoid code duplication
    let state = selected_path_music.clone();
    button_pick_music.connect_clicked(move |button| {
        let state = state.clone();
        let dialog = filedialog_music.clone();
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

    
    let button_pick_img = Button::builder()
        .label("Pick an image")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let state = selected_path_img.clone();
    button_pick_img.connect_clicked(move |button| {
        let state = state.clone();
        let dialog = filedialog_img.clone();
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
                    eprintln!("Error selecting image: {:?}", err);
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

    let state_music = selected_path_music.clone();
    let state_img = selected_path_img.clone();
    let entry_for_closure = entry_artist.clone();
    let entry_title_for_closure = entry_title.clone();
    button_save.connect_clicked(move |_| {
        let Some(path_music) = state_music.borrow().clone() else {
            eprintln!("no music file selected");
            return;
        };

        // no need to unwrap here as it should be optional
        let path_img = state_img.borrow().clone();

        let artist = entry_for_closure.text();
        let title = entry_title_for_closure.text();
        match save_tags(&path_music, path_img.as_deref(), &artist, &title) {
            Ok(()) => {
                println!("Tags updated successfully");
            }
            Err(err) => {
                eprintln!("Error updating tags: {:?}", err);
                        }
                    };
    });


    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    
    content.append(&button_pick_music);
    content.append(&button_pick_img);
    content.append(&tag_group);
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
