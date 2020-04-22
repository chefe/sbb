extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::sync::Arc;
use std::thread;

use crate::api;
use crate::favorites::Favorites;
use crate::string_event_handler::StringEventHandler;

#[derive(Clone)]
pub struct LocationRowWidget {
    pub container: gtk::Box,
    entry: gtk::Entry,
    favorite_button: gtk::Button,
    clear_button: gtk::Button,
    favorites: Arc<Favorites>,
    sender: glib::Sender<Message>,
    add_favorite: StringEventHandler,
    remove_favorite: StringEventHandler,
    cleared: gio::SimpleAction,
    completion: Arc<gtk::EntryCompletion>,
}

enum Message {
    UpdateAutoCompleteList(Vec<String>),
}

impl LocationRowWidget {
    pub fn new(
        caption: &str,
        label_size_group: &gtk::SizeGroup,
        favorites: Arc<Favorites>,
    ) -> Self {
        let label_caption = format!("{}:", caption);
        let label = gtk::Label::new(Some(&label_caption));
        label.set_margin_top(5);
        label.set_margin_bottom(5);
        label.set_margin_start(5);
        label.set_margin_end(0);

        let completion = gtk::EntryCompletion::new();
        completion.set_text_column(0);
        completion.set_minimum_key_length(2);
        completion.set_popup_completion(true);

        let entry = gtk::Entry::new();
        entry.set_margin_top(5);
        entry.set_margin_bottom(5);
        entry.set_margin_start(5);
        entry.set_margin_end(5);
        entry.set_hexpand(true);
        entry.set_completion(Some(&completion));

        let favorite_button = gtk::Button::new();
        favorite_button.set_margin_top(5);
        favorite_button.set_margin_bottom(5);
        favorite_button.set_margin_start(0);
        favorite_button.set_margin_end(5);

        let clear_button = gtk::Button::new();
        clear_button.set_margin_top(5);
        clear_button.set_margin_bottom(5);
        clear_button.set_margin_start(0);
        clear_button.set_margin_end(5);
        Self::set_button_icon(&clear_button, "user-trash-symbolic");

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        container.add(&label);
        container.add(&entry);
        container.add(&favorite_button);
        container.add(&clear_button);

        label_size_group.add_widget(&label);

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let widget = Self {
            container,
            entry,
            favorite_button,
            clear_button,
            favorites,
            sender,
            completion: Arc::new(completion),
            add_favorite: StringEventHandler::new("add-favorite"),
            remove_favorite: StringEventHandler::new("remove-favorite"),
            cleared: gio::SimpleAction::new("cleared", None),
        };

        widget.setup_event_handlers();
        widget.update_favorite_button_icon();

        let parent = widget.clone();
        receiver.attach(None, move |msg| {
            match msg {
                Message::UpdateAutoCompleteList(locations) => {
                    parent.set_auto_complete_list(locations);
                }
            }

            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });

        widget
    }

    fn setup_event_handlers(&self) {
        let widget = self.clone();
        self.entry.connect_changed(move |_| {
            widget.update_favorite_button_icon();
            widget.update_completion_list();
        });

        let widget = self.clone();
        self.clear_button.connect_clicked(move |_| {
            widget.set_text("");
            widget.cleared.activate(None);
        });

        let widget = self.clone();
        self.favorite_button.connect_clicked(move |_| {
            match widget.is_current_text_in_favorites() {
                true => widget.remove_favorite.trigger(&widget.get_text()),
                false => widget.add_favorite.trigger(&widget.get_text()),
            }
        });

        let widget = self.clone();
        self.favorites.connect_changed(move || {
            widget.update_favorite_button_icon();
        });

        let widget = self.clone();
        self.connect_add_favorite(move |favorite| {
            widget.favorites.add(favorite);
        });

        let widget = self.clone();
        self.connect_remove_favorite(move |favorite| {
            widget.favorites.remove(favorite);
        });
    }

    pub fn get_text(&self) -> String {
        match self.entry.get_text() {
            Some(gstr) => gstr.to_string(),
            None => String::from(""),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.get_text().len() == 0
    }

    pub fn set_text(&self, text: &str) {
        self.entry.set_text(text);
    }

    fn is_current_text_in_favorites(&self) -> bool {
        self.favorites.contains(&self.get_text())
    }

    fn update_favorite_button_icon(&self) {
        let icon = if self.is_current_text_in_favorites() {
            "starred-symbolic"
        } else {
            "non-starred-symbolic"
        };

        Self::set_button_icon(&self.favorite_button, icon);
    }

    fn set_button_icon(button: &gtk::Button, name: &str) {
        let icon = gtk::Image::new_from_icon_name(Some(name), gtk::IconSize::Menu);
        button.set_image(Some(&icon));
    }

    fn update_completion_list(&self) {
        let text = self.get_text();
        let sender = self.sender.clone();

        thread::spawn(move || {
            if let Ok(locations) = api::search_location(&text) {
                let _ = sender.send(Message::UpdateAutoCompleteList(locations));
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

    pub fn connect_add_favorite<F>(&self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.add_favorite.connect(callback);
    }

    pub fn connect_remove_favorite<F>(&self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.remove_favorite.connect(callback);
    }

    pub fn connect_cleared<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.cleared.connect_activate(move |_, _| {
            callback();
        });
    }
}
