extern crate core;

use std::sync::{Arc, Mutex};
use gtk::{Application, ApplicationWindow, Button};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt, WidgetExt};

mod database;
mod gui;
mod widgets;
mod error_window;
mod icmp;
mod udp;

fn main() {
    let application = Application::builder()
        .application_id("Network Packet Generator")
        .build();

    application.connect_activate(|app| {
        let window = MainWindow::new(app);
        window.show();
    });

    application.run();
}