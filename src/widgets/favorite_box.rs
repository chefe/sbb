use gtk::prelude::*;

use std::sync::Arc;

use crate::favorites::Favorites;
use crate::string_event_handler::StringEventHandler;

const STRING_TARGET_INFO: u32 = 0;
const STRING_TARGET_NAME: &'static str = "STRING";

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

        let favorite_selected = StringEventHandler::new("favorite-selected");

        let widget = Self {
            container,
            favorites,
            favorite_selected,
        };

        widget.setup_event_handlers();
        widget.update_favorites();

        widget
    }

    fn setup_event_handlers(&self) {
        let widget = self.clone();
        self.favorites.connect_changed(move || {
            widget.update_favorites();
        });
    }

    fn enable_drag_an_drop_on_button(&self, button: &gtk::Button) {
        let targets = vec![gtk::TargetEntry::new(
            STRING_TARGET_NAME,
            gtk::TargetFlags::SAME_APP,
            STRING_TARGET_INFO,
        )];

        // enable dragging
        button.drag_source_set(
            gdk::ModifierType::MODIFIER_MASK,
            &targets,
            gdk::DragAction::COPY,
        );

        // enable dropping
        button.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

        // set drag data
        button.connect_drag_data_get(|b, _, s, _, _| {
            let caption = b.get_label().unwrap().as_str().to_owned();

            s.set(
                &gdk::SELECTION_TYPE_STRING,
                STRING_TARGET_INFO as i32,
                &caption.into_bytes(),
            );
        });

        // read drag data
        let action = self.favorite_selected.clone();
        button.connect_drag_data_received(move |b, _, _, _, s, _, _| {
            let caption = b.get_label().unwrap();

            if let Some(text) = s.get_text() {
                // trigger the favorite_selected callback with the source
                action.trigger(&text.as_str());

                // trigger the favorite_selected callback with the destination
                action.trigger(caption.as_str());
            }
        });
    }

    fn update_favorites(&self) {
        self.clear();

        for favorite in self.favorites.get() {
            let button = gtk::Button::with_label(&favorite);
            self.enable_drag_an_drop_on_button(&button);

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
