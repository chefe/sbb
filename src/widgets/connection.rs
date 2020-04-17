extern crate gtk;

use gtk::prelude::*;

use crate::api::Connection;
use crate::widgets::SectionWidget;

pub struct ConnectionWidget {
    pub container: gtk::Box,
}

impl ConnectionWidget {
    pub fn new(connection: &Connection, number: usize) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let connection_label = gtk::Label::new(None);
        connection_label.set_margin_top(5);
        connection_label.set_margin_bottom(5);
        connection_label.set_margin_start(5);
        connection_label.set_margin_end(5);
        connection_label.set_markup(&format!("<big>Connection {}</big>", number));
        container.add(&connection_label);

        let seperator = gtk::Separator::new(gtk::Orientation::Horizontal);
        container.add(&seperator);

        for section in &connection.sections {
            let widget = SectionWidget::new(section);
            container.add(&widget.container);

            let seperator = gtk::Separator::new(gtk::Orientation::Horizontal);
            container.add(&seperator);
        }

        Self { container }
    }
}
