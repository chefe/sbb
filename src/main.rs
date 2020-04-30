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
    let favorites = Arc::new(Favorites::new());

    let search = SearchWidget::new(favorites.clone());

    let conbox = ConnectionListWidget::new();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&search.container);
    vbox.add(&conbox.container);

    search.connect_search(move |data| {
        let connections = sbb::api::search_connection(data).unwrap();
        conbox.set_connections(connections);
    });

    window.add(&vbox);
    window.show_all();
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
