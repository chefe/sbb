use gio::prelude::*;

use std::fs;
use std::path;

pub struct Favorites {
    favorites_file: Option<path::PathBuf>,
    key_file: glib::KeyFile,
    changed: gio::SimpleAction,
}

impl Favorites {
    pub fn new() -> Self {
        let key_file = glib::KeyFile::new();

        let favorites_file = match Self::get_data_dir() {
            Some(dir) => Some(dir.join("favorites")),
            None => None,
        };

        Self {
            key_file,
            favorites_file,
            changed: gio::SimpleAction::new("changed", None),
        }
    }

    fn get_data_dir() -> Option<path::PathBuf> {
        let data_dir = match glib::get_user_data_dir() {
            Some(dir) => dir.join("io.chefe.sbb"),
            None => return None,
        };

        match fs::create_dir_all(&data_dir) {
            Ok(()) => Some(data_dir),
            Err(_) => None,
        }
    }

    pub fn get(&self) -> Vec<String> {
        let favorites_file = match &self.favorites_file {
            Some(file) => file,
            None => return vec![],
        };

        let flags = glib::KeyFileFlags::all();
        if let Err(_) = self.key_file.load_from_file(favorites_file, flags) {
            return vec![];
        }

        let favorites = match self.key_file.get_string("General", "Favorites") {
            Ok(f) => f.as_str().to_owned(),
            Err(_) => "".to_owned(),
        };

        favorites.split("; ").map(|i| i.to_owned()).collect()
    }

    fn store(&self, favorites: Vec<String>) {
        let favorites_file = match &self.favorites_file {
            Some(f) => f,
            None => return,
        };

        let favorites = favorites.join("; ");
        self.key_file.set_string("General", "Favorites", &favorites);

        self.key_file
            .save_to_file(favorites_file)
            .expect("Failed to store favorites");
    }

    pub fn add(&self, favorite: &str) {
        if favorite.len() == 0 {
            return;
        }

        let mut favorites = self.get();
        favorites.push(favorite.to_owned());
        self.store(favorites);
        self.changed.activate(None);
    }

    pub fn remove(&self, favorite: &str) {
        let mut favorites = self.get();
        favorites.retain(|f| f != favorite);
        favorites.sort();
        self.store(favorites);
        self.changed.activate(None);
    }

    pub fn contains(&self, favorite: &str) -> bool {
        self.get().contains(&favorite.to_owned())
    }

    pub fn connect_changed<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.changed.connect_activate(move |_, _| {
            callback();
        });
    }
}
