extern crate gio;
extern crate gtk;

mod toolbar;

use gtk::{Application, ApplicationWindow, ContainerExt, GtkWindowExt, WidgetExt};

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};

use std::env;
use toolbar::MusicToolbar;

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let toolbar = MusicToolbar::new();
        window.add(toolbar.toolbar());
        window.show_all();

        let app = App { toolbar, window };

        app.connect_events();

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
