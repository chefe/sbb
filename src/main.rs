extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::sync::Arc;

use sbb::favorites::Favorites;
use sbb::widgets::*;

const APP_TITLE: &str = "SBB";

fn main() {
    let app = gtk::Application::new(Some("io.chefe.sbb"), Default::default())
        .expect("Initialization failed...");

    app.connect_activate(move |app| {
        build_ui(app);
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}

fn build_ui(app: &gtk::Application) {
    let window = create_application_window(app);
    let label_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    let favorites = Arc::new(Favorites::new());

    let from_entry = create_location_entry("From", &label_size_group, favorites.clone());
    let to_entry = create_location_entry("To", &label_size_group, favorites.clone());
    let button = gtk::Button::new_with_label("Submit");

    let conbox = ConnectionListWidget::new();
    let fav_box = FavoriteBoxWidget::new(favorites.clone());

    {
        let from_entry = from_entry.clone();
        let to_entry = to_entry.clone();
        fav_box.connect_selected(move |favorite| {
            if from_entry.is_empty() {
                from_entry.set_text(favorite);
            } else if to_entry.is_empty() {
                to_entry.set_text(favorite);
            }
        });
    }

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&fav_box.container);
    vbox.add(&from_entry.container);
    vbox.add(&to_entry.container);
    vbox.add(&button);
    vbox.add(&conbox.container);

    button.connect_clicked(move |_| {
        let from = from_entry.get_text();
        let to = to_entry.get_text();

        let connections = sbb::api::search_connection(&from, &to).unwrap();
        conbox.set_connections(connections);
    });

    window.add(&vbox);
    window.show_all();
}

fn create_location_entry(
    caption: &str,
    label_size_group: &gtk::SizeGroup,
    favorites: Arc<Favorites>,
) -> LocationEntryWidget {
    let entry = LocationEntryWidget::new(caption, &label_size_group, favorites.clone());

    {
        let favorites = favorites.clone();
        entry.connect_add_favorite(move |favorite| {
            favorites.add(favorite);
        });
    }

    {
        let favorites = favorites.clone();
        entry.connect_remove_favorite(move |favorite| {
            favorites.remove(favorite);
        });
    }

    entry
}

fn create_application_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_title(Some(APP_TITLE));
    header_bar.set_show_close_button(true);

    let window = gtk::ApplicationWindow::new(app);
    window.set_title(APP_TITLE);
    window.set_default_size(360, 648);
    window.set_titlebar(Some(&header_bar));

    return window;
}
