extern crate chrono;
extern crate gtk;

use chrono::prelude::*;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TimePickerWidget {
    pub container: gtk::Box,
    button: gtk::Button,
    entry: gtk::Entry,
    label: gtk::Label,
    popover: gtk::Popover,
    now_button: gtk::Button,
    tonight_button: gtk::Button,
    tomorrow_morning_button: gtk::Button,
    tomorrow_evening_button: gtk::Button,
    minute_input: gtk::SpinButton,
    hour_input: gtk::SpinButton,
    day_input: gtk::SpinButton,
    month_input: gtk::SpinButton,
    year_input: gtk::SpinButton,
    time: Arc<Mutex<Option<DateTime<Local>>>>,
    is_arrival_time: Arc<Mutex<bool>>,
}

impl TimePickerWidget {
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

        let hour_input = Self::create_spin_button(0, 23);
        let minute_input = Self::create_spin_button(0, 59);
        let day_input = Self::create_spin_button(1, 31);
        let month_input = Self::create_spin_button(1, 12);
        let year_input = Self::create_spin_button(2000, 2200);

        let (date_frame, date_box) = Self::create_frame("Date");
        date_box.add(&year_input);
        date_box.add(&Self::create_label("-"));
        date_box.add(&month_input);
        date_box.add(&Self::create_label("-"));
        date_box.add(&day_input);

        let (time_frame, time_box) = Self::create_frame("Time");
        time_box.add(&hour_input);
        time_box.add(&Self::create_label(":"));
        time_box.add(&minute_input);

        let custom_time_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        custom_time_box.add(&date_frame);
        custom_time_box.add(&time_frame);

        let now_button = Self::create_button("Now");
        let tonight_button = Self::create_button("Tonight 19:00");
        let tomorrow_morning_button = Self::create_button("Tomorrow morning 09:00");
        let tomorrow_evening_button = Self::create_button("Tomorrow evening 19:00");

        let popover_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        popover_box.add(&now_button);
        popover_box.add(&tonight_button);
        popover_box.add(&tomorrow_morning_button);
        popover_box.add(&tomorrow_evening_button);
        popover_box.add(&custom_time_box);

        let popover = gtk::Popover::new(Some(&entry));
        popover.set_position(gtk::PositionType::Bottom);
        popover.add(&popover_box);

        container.add(&label);
        container.add(&entry);
        container.add(&button);

        let widget = Self {
            container,
            button,
            entry,
            label,
            popover,
            now_button,
            tonight_button,
            tomorrow_morning_button,
            tomorrow_evening_button,
            minute_input,
            hour_input,
            day_input,
            month_input,
            year_input,
            time: Arc::new(Mutex::new(None)),
            is_arrival_time: Arc::new(Mutex::new(false)),
        };

        widget.setup_event_handlers();
        widget.update_entry_text();
        widget.update_button_icon();

        widget
    }

    pub fn get_date(&self) -> Option<String> {
        match *self.time.lock().unwrap() {
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
        match *self.time.lock().unwrap() {
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
            widget.popover.show_all();
            widget.popover.popup();

            let time = widget.time.lock().unwrap();
            widget.set_inputs_to_time(*time);

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
        self.now_button.connect_clicked(move |_| {
            widget.set_inputs_to_time(None);
            widget.set_time(None);
            widget.popover.popdown();
        });

        let widget = self.clone();
        self.tonight_button.connect_clicked(move |_| {
            let time = Local::today().and_hms(19, 0, 0);
            widget.set_inputs_to_time(Some(time));
            widget.set_time(Some(time));
            widget.popover.popdown();
        });

        let widget = self.clone();
        self.tomorrow_morning_button.connect_clicked(move |_| {
            let tomorrow = Local::today() + chrono::Duration::days(1);
            let time = tomorrow.and_hms(9, 0, 0);
            widget.set_inputs_to_time(Some(time));
            widget.set_time(Some(time));
            widget.popover.popdown();
        });

        let widget = self.clone();
        self.tomorrow_evening_button.connect_clicked(move |_| {
            let tomorrow = Local::today() + chrono::Duration::days(1);
            let time = tomorrow.and_hms(19, 0, 0);
            widget.set_inputs_to_time(Some(time));
            widget.set_time(Some(time));
            widget.popover.popdown();
        });

        let widget = self.clone();
        self.minute_input.connect_value_changed(move |_| {
            widget.store_time();
        });

        let widget = self.clone();
        self.hour_input.connect_value_changed(move |_| {
            widget.store_time();
        });

        let widget = self.clone();
        self.day_input.connect_value_changed(move |_| {
            widget.store_time();
        });

        let widget = self.clone();
        self.month_input.connect_value_changed(move |_| {
            widget.store_time();
        });

        let widget = self.clone();
        self.year_input.connect_value_changed(move |_| {
            widget.store_time();
        });
    }

    fn store_time(&self) {
        let year = self.year_input.get_value() as i32;
        let month = self.month_input.get_value() as u32;
        let day = self.day_input.get_value() as u32;
        let hour = self.hour_input.get_value() as u32;
        let minute = self.minute_input.get_value() as u32;

        let days_in_month = Self::get_days_in_month(year, month);
        if day > days_in_month {
            // fix invalid date and do not call set_time
            self.day_input.set_value(days_in_month as f64);
        } else {
            let time = Local.ymd(year, month, day).and_hms(hour, minute, 0);
            self.set_time(Some(time));
        }
    }

    fn set_time(&self, time: Option<DateTime<Local>>) {
        let mut trigger_button_update = false;

        // the set_time method can be triggerd from itselfe,
        // threfore if we can not obtain the lock we are already
        // in the method and then we don't set the time
        if let Ok(mut mutable_time) = self.time.try_lock() {
            *mutable_time = time;
            trigger_button_update = true;
        }

        // this is required because the lock needs to be released
        // before the update_entry_text method is called
        if trigger_button_update {
            self.update_entry_text();
        }
    }

    fn update_entry_text(&self) {
        let arrival_text = match self.is_arrival_time() {
            true => "Arrival",
            false => "Departure",
        };

        let entry_text = match *self.time.lock().unwrap() {
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

    fn set_inputs_to_time(&self, time: Option<DateTime<Local>>) {
        let time = match time {
            Some(t) => t,
            None => Local::now(),
        };

        Self::set_if_different(&self.minute_input, time.minute() as f64);
        Self::set_if_different(&self.hour_input, time.hour() as f64);
        Self::set_if_different(&self.day_input, time.day() as f64);
        Self::set_if_different(&self.month_input, time.month() as f64);
        Self::set_if_different(&self.year_input, time.year() as f64);
    }

    fn set_if_different(button: &gtk::SpinButton, value: f64) {
        if button.get_value() != value {
            button.set_value(value);
        }
    }

    fn create_spin_button(min: u32, max: u32) -> gtk::SpinButton {
        let entry = gtk::SpinButton::new_with_range(min as f64, max as f64, 1.0);
        entry.set_orientation(gtk::Orientation::Vertical);
        entry.set_wrap(true);
        entry
    }

    fn create_label(caption: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(caption));
        Self::set_default_margin(label.clone());
        label
    }

    fn create_button(caption: &str) -> gtk::Button {
        let button = gtk::Button::new_with_label(caption);
        Self::set_default_margin(button.clone());
        button
    }

    fn create_frame(caption: &str) -> (gtk::Frame, gtk::Box) {
        let frame_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        Self::set_default_margin(frame_box.clone());

        let frame = gtk::Frame::new(Some(caption));
        Self::set_default_margin(frame.clone());
        frame.add(&frame_box);

        (frame, frame_box)
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

    fn get_days_in_month(year: i32, month: u32) -> u32 {
        let first_of_this_month = NaiveDate::from_ymd(year, month, 1);
        let first_of_next_month = match month {
            12 => NaiveDate::from_ymd(year + 1, 1, 1),
            _ => NaiveDate::from_ymd(year, month + 1, 1),
        };

        first_of_next_month
            .signed_duration_since(first_of_this_month)
            .num_days() as u32
    }
}
