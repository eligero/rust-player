extern crate gdk_pixbuf; // Show and manipulate images
extern crate gio;
extern crate gtk;
extern crate id3; // Metadata from MP3 files

mod playlist;
mod toolbar;

use gtk::{
    Adjustment, Application, ApplicationWindow, ContainerExt, GtkWindowExt, Image, ImageExt, Scale,
    ScaleExt, WidgetExt,
};

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::Orientation::{Horizontal, Vertical};
use std::env;

use playlist::Playlist;
use toolbar::MusicToolbar;

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
    cover: Image,
    adjustment: Adjustment,
    playlist: Playlist,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        // Box container, disposes widgets either horizontally or vertically
        let vbox = gtk::Box::new(Vertical, 0); // 0 separation between the children widgets of the Box container
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let playlist = Playlist::new();
        vbox.add(playlist.view());

        let cover = Image::new();
        cover.set_from_file("cover.jpg"); // file in project directory
        vbox.add(&cover);

        // cursor widget. For now, hardcoded values for adjustment
        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false); // disable show actual value
        vbox.add(&scale);

        window.show_all();

        let app = App {
            toolbar,
            window,
            cover,
            adjustment,
            playlist,
        };

        app.connect_events();
        app.connect_toolbar_events();

        app
    }

    fn connect_events(&self) {}
}

fn main() {
    // gio application
    let application = Application::new("com.github.eligero-rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");

    // create the window
    application.connect_startup(|application| {
        let _app = App::new(application.clone());
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
