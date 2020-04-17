extern crate gtk;

use gtk::prelude::*;

use std::sync::Arc;

use super::super::favorites::Favorites;
use super::super::string_event_handler::StringEventHandler;

#[derive(Clone)]
pub struct LocationEntryWidget {
    pub container: gtk::Box,
    entry: gtk::Entry,
    favorite_button: gtk::Button,
    clear_button: gtk::Button,
    favorites: Arc<Favorites>,
    add_favorite: StringEventHandler,
    remove_favorite: StringEventHandler,
}

impl LocationEntryWidget {
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

        let entry = gtk::Entry::new();
        entry.set_margin_top(5);
        entry.set_margin_bottom(5);
        entry.set_margin_start(5);
        entry.set_margin_end(5);
        entry.set_hexpand(true);

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

        let widget = Self {
            container,
            entry,
            favorite_button,
            clear_button,
            favorites,
            add_favorite: StringEventHandler::new("add-favorite"),
            remove_favorite: StringEventHandler::new("remove-favorite"),
        };

        {
            let parent = widget.clone();
            widget.entry.connect_changed(move |_| {
                parent.update_favorite_button_icon();
            });

            let parent = widget.clone();
            widget.clear_button.connect_clicked(move |_| {
                parent.set_text("");
            });

            let parent = widget.clone();
            widget.favorite_button.connect_clicked(move |_| {
                match parent.is_current_text_in_favorites() {
                    true => parent.remove_favorite.trigger(&parent.get_text()),
                    false => parent.add_favorite.trigger(&parent.get_text()),
                }
            });

            let parent = widget.clone();
            widget.favorites.connect_changed(move || {
                parent.update_favorite_button_icon();
            });
        }

        widget.update_favorite_button_icon();

        widget
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
}
