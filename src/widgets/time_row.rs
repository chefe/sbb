use chrono::prelude::*;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

use crate::widgets::date_time_picker_popover::DateTimePickerPopover;

#[derive(Clone)]
pub struct TimeRowWidget {
    pub container: gtk::Box,
    button: gtk::Button,
    entry: gtk::Entry,
    label: gtk::Label,
    time_picker: DateTimePickerPopover,
    is_arrival_time: Arc<Mutex<bool>>,
}

impl TimeRowWidget {
    pub fn new(label_size_group: &gtk::SizeGroup) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label = gtk::Label::new(Some("Time:"));
        Self::set_default_margin(label.clone());
        label_size_group.add_widget(&label);

        let entry = gtk::Entry::new();
        Self::set_default_margin(entry.clone());
        entry.set_editable(false);
        entry.set_hexpand(true);

        let button = gtk::Button::new();
        Self::set_default_margin(button.clone());
        button.set_margin_start(0);

        let time_picker = DateTimePickerPopover::new(&entry);

        container.add(&label);
        container.add(&entry);
        container.add(&button);

        let widget = Self {
            container,
            button,
            entry,
            label,
            time_picker,
            is_arrival_time: Arc::new(Mutex::new(false)),
        };

        widget.setup_event_handlers();
        widget.update_entry_text();
        widget.update_button_icon();

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
        self.entry.connect_focus_in_event(move |_, _| {
            widget.time_picker.popup();
            gtk::Inhibit(true)
        });

        let widget = self.clone();
        self.button.connect_clicked(move |_| {
            {
                let mut is_arrival_time = widget.is_arrival_time.lock().unwrap();
                *is_arrival_time = !(*is_arrival_time);
            }

            widget.update_button_icon();
            widget.update_entry_text();
        });

        let widget = self.clone();
        self.time_picker.connect_changed(move || {
            widget.update_entry_text();
        });
    }

    fn update_entry_text(&self) {
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

        self.entry.set_text(&entry_text);
    }

    fn update_button_icon(&self) {
        let icon = if self.is_arrival_time() {
            "orientation-portrait-left"
        } else {
            "orientation-portrait-right"
        };

        let icon = gtk::Image::new_from_icon_name(Some(icon), gtk::IconSize::Menu);
        self.button.set_image(Some(&icon));
    }

    fn set_default_margin<W>(widget: W)
    where
        W: IsA<gtk::Widget>,
    {
        widget.set_margin_top(5);
        widget.set_margin_bottom(5);
        widget.set_margin_start(5);
        widget.set_margin_end(5);
    }
}
