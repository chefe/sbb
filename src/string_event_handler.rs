use gio::prelude::*;

#[derive(Clone)]
pub struct StringEventHandler {
    action: gio::SimpleAction,
}

impl StringEventHandler {
    pub fn new(name: &str) -> Self {
        let str_variant = "".to_variant();
        let arg_type = Some(str_variant.type_());
        let action = gio::SimpleAction::new(name, arg_type);
        Self { action }
    }

    pub fn trigger(&self, text: &str) {
        self.action.activate(Some(&text.to_variant()));
    }

    pub fn connect<F>(&self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.action.connect_activate(move |_, text| {
            callback(text.unwrap().get_str().unwrap());
        });
    }
}
