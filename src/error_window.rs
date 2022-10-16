use gtk::{Label, Window};
use gtk::prelude::{GtkWindowExt, WidgetExt};

pub fn error(what: &str) {
    let window = Window::builder().title("Error").default_width(200).default_height(100).build();
    let label = Label::new(Some(what)); window.set_child(Some(&label)); window.show();
}