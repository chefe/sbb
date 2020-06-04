use chrono::DateTime;
use gtk::prelude::*;

use crate::api::Connection;
use crate::api::Section;
use crate::widgets::SectionWidget;

pub struct ConnectionWidget {
    pub container: gtk::Expander,
}

impl ConnectionWidget {
    pub fn new(connection: &Connection) -> Self {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let seperator = gtk::Separator::new(gtk::Orientation::Horizontal);
        main_box.add(&seperator);

        for section in &connection.sections {
            let widget = SectionWidget::new(section);
            main_box.add(&widget.container);

            let seperator = gtk::Separator::new(gtk::Orientation::Horizontal);
            main_box.add(&seperator);
        }

        let section = Section {
            departure: connection.from.clone(),
            arrival: connection.to.clone(),
            journey: None,
            walk: None,
        };

        let expander_label = SectionWidget::new(&section);
        let expander_seperator = gtk::Separator::new(gtk::Orientation::Horizontal);

        let expander_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        expander_box.set_hexpand(true);
        expander_box.add(&expander_label.container);
        expander_box.add(&expander_seperator);

        let container = gtk::Expander::new(Some("Demo"));
        container.set_label_fill(true);
        container.add(&main_box);

        if let Some(widget) = container.get_label_widget() {
            match widget.downcast::<gtk::Label>() {
                Ok(label) => {
                    let markup = Self::get_expander_label_text(connection);
                    label.set_markup(&markup);
                    label.set_margin_start(5);
                    label.set_margin_end(5);
                    label.set_margin_top(5);
                    label.set_margin_bottom(5);
                }
                _ => {}
            }
        }

        Self { container }
    }

    fn get_expander_label_text(connection: &Connection) -> String {
        format!(
            "<big><tt>{}</tt> {} - <tt>{}</tt> {}</big>",
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
