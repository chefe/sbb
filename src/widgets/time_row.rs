use chrono::prelude::*;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

use crate::widgets::date_time_picker_popover::DateTimePickerPopover;

#[derive(Clone)]
pub struct TimeRowWidget {
    pub container: gtk::Box,
    time_button: gtk::MenuButton,
    arrival_button: gtk::Button,
    label: gtk::Label,
    time_picker: DateTimePickerPopover,
    is_arrival_time: Arc<Mutex<bool>>,
}

impl TimeRowWidget {
    pub fn new(label_size_group: &gtk::SizeGroup) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label = gtk::LabelBuilder::new().label("Time:").margin(5).build();
        label_size_group.add_widget(&label);

        let arrival_button = gtk::ButtonBuilder::new().margin(5).margin_start(0).build();

        let time_button = gtk::MenuButtonBuilder::new()
            .hexpand(true)
            .margin(5)
            .build();

        let time_picker = DateTimePickerPopover::new(&time_button);

        time_button.set_popover(Some(time_picker.get_popover()));

        container.add(&label);
        container.add(&time_button);
        container.add(&arrival_button);

        let widget = Self {
            container,
            time_button,
            arrival_button,
            label,
            time_picker,
            is_arrival_time: Arc::new(Mutex::new(false)),
        };

        widget.setup_event_handlers();
        widget.update_time_button_label();
        widget.update_arrival_button_icon();

        widget
    }

    pub fn get_date(&self) -> Option<String> {
        match self.time_picker.get_date_time() {
            Some(t) => {
                if t.date() == Local::today() {
                    return None;
                }

                Some(t.format("%Y-%m-%d").to_string())
            }
            None => None,
        }
    }

    pub fn get_time(&self) -> Option<String> {
        match self.time_picker.get_date_time() {
            Some(t) => Some(t.format("%H:%M").to_string()),
            None => None,
        }
    }

    pub fn is_arrival_time(&self) -> bool {
        *self.is_arrival_time.lock().unwrap()
    }

    fn setup_event_handlers(&self) {
        let widget = self.clone();
        self.time_button.connect_clicked(move |_| {
            widget.time_picker.popup();
        });

        let widget = self.clone();
        self.arrival_button.connect_clicked(move |_| {
            {
                let mut is_arrival_time = widget.is_arrival_time.lock().unwrap();
                *is_arrival_time = !(*is_arrival_time);
            }

            widget.update_arrival_button_icon();
            widget.update_time_button_label();
        });

        let widget = self.clone();
        self.time_picker.connect_changed(move || {
            widget.update_time_button_label();
        });
    }

    fn update_time_button_label(&self) {
        let arrival_text = match self.is_arrival_time() {
            true => "Arrival",
            false => "Departure",
        };

        let entry_text = match self.time_picker.get_date_time() {
            Some(t) => {
                let time = t.format("%Y-%m-%d %H:%M").to_string();
                format!("{} at {}", arrival_text, time)
            }
            None => format!("{} now", arrival_text),
        };

        self.time_button.set_label(&entry_text);
    }

    fn update_arrival_button_icon(&self) {
        let icon = if self.is_arrival_time() {
            "orientation-portrait-left"
        } else {
            "orientation-portrait-right"
        };

        let icon = gtk::Image::from_icon_name(Some(icon), gtk::IconSize::Menu);
        self.arrival_button.set_image(Some(&icon));
    }
}
