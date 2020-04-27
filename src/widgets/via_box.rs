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
    vias: Arc<Mutex<Vec<LocationRowWidget>>>,
}

impl ViaBoxWidget {
    pub fn new(label_size_group: &gtk::SizeGroup, favorites: Arc<Favorites>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let widget = Self {
            container,
            label_size_group: label_size_group.clone(),
            favorites,
            vias: Arc::new(Mutex::new(vec![])),
        };

        widget.add_entry();

        widget
    }

    fn add_entry(&self) {
        let entry = self.create_entry();
        self.container.add(&entry.container);

        let vias = self.vias.clone();
        let mut vias = vias.lock().unwrap();
        vias.push(entry);

        self.container.show_all();
    }

    fn create_entry(&self) -> LocationRowWidget {
        let entry = LocationRowWidget::new("Via", &self.label_size_group, self.favorites.clone());

        let vias = self.vias.clone();
        let widget = entry.clone();
        let parent = self.clone();
        entry.connect_cleared(move || {
            let mut vias = vias.lock().unwrap();

            if vias.len() == 1 {
                // the last element can no be removed
                return;
            }

            vias.retain(|f| f.container != widget.container);
            parent.container.remove(&widget.container);
            parent.container.show_all();
        });

        let parent = self.clone();
        entry.connect_changed(move || {
            parent.add_new_via_if_required();
        });

        entry
    }

    fn add_new_via_if_required(&self) {
        let are_all_vias_filled = self
            .get_vias_matching(|via| via.is_empty() == true)
            .is_empty();

        if are_all_vias_filled {
            self.add_entry();
        }
    }

    pub fn get_vias(&self) -> Vec<String> {
        self.get_vias_matching(|via| via.is_empty() == false)
    }

    fn get_vias_matching<F>(&self, filter: F) -> Vec<String>
    where
        F: Fn(&str) -> bool,
    {
        let vias = self.vias.clone();
        let vias = vias.lock().unwrap();

        vias.iter()
            .map(|entry| entry.get_text())
            .filter(|via| filter(via))
            .collect()
    }
}
