use gtk::prelude::*;

use std::sync::Arc;

use crate::api::SearchConnectionRequest;
use crate::favorites::Favorites;
use crate::widgets::*;

#[derive(Clone)]
pub struct SearchWidget {
    pub container: gtk::Box,
    button: gtk::Button,
    from_entry: LocationRowWidget,
    to_entry: LocationRowWidget,
    via_box: ViaBoxWidget,
    time_input: TimeRowWidget,
}

impl SearchWidget {
    pub fn new(favorites: Arc<Favorites>) -> Self {
        let label_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

        let from_entry = LocationRowWidget::new("From", &label_size_group, favorites.clone());
        let to_entry = LocationRowWidget::new("To", &label_size_group, favorites.clone());

        let button = gtk::Button::new_with_label("Submit");
        button.set_margin_top(5);
        button.set_margin_bottom(5);
        button.set_margin_start(5);
        button.set_margin_end(5);

        let fav_box = FavoriteBoxWidget::new(favorites.clone());
        let via_box = ViaBoxWidget::new(&label_size_group, favorites.clone());

        let time_input = TimeRowWidget::new(&label_size_group);

        {
            let from_entry = from_entry.clone();
            let to_entry = to_entry.clone();
            fav_box.connect_selected(move |favorite| {
                if from_entry.is_empty() {
                    from_entry.set_text(favorite);
                } else if to_entry.is_empty() {
                    to_entry.set_text(favorite);
                }
            });
        }

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.add(&fav_box.container);
        container.add(&from_entry.container);
        container.add(&to_entry.container);
        container.add(&via_box.container);
        container.add(&time_input.container);
        container.add(&button);

        Self {
            container,
            button,
            from_entry,
            to_entry,
            via_box,
            time_input,
        }
    }

    pub fn connect_search<F>(&self, callback: F)
    where
        F: Fn(SearchConnectionRequest) + 'static,
    {
        let parent = self.clone();

        self.button.connect_clicked(move |_| {
            let data = SearchConnectionRequest {
                from: parent.from_entry.get_text(),
                to: parent.to_entry.get_text(),
                vias: parent.via_box.get_vias(),
                date: parent.time_input.get_date(),
                time: parent.time_input.get_time(),
                is_arrival_time: parent.time_input.is_arrival_time(),
            };

            callback(data);
        });
    }
}
