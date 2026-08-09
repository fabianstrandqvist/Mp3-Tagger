# Grafitti — getting started

A libadwaita/Rust app for tagging MP3 files with cover art.

Environment as of 2026-08-04: Fedora 43, GNOME 49, gtk4-devel 4.20.4 installed.
Missing: `libadwaita-devel`, Rust toolchain.

## 1. Install what's missing

```bash
sudo dnf install libadwaita-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Use rustup rather than `dnf install rust` — worth controlling the toolchain, since
gtk-rs moves fast. Restart the shell afterward.

## 2. Crate choices

**`libadwaita`** — the Rust bindings. Crate is named `libadwaita`, but conventionally
imported as `adw`. Set the rename in `Cargo.toml`:

```toml
[dependencies]
gtk = { package = "gtk4", version = "0.10" }
adw = { package = "libadwaita", version = "0.8", features = ["v1_7"] }
```

Let `cargo add` pick the actual versions rather than trusting the ones above. The
`features` flag gates which libadwaita API level is callable — check with
`pkg-config --modversion libadwaita-1` after installing and enable the matching
`v1_x`. Skipping it pins you to 1.0-era API and half the widgets appear not to exist.

**`lofty`** for tags. Handles ID3v2 `APIC` frames (embedded cover art) properly,
including picture-type and MIME metadata. Alternative is the `id3` crate — MP3-only,
slightly simpler API. Lofty gets you FLAC/M4A for free later.

**Nothing for images.** GTK reads JPEG/PNG bytes natively via
`gdk::Texture::from_bytes()`. Only pull in the `image` crate if resizing or
re-encoding becomes necessary.

## 3. Get a window on screen first

Before any tagging logic, prove the toolchain works:

```rust
use adw::prelude::*;

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("io.github.fabianstrandqvist.Grafitti")
        .build();

    app.connect_activate(|app| {
        let header = adw::HeaderBar::new();
        let content = adw::ToolbarView::new();
        content.add_top_bar(&header);
        content.set_content(Some(&adw::StatusPage::builder()
            .icon_name("audio-x-generic-symbolic")
            .title("Drop an MP3 here")
            .build()));

        adw::ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .content(&content)
            .build()
            .present();
    });

    app.run()
}
```

`cargo run`. A proper GNOME window with a flat headerbar means linking is correct.

`application_id` must be a valid reverse-DNS string — GTK refuses to start otherwise,
with an unhelpful error message.

## 4. Build order

1. **Open a file.** `gtk::FileDialog` (GTK 4.10+) with a `gtk::FileFilter` on
   `audio/mpeg`. It's async — returns a future, handled with
   `glib::spawn_future_local`. This async-inside-a-GUI-callback pattern is the first
   genuinely unfamiliar piece.
2. **Read existing tags.** `lofty::read_from_path`, then the primary tag. Lofty needs
   trait imports (`Accessor`, `TaggedFileExt`) for getters to be visible — "method not
   found" on obviously-correct code is almost always a missing trait import.
3. **Display the cover.** Picture bytes from the tag → `glib::Bytes` →
   `gdk::Texture::from_bytes()` → `gtk::Picture`.
4. **Set a new cover.** Second file dialog for the image; build a
   `lofty::picture::Picture` with `PictureType::CoverFront` and correct MIME type,
   attach to the tag, `save_to_path`.
5. **Then** drag-and-drop (`gtk::DropTarget` accepting `gdk::FileList`), and editable
   title/artist/album fields.

Step 3 is where it stops being boilerplate.

## 5. Making it look native

The libadwaita widgets do the work if used as intended:

- `AdwToolbarView` — window frame
- `AdwPreferencesGroup` + `AdwEntryRow` — tag fields (the boxed-list look from GNOME Settings)
- `AdwToast` — "Saved" confirmations
- `AdwStatusPage` — empty state

Resist custom CSS. Reaching for it usually means the wrong widget.

## References

- [gtk-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/) — Rust-specific patterns,
  especially the `GObject` subclassing chapter (needed once state gets non-trivial)
- [libadwaita widget gallery](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/) —
  check what exists before building it yourself
