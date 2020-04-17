extern crate gtk;

use gtk::prelude::*;

use std::sync::Arc;

use super::super::favorites::Favorites;
use super::super::string_event_handler::StringEventHandler;

#[derive(Clone)]
pub struct FavoriteBoxWidget {
    pub container: gtk::FlowBox,
    favorite_selected: StringEventHandler,
    favorites: Arc<Favorites>,
}

impl FavoriteBoxWidget {
    pub fn new(favorites: Arc<Favorites>) -> Self {
        let container = gtk::FlowBox::new();
        container.set_selection_mode(gtk::SelectionMode::None);
        container.set_hexpand(true);

        let widget = Self {
            container,
            favorites,
            favorite_selected: StringEventHandler::new("favorite-selected"),
        };

        {
            let parent = widget.clone();
            widget.favorites.connect_changed(move || {
                parent.update_favorites();
            });
        }

        widget.update_favorites();

        widget
    }

    fn update_favorites(&self) {
        self.clear();

        for favorite in self.favorites.get() {
            let button = gtk::Button::new_with_label(&favorite);
            self.container.add(&button);

            let action = self.favorite_selected.clone();
            button.connect_clicked(move |_| {
                action.trigger(&favorite);
            });
        }

        self.container.show_all();
    }

    fn clear(&self) {
        self.container.foreach(|child| {
            self.container.remove(child);
        });
    }

    pub fn connect_selected<F>(&self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.favorite_selected.connect(callback);
    }
}
