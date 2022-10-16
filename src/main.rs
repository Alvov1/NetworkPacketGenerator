extern crate core;

use std::sync::{Arc, Mutex};
use gtk::{Application, ApplicationWindow, Button};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use crate::widgets::MyWidgets;

mod database;
mod gui;
mod widgets;
mod error_window;
mod icmp;

fn main() {
    let application = Application::builder()
        .application_id("Network Packet Generator")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder().application(app)
            .title("Network Packet Generator").default_width(900).default_height(500).build();

        let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24).margin_bottom(24)
            .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        let widgets = Arc::new(Mutex::new(MyWidgets::new()));

        let ip_protocol_table = gui::generate_interface_protocol_section(&widgets.lock().unwrap());
        let main_button = Button::with_label("Collect");
        let pointer = widgets.clone();
        main_button.connect_clicked(move |_| {
            pointer.lock().unwrap().collect();
        });
        ip_protocol_table.append(&main_button);
        container.append(&ip_protocol_table);

        let mac_address_table = gui::generate_address_table(&widgets.lock().unwrap());
        container.append(&mac_address_table);

        let buttons = gui::generate_utility_buttons(&widgets.lock().unwrap());
        container.append(&buttons);

        let ip_options = gui::generate_ip_section(&widgets.lock().unwrap());
        container.append(&ip_options);

        let tcp_options = gui::generate_tcp_section(&widgets.lock().unwrap());
        container.append(&tcp_options);

        window.set_child(Some(&container));
        window.show();
    });

    application.run();
}