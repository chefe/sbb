use gtk::prelude::*;

use crate::api::Connection;
use crate::widgets::ConnectionWidget;

pub struct ConnectionListWidget {
    pub container: gtk::ScrolledWindow,
    main_box: gtk::Box,
}

impl ConnectionListWidget {
    pub fn new() -> Self {
        let container = Self::create_scrolled_window();
        container.set_vexpand(true);
        container.set_hexpand(true);

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.add(&main_box);

        Self {
            container,
            main_box,
        }
    }

    pub fn set_connections(&self, connections: Vec<Connection>) {
        self.clear();

        for connection in connections.iter() {
            let connection_widget = ConnectionWidget::new(&connection);
            self.main_box.add(&connection_widget.container);

            let main_box = self.main_box.clone();
            connection_widget
                .container
                .connect_property_expanded_notify(move |widget| {
                    if widget.get_expanded() {
                        Self::collapse_all_except(&main_box, widget);
                    }
                });
        }

        self.main_box.show_all();
    }

    fn collapse_all_except(main_box: &gtk::Box, widget: &gtk::Expander) {
        for child in main_box.get_children() {
            match child.downcast::<gtk::Expander>() {
                Ok(expander) => {
                    if widget.get_label_widget() != expander.get_label_widget() {
                        expander.set_expanded(false);
                    }
                }
                _ => {}
            }
        }
    }

    fn clear(&self) {
        self.main_box.foreach(|child| {
            self.main_box.remove(child);
        });
    }

    fn create_scrolled_window() -> gtk::ScrolledWindow {
        let hadjust: Option<&gtk::Adjustment> = None;
        let vadjust: Option<&gtk::Adjustment> = None;
        gtk::ScrolledWindow::new(hadjust, vadjust)
    }
}
