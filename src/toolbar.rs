use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        let next_button = ToolButton::new_from_stock("gtk-media-next");
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        let quit_button = ToolButton::new_from_stock("gtk-quit");

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

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}
