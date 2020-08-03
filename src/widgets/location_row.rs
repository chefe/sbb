use gio::prelude::*;
use gtk::prelude::*;

use std::sync::Arc;

use crate::favorites::Favorites;
use crate::string_event_handler::StringEventHandler;
use crate::widgets::LocationEntry;

#[derive(Clone)]
pub struct LocationRowWidget {
    pub container: gtk::Box,
    entry: LocationEntry,
    favorite_button: gtk::Button,
    clear_button: gtk::Button,
    favorites: Arc<Favorites>,
    add_favorite: StringEventHandler,
    remove_favorite: StringEventHandler,
    cleared: gio::SimpleAction,
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

        let entry = LocationEntry::new();

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
        container.add(&entry.container);
        container.add(&favorite_button);
        container.add(&clear_button);

        label_size_group.add_widget(&label);

        let widget = Self {
            container,
            entry,
            favorite_button,
            clear_button,
            favorites,
            add_favorite: StringEventHandler::new("add-favorite"),
            remove_favorite: StringEventHandler::new("remove-favorite"),
            cleared: gio::SimpleAction::new("cleared", None),
        };

        widget.setup_event_handlers();
        widget.update_favorite_button_icon();

        widget
    }

    fn setup_event_handlers(&self) {
        let widget = self.clone();
        self.entry.connect_changed(move || {
            widget.update_favorite_button_icon();
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
        self.entry.get_text()
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
        let icon = gtk::Image::from_icon_name(Some(name), gtk::IconSize::Menu);
        button.set_image(Some(&icon));
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

    pub fn connect_changed<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.entry.connect_changed(move || {
            callback();
        });
    }
}
