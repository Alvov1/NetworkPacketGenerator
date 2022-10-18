use gtk::prelude::*;

pub fn error(what: &str) {
    let window = gtk::Window::builder().title("Error").default_width(200).default_height(100).build();
    let label = gtk::Label::new(Some(what)); window.set_child(Some(&label)); window.show();
}