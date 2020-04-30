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

        for (index, connection) in connections.iter().enumerate() {
            let connection_widget = ConnectionWidget::new(&connection, index + 1);
            self.main_box.add(&connection_widget.container);
        }

        self.main_box.show_all();
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
