use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DateTimePickerPopover {
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
    changed: gio::SimpleAction,
}

impl DateTimePickerPopover {
    pub fn new(entry: &gtk::Entry) -> Self {
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

        let popover = gtk::Popover::new(Some(entry));
        popover.set_position(gtk::PositionType::Bottom);
        popover.add(&popover_box);

        let widget = Self {
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
            changed: gio::SimpleAction::new("changed", None),
        };

        widget.setup_event_handlers();

        widget
    }

    pub fn get_date_time(&self) -> Option<DateTime<Local>> {
        *self.time.lock().unwrap()
    }

    pub fn popup(&self) {
        self.popover.show_all();
        self.popover.popup();

        let time = self.time.lock().unwrap();
        self.set_inputs_to_time(*time);
    }

    pub fn connect_changed<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.changed.connect_activate(move |_, _| {
            callback();
        });
    }

    fn setup_event_handlers(&self) {
        let widget = self.clone();
        self.now_button.connect_clicked(move |_| {
            widget.set_time_and_popdown(None);
        });

        let widget = self.clone();
        self.tonight_button.connect_clicked(move |_| {
            let time = Local::today().and_hms(19, 0, 0);
            widget.set_time_and_popdown(Some(time));
        });

        let widget = self.clone();
        self.tomorrow_morning_button.connect_clicked(move |_| {
            let tomorrow = Local::today() + chrono::Duration::days(1);
            let time = tomorrow.and_hms(9, 0, 0);
            widget.set_time_and_popdown(Some(time));
        });

        let widget = self.clone();
        self.tomorrow_evening_button.connect_clicked(move |_| {
            let tomorrow = Local::today() + chrono::Duration::days(1);
            let time = tomorrow.and_hms(19, 0, 0);
            widget.set_time_and_popdown(Some(time));
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

    fn set_time_and_popdown(&self, time: Option<DateTime<Local>>) {
        self.set_inputs_to_time(time);
        self.set_time(time);
        self.popover.popdown();
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
            self.changed.activate(None);
        }
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
