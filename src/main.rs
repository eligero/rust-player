extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application, ApplicationWindow, ContainerExt, GtkWindowExt, SeparatorToolItem, ToolButton,
    Toolbar, WidgetExt,
};
use std::env;

const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
    // gio application
    let application = Application::new("com.github.eligero-rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");

    // create the window
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        let next_button = ToolButton::new_from_stock("gtk-media-next");
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        let quit_button = ToolButton::new_from_stock("gtk-quit");

        window.add(&toolbar);
        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());
        toolbar.add(&previous_button);
        toolbar.add(&play_button);
        toolbar.add(&stop_button);
        toolbar.add(&next_button);
        toolbar.add(&SeparatorToolItem::new());
        toolbar.add(&remove_button);
        toolbar.add(&SeparatorToolItem::new());
        toolbar.add(&quit_button);

        window.show_all();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
