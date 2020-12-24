use chrono::offset::FixedOffset;
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
        departure_time.set_markup(&Self::format_time_with_delay(
            &section.departure.departure,
            &section.departure.delay,
        ));
        container.attach(&departure_time, 0, 0, 1, 1);

        let departure_platform = Self::create_platform_label(&section.departure.platform);
        container.attach(&departure_platform, 2, 0, 1, 1);

        let arrival_location = Self::create_label_with_default_margin();
        arrival_location.set_markup(&section.arrival.station.name);
        arrival_location.set_hexpand(true);
        container.attach(&arrival_location, 1, 2, 1, 1);

        let arrival_time = Self::create_label_with_default_margin();
        arrival_time.set_markup(&Self::format_time_with_delay(
            &section.arrival.arrival,
            &section.arrival.delay,
        ));
        container.attach(&arrival_time, 0, 2, 1, 1);

        let arrival_platform = Self::create_platform_label(&section.arrival.platform);
        container.attach(&arrival_platform, 2, 2, 1, 1);

        let journey_text = Self::get_journey_text(&section);
        if !journey_text.is_empty() {
            let journey_name = Self::create_label_with_default_margin();
            journey_name.set_markup(&journey_text);
            container.attach(&journey_name, 1, 1, 1, 1);
        }

        Self { container }
    }

    fn get_journey_text(section: &Section) -> String {
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

        if platform.ends_with("!") {
            let mut platform = platform.to_owned();
            platform.pop();
            label.set_markup(&format!("<span foreground=\"red\">Pl. {}</span>", platform));
        }

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

    fn format_time_with_delay(time: &Option<String>, delay: &Option<u16>) -> String {
        let time = match Self::parse_time(time) {
            Some(date) => date.format("%H:%M").to_string(),
            None => return "".to_owned(),
        };

        let delay: u16 = match delay {
            Some(val) => *val,
            None => 0,
        };

        let color = match delay {
            0 => "gray",
            _ => "red",
        };

        return format!("{} <span foreground=\"{}\">+{}</span>", time, color, delay);
    }

    fn parse_time(input: &Option<String>) -> Option<DateTime<FixedOffset>> {
        let format = "%Y-%m-%dT%H:%M:%S%z";

        let time = match input.as_ref() {
            Some(time) => time,
            None => return None,
        };

        match DateTime::parse_from_str(time, format) {
            Ok(rfc3339) => Some(rfc3339),
            Err(_) => None,
        }
    }
}
