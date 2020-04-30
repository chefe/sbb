use chrono::DateTime;
use gtk::prelude::*;

use crate::api::Section;

pub struct SectionWidget {
    pub container: gtk::Grid,
}

impl SectionWidget {
    pub fn new(section: &Section) -> Self {
        let container = gtk::Grid::new();
        container.set_hexpand(true);

        let departure_location = Self::create_label_with_default_margin();
        departure_location.set_markup(&section.departure.station.name);
        departure_location.set_hexpand(true);
        container.attach(&departure_location, 1, 0, 1, 1);

        let departure_time = Self::create_label_with_default_margin();
        departure_time.set_markup(&Self::get_formatted_time(&section.departure.departure));
        container.attach(&departure_time, 0, 0, 1, 1);

        let departure_platform = Self::create_platform_label(&section.departure.platform);
        container.attach(&departure_platform, 2, 0, 1, 1);

        let arrival_location = Self::create_label_with_default_margin();
        arrival_location.set_markup(&section.arrival.station.name);
        arrival_location.set_hexpand(true);
        container.attach(&arrival_location, 1, 2, 1, 1);

        let arrival_time = Self::create_label_with_default_margin();
        arrival_time.set_markup(&Self::get_formatted_time(&section.arrival.arrival));
        container.attach(&arrival_time, 0, 2, 1, 1);

        let arrival_platform = Self::create_platform_label(&section.arrival.platform);
        container.attach(&arrival_platform, 2, 2, 1, 1);

        let journey_name = Self::create_label_with_default_margin();
        journey_name.set_markup(&Self::get_journey_name(&section));
        container.attach(&journey_name, 1, 1, 1, 1);

        Self { container }
    }

    fn get_journey_name(section: &Section) -> String {
        if let Some(journey) = section.journey.as_ref() {
            return format!("<i>{}</i>", journey.name);
        }

        if let Some(walk) = section.walk.as_ref() {
            let duration: u16 = walk.duration / 60;
            return format!("<i>Walk {} min</i>", duration);
        }

        "".to_owned()
    }

    fn create_platform_label(platform: &Option<String>) -> gtk::Label {
        let platform = match platform {
            Some(text) => &text,
            None => "-",
        };

        let label = Self::create_label_with_default_margin();
        label.set_markup(&format!("Pl. {}", platform));
        label
    }

    fn create_label_with_default_margin() -> gtk::Label {
        let label = gtk::Label::new(None);
        label.set_margin_top(5);
        label.set_margin_bottom(5);
        label.set_margin_start(5);
        label.set_margin_end(5);
        label
    }

    fn get_formatted_time(input: &Option<String>) -> String {
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
