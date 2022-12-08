use gtk::prelude::*;
use crate::widgets::MainWindow;

mod database;
mod widgets;
mod error_window;
mod icmp;
mod udp;
mod ip;
mod tcp;
mod show_packet;

fn main() {
    let application = gtk::Application::builder()
        .application_id("Network Packet Generator")
        .build();

    application.connect_activate(|app| {
        let window = MainWindow::new(app);
        window.show();
    });

    application.run();
}