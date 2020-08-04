use chrono::DateTime;
use gtk::prelude::*;

use crate::api::Connection;
use crate::widgets::SectionWidget;

pub struct ConnectionWidget {
    pub container: gtk::Box,
}

impl ConnectionWidget {
    pub fn new(connection: &Connection) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let label = gtk::Label::new(None);
        label.set_margin_top(10);
        label.set_margin_bottom(10);
        label.set_margin_start(5);
        label.set_margin_end(5);
        label.set_markup(&Self::get_label_text(connection));
        container.add(&label);

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

    fn get_label_text(connection: &Connection) -> String {
        format!(
            "<big><b>{} {} - {} {}</b></big>",
            Self::format_time(&connection.from.departure),
            connection.from.station.name,
            Self::format_time(&connection.to.arrival),
            connection.to.station.name
        )
    }

    fn format_time(input: &Option<String>) -> String {
        let format = "%Y-%m-%dT%H:%M:%S%z";

        let time = match input.as_ref() {
            Some(time) => time,
            None => return "".to_owned(),
        };

        match DateTime::parse_from_str(time, format) {
            Ok(rfc3339) => rfc3339.format("%H:%M").to_string(),
            Err(_) => "".to_owned(),
        }
    }
}
