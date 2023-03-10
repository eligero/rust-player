use gtk::{
    ApplicationWindow, ContainerExt, DialogExt, FileChooserAction, FileChooserDialog,
    FileChooserExt, FileFilter, FileFilterExt, Image, ImageExt, LabelExt, SeparatorToolItem,
    ToolButton, ToolButtonExt, Toolbar, WidgetExt,
};

use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};
use std::path::PathBuf;

use playlist::Playlist;
use App;

pub const PAUSE_ICON: &str = "gtk-media-pause";
pub const PLAY_ICON: &str = "gtk-media-play";
const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    pub play_image: Image,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();

        let (open_button, _) = new_tool_button("document-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());

        let (previous_button, _) = new_tool_button("gtk-media-previous");
        toolbar.add(&previous_button);

        let (play_button, play_image) = new_tool_button(PLAY_ICON);
        toolbar.add(&play_button);

        let (stop_button, _) = new_tool_button("gtk-media-stop");
        toolbar.add(&stop_button);

        let (next_button, _) = new_tool_button("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let (remove_button, _) = new_tool_button("remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let (quit_button, _) = new_tool_button("gtk-quit");
        toolbar.add(&quit_button);

        let toolbar = MusicToolbar {
            open_button,
            next_button,
            play_button,
            play_image,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        };

        toolbar
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

impl App {
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });

        let playlist = self.playlist.clone();
        let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        let state = self.state.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if state.lock().unwrap().stopped {
                if playlist.play() {
                    set_image_icon(&play_image, PAUSE_ICON);
                    set_cover(&cover, &playlist);
                }
            } else {
                playlist.pause();
                set_image_icon(&play_image, PLAY_ICON);
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });

        let current_time_label = self.current_time_label.clone();
        let duration_label = self.duration_label.clone();
        let playlist = self.playlist.clone();
        let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.stop_button.connect_clicked(move |_| {
            current_time_label.set_text("");
            duration_label.set_text("");
            playlist.stop();
            cover.hide();
            set_image_icon(&play_image, PLAY_ICON);
        });

        let playlist = self.playlist.clone();
        let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.next_button.connect_clicked(move |_| {
            if playlist.next() {
                set_image_icon(&play_image, PAUSE_ICON);
                set_cover(&cover, &playlist);
            }
        });

        let playlist = self.playlist.clone();
        let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.previous_button.connect_clicked(move |_| {
            if playlist.previous() {
                set_image_icon(&play_image, PAUSE_ICON);
                set_cover(&cover, &playlist);
            }
        });
    }
}

fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;

    let dialog = FileChooserDialog::new(
        Some("Select an MP3 audio file"),
        Some(parent),
        FileChooserAction::Open,
    );

    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");

    dialog.add_filter(&filter);

    dialog.add_button("Cancel", RESPONSE_CANCEL);
    dialog.add_button("Accept", RESPONSE_ACCEPT);
    let result = dialog.run();
    if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}

fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    cover.show();
}

fn new_tool_button(icon: &str) -> (ToolButton, Image) {
    let image = Image::new_from_file(format!("assets/{}.png", icon));
    (ToolButton::new(&image, None), image)
}

pub fn set_image_icon(image: &Image, icon: &str) {
    image.set_from_file(format!("assets/{}.png", icon));
}
