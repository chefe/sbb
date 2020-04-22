extern crate gtk;

use gtk::prelude::*;

use std::sync::{Arc, Mutex};

use crate::favorites::Favorites;
use crate::widgets::LocationRowWidget;

#[derive(Clone)]
pub struct ViaBoxWidget {
    pub container: gtk::Box,
    label_size_group: gtk::SizeGroup,
    favorites: Arc<Favorites>,
    add_button: gtk::Button,
    vias: Arc<Mutex<Vec<LocationRowWidget>>>,
}

impl ViaBoxWidget {
    pub fn new(label_size_group: &gtk::SizeGroup, favorites: Arc<Favorites>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let add_button = gtk::Button::new_with_label("Add via");
        add_button.set_margin_top(5);
        add_button.set_margin_bottom(5);
        add_button.set_margin_start(5);
        add_button.set_margin_end(5);

        container.add(&add_button);

        let widget = Self {
            container,
            label_size_group: label_size_group.clone(),
            favorites,
            add_button,
            vias: Arc::new(Mutex::new(vec![])),
        };

        widget.setup_event_handlers();

        widget
    }

    fn setup_event_handlers(&self) {
        let parent = self.clone();
        let vias = self.vias.clone();
        self.add_button.connect_clicked(move |_| {
            let entry = parent.create_entry();

            // Release lock before reload is called
            {
                let mut vias = vias.lock().unwrap();
                vias.push(entry);
            }

            parent.reload();
        });
    }

    fn create_entry(&self) -> LocationRowWidget {
        let entry = LocationRowWidget::new("Via", &self.label_size_group, self.favorites.clone());

        let vias = self.vias.clone();
        let widget = entry.clone();
        let parent = self.clone();
        entry.connect_cleared(move || {
            // Release lock before reload is called
            {
                let mut vias = vias.lock().unwrap();
                vias.retain(|f| f.container != widget.container);
            }

            parent.reload();
        });

        entry
    }

    fn reload(&self) {
        self.container.foreach(|child| {
            self.container.remove(child);
        });

        let vias = self.vias.clone();
        for via in vias.lock().unwrap().iter() {
            self.container.add(&via.container);
        }

        self.container.add(&self.add_button);
        self.container.show_all();
    }

    pub fn get_vias(&self) -> Vec<String> {
        let vias = self.vias.clone();
        let vias = vias.lock().unwrap();

        vias.iter()
            .map(|entry| entry.get_text())
            .filter(|via| via.is_empty() == false)
            .collect()
    }
}
