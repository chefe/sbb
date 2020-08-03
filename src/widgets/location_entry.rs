use gtk::prelude::*;

use std::thread;

use crate::api;

enum LocationEntryMessage {
    UpdateAutoCompleteList(Vec<String>),
}

#[derive(Clone)]
pub struct LocationEntry {
    pub container: gtk::Entry,
    sender: glib::Sender<LocationEntryMessage>,
    completion: gtk::EntryCompletion,
}

impl LocationEntry {
    pub fn new() -> Self {
        let completion = gtk::EntryCompletion::new();
        completion.set_text_column(0);
        completion.set_minimum_key_length(2);
        completion.set_popup_completion(true);

        let container = gtk::Entry::new();
        container.set_margin_top(5);
        container.set_margin_bottom(5);
        container.set_margin_start(5);
        container.set_margin_end(5);
        container.set_hexpand(true);
        container.set_completion(Some(&completion));

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let widget = Self {
            container,
            sender,
            completion,
        };

        widget.setup_event_handlers(receiver);

        widget
    }

    fn setup_event_handlers(&self, receiver: glib::Receiver<LocationEntryMessage>) {
        let parent = self.clone();
        receiver.attach(None, move |msg| {
            match msg {
                LocationEntryMessage::UpdateAutoCompleteList(locations) => {
                    parent.set_auto_complete_list(locations);
                }
            }

            // Returning false here would close the receiver and have senders fail
            glib::Continue(true)
        });

        let parent = self.clone();
        self.connect_changed(move || {
            parent.update_completion_list();
        });
    }

    fn update_completion_list(&self) {
        let text = self.get_text();
        let sender = self.sender.clone();

        thread::spawn(move || {
            if let Ok(locations) = api::search_location(&text) {
                let _ = sender.send(LocationEntryMessage::UpdateAutoCompleteList(locations));
            }
        });
    }

    fn set_auto_complete_list(&self, locations: Vec<String>) {
        let store = gtk::ListStore::new(&[String::static_type()]);
        let col_indices: [u32; 1] = [0];

        for location in locations.iter() {
            let values: [&dyn ToValue; 1] = [&location];
            store.set(&store.append(), &col_indices, &values);
        }

        self.completion.set_model(Some(&store));
    }

    pub fn connect_changed<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.container.connect_changed(move |_| {
            callback();
        });
    }

    pub fn get_text(&self) -> String {
        self.container.get_text().to_string()
    }

    pub fn set_text(&self, text: &str) {
        self.container.set_text(text);
    }
}
