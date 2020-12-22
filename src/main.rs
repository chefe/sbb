use gio::prelude::*;
use gtk::prelude::*;
use libhandy::prelude::*;

use std::sync::Arc;

use sbb::favorites::Favorites;
use sbb::widgets::*;

const APP_TITLE: &str = "SBB";
const MAIN_PAGE: &str = "main_page";
const CONNECTION_LIST_PAGE: &str = "connection_list_page";
const PAGE_WIDTH: i32 = 320;
const PAGE_HEIGHT: i32 = -1;
const WINDOW_WIDTH: i32 = 360;
const WINDOW_HEIGHT: i32 = 648;

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
    let main_header = libhandy::HeaderBarBuilder::new()
        .title(APP_TITLE)
        .hexpand(true)
        .show_close_button(true)
        .build();

    let back_button =
        gtk::Button::from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Menu);

    let connection_list_header = libhandy::HeaderBarBuilder::new()
        .show_close_button(true)
        .hexpand(true)
        .build();
    connection_list_header.add(&back_button);

    let header_group = libhandy::HeaderGroup::new();
    header_group.add_header_bar(&main_header);
    header_group.add_header_bar(&connection_list_header);

    let title_separator = gtk::Separator::new(gtk::Orientation::Vertical);
    let content_separator = gtk::Separator::new(gtk::Orientation::Vertical);

    let title_leaflet = libhandy::Leaflet::new();
    title_leaflet.add(&main_header);
    title_leaflet.set_child_name(&main_header, Some(MAIN_PAGE));
    title_leaflet.add(&title_separator);
    title_leaflet.add(&connection_list_header);
    title_leaflet.set_child_name(&connection_list_header, Some(CONNECTION_LIST_PAGE));

    let title_bar = libhandy::TitleBar::new();
    title_bar.add(&title_leaflet);

    let window = gtk::ApplicationWindow::new(app);
    window.set_title(APP_TITLE);
    window.set_default_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_titlebar(Some(&title_bar));

    let favorites = Arc::new(Favorites::new());

    let search_page = SearchWidget::new(favorites.clone());
    search_page
        .container
        .set_size_request(PAGE_WIDTH, PAGE_HEIGHT);

    let connection_list_page = ConnectionListWidget::new();
    connection_list_page
        .container
        .set_size_request(PAGE_WIDTH, PAGE_HEIGHT);

    let content_leaflet = libhandy::Leaflet::new();
    content_leaflet.add(&search_page.container);
    content_leaflet.set_child_name(&search_page.container, Some(MAIN_PAGE));
    content_leaflet.add(&content_separator);
    content_leaflet.add(&connection_list_page.container);
    content_leaflet.set_child_name(&connection_list_page.container, Some(CONNECTION_LIST_PAGE));

    let left_page_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    left_page_size_group.add_widget(&main_header);
    left_page_size_group.add_widget(&search_page.container);

    let right_page_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    right_page_size_group.add_widget(&connection_list_header);
    right_page_size_group.add_widget(&connection_list_page.container);

    let separator_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    separator_size_group.add_widget(&title_separator);
    separator_size_group.add_widget(&content_separator);

    content_leaflet
        .bind_property("visible-child-name", &title_leaflet, "visible-child-name")
        .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
        .build();

    content_leaflet
        .bind_property("folded", &back_button, "visible")
        .flags(glib::BindingFlags::SYNC_CREATE)
        .build();

    let leaflet = content_leaflet.clone();
    search_page.connect_search(move |data| match sbb::api::search_connection(data) {
        Ok(connections) => {
            connection_list_page.set_connections(connections);
            leaflet.set_visible_child_name(CONNECTION_LIST_PAGE);
        }
        Err(_) => {
            connection_list_page.set_connections(vec![]);

            let dialog = gtk::MessageDialogBuilder::new()
                .modal(true)
                .message_type(gtk::MessageType::Error)
                .title("Error")
                .text("Search failed! Please verify\nthat you are connected to\nthe internet and then retry.")
                .buttons(gtk::ButtonsType::Ok)
                .build();

            dialog.connect_response(|d, _| unsafe {
                d.destroy();
            });

            dialog.show_all();
        }
    });

    let leaflet = content_leaflet.clone();
    back_button.connect_clicked(move |_| {
        leaflet.set_visible_child_name(MAIN_PAGE);
    });

    window.add(&content_leaflet);
    window.show_all();
}
