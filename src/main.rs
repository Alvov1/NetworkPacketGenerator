extern crate core;

mod networking;
mod ipv4;
mod ethernet;
mod tcp;
mod udp;
mod arp;
mod frame;
mod icmp;

use pnet::datalink;
use pnet::datalink::NetworkInterface;

use std::env;
use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow, Button,
           CheckButton, Frame, Grid, Label, ComboBoxText, Entry, gdk};
use glib::GString;
use crate::gdk::gio;
use crate::gdk::glib::clone;

fn send_packet(iface: &NetworkInterface) {
    println!("Package is sent through interface {}.", iface.name);
}

fn generate_interface_protocol_section() -> gtk::Box {
    let common_box = gtk::Box::builder().name("Section 1").orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
        .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

    /* Left "Interface:" label. */
    {
        let interfaces_title = Label::builder().name("Interface label").label("Interface")
            .halign(gtk::Align::Start).name("Interface-label").build();
        interfaces_title.add_css_class("ifaces-title");
        common_box.append(&interfaces_title);
    }

    /* Dropdown list in the middle. */
    {
        let interfaces = datalink::interfaces();
        let iface_list = ComboBoxText::builder().name("Interface list").build();
        interfaces.iter().for_each(|iface| {
            iface_list.append(Some(&*iface.name), &*iface.name);
        });
        iface_list.set_active(Some(0));
        common_box.append(&iface_list);
    }

    /* Protocol grid table. */
    {
        let protocol_table = Grid::builder().name("Protocol table").margin_start(6).margin_end(6).row_spacing(6)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(6).name("Protocol-table").build();

        let ip_button = CheckButton::builder().label("IP").active(true).build();
        let icmp_button = CheckButton::with_label("ICMP");
        let tcp_button = CheckButton::with_label("TCP");
        let udp_button = CheckButton::with_label("UDP");

        tcp_button.set_group(Some(&udp_button));
        icmp_button.set_group(Some(&tcp_button));
        ip_button.set_group(Some(&icmp_button));

        protocol_table.attach(&ip_button, 0, 0, 1, 1);
        protocol_table.attach(&icmp_button, 1, 0, 1, 1);
        protocol_table.attach(&tcp_button, 0, 1, 1, 1);
        protocol_table.attach(&udp_button, 1, 1, 1, 1);

        common_box.append(&protocol_table);
    }

    /* IP addresses grid. */
    {
        let grid = Grid::builder().name("IP table").margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).name("IP-table").build();

        grid.attach(&Label::new(Some("Source IP")), 0, 0, 1, 1);
        grid.attach(&Entry::builder().placeholder_text("Source IPv4").build(), 1, 0, 1, 1);
        grid.attach(&Label::new(Some("Destination IP")), 0, 1, 1, 1);
        grid.attach(&Entry::builder().placeholder_text("Destination IPv4").build(), 1, 1, 1, 1);

        common_box.append(&grid);
    }

    /* Sending button on the right. */
    {
        let main_button = Button::builder().name("Generate button").label("Generate").name("Generate-button").build();
        main_button.connect_clicked(move |button| {
            println!("Packet is sent.");
        });
        common_box.append(&main_button);
    }

    common_box
}
fn generate_address_table() -> Grid {
    let grid = Grid::builder().name("Section 2").margin_start(24).margin_end(24).halign(gtk::Align::Center)
        .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

    let source_lable = Label::new(Some("Source MAC")); source_lable.set_halign(gtk::Align::Start);
    grid.attach(&source_lable, 0, 0, 1, 1);
    grid.attach(&Entry::builder().placeholder_text("Source MAC").build(), 1, 0, 1, 1);
    let destination_lable = Label::new(Some("Destination MAC")); destination_lable.set_halign(gtk::Align::Start);
    grid.attach(&destination_lable, 2, 0, 1, 1);
    grid.attach(&Entry::builder().placeholder_text("Destination MAC").build(), 3, 0, 1, 1);

    grid
}
fn generate_utility_buttons() -> gtk::Box {
    let main_box = gtk::Box::builder().name("Section 3").orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
        .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

    main_box.append(&Button::with_label("Save Packet"));
    main_box.append(&Button::with_label("Send Sequence"));
    main_box.append(&Button::with_label("Open File..."));
    main_box.append(&Button::with_label("Delete Packet"));
    main_box.append(&Button::with_label("Delete File"));
    main_box.append(&Button::with_label("Create File"));

    main_box
}
fn generate_ip_section() -> Frame {
    /* Left grid. Five rows. Each row consists of label, checkbox 'auto', text entry. */
    let left_grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
        .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();
    let version = Label::new(Some("Version:")); version.set_halign(gtk::Align::Start);
    left_grid.attach(&version, 0, 0, 1, 1);
    let ihl = Label::new(Some("IHL:")); ihl.set_halign(gtk::Align::Start);
    left_grid.attach(&ihl, 0, 1, 1, 1);
    let tos = Label::new(Some("Type of Service:")); tos.set_halign(gtk::Align::Start);
    left_grid.attach(&tos, 0, 2, 1, 1);
    let length = Label::new(Some("Header Length:")); length.set_halign(gtk::Align::Start);
    left_grid.attach(&length, 0, 3, 1, 1);
    let checksum = Label::new(Some("Header Checksum:")); checksum.set_halign(gtk::Align::Start);
    left_grid.attach(&checksum, 0, 4, 1, 1);
    for row in 0..5 {
        let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        auto_entry_box.append(&CheckButton::builder().label("Auto").active(true).build());
        auto_entry_box.append(&Entry::new());
        left_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
    }

    /* Right grid. Four rows. Each row consists of label, checkbox 'auto', text entry. */
    let right_grid = Grid::builder().halign(gtk::Align::Center)
        .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();
    let packet_id = Label::new(Some("Packet ID:")); packet_id.set_halign(gtk::Align::Start);
    right_grid.attach(&packet_id, 0, 0, 1, 1);
    let protocol = Label::new(Some("Protocol:")); protocol.set_halign(gtk::Align::Start);
    right_grid.attach(&protocol, 0, 1, 1, 1);
    let offset= Label::new(Some("Fragment offset:")); offset.set_halign(gtk::Align::Start);
    right_grid.attach(&offset, 0, 2, 1, 1);
    let ttl = Label::new(Some("Time to Live:")); ttl.set_halign(gtk::Align::Start);
    right_grid.attach(&ttl, 0, 3, 1, 1);
    for row in 0..4 {
        let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        auto_entry_box.append(&CheckButton::builder().label("Auto").active(true).build());
        auto_entry_box.append(&Entry::new());
        right_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
    }

    /* Right box in the bottom. Specifies flags DF, MF and reserved bit. */
    let right_down_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
        .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
    right_down_box.append(&Label::new(Some("Flags:")));
    right_down_box.append(&CheckButton::builder().label("DF").build());
    right_down_box.append(&CheckButton::builder().label("MF").build());
    right_down_box.append(&CheckButton::builder().label("Reserved bit").build());

    /* Right box. Gathers right grid and bottom box together. */
    let right_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
    right_box.append(&right_grid); right_box.append(&right_down_box);

    /* Result box. */
    let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
        .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();
    common_box.append(&left_grid); common_box.append(&right_box);

    let box_frame = Frame::builder().name("Section 4").label("IP options").build();
    box_frame.set_child(Some(&common_box));

    box_frame
}
fn generate_tcp_section() -> Frame {
    let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).halign(gtk::Align::Center)
        .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

    /* Upper box. */
    {
        /* Left grid. Consists of 4 rows. Each one is label - checkbox auto - text entry. */
        let left_grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();
        let source_port = Label::builder().label("Source port:").halign(gtk::Align::Start).build();
        left_grid.attach(&source_port, 0, 0, 1, 1);
        let destination_port = Label::builder().label("Destination port:").halign(gtk::Align::Start).build();
        left_grid.attach(&destination_port, 0, 1, 1, 1);
        let sequence_number = Label::builder().label("Sequence number:").halign(gtk::Align::Start).build();
        left_grid.attach(&sequence_number, 0, 2, 1, 1);
        let acknowledgement = Label::builder().label("Acknowledgement number:").halign(gtk::Align::Start).build();
        left_grid.attach(&acknowledgement, 0, 3, 1, 1);
        for row in 0..4 {
            let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            auto_entry_box.append(&CheckButton::builder().label("Auto").active(true).build());
            auto_entry_box.append(&Entry::new());
            left_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
        }

        /* Middle grid. Consists of 4 rows. Each one is label - checkbox auto - text entry. */
        let middle_grid = Grid::builder().halign(gtk::Align::Center)
            .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();
        let data_offset = Label::builder().label("Data offset:").halign(gtk::Align::Start).build();
        middle_grid.attach(&data_offset, 0, 0, 1, 1);
        let window = Label::builder().label("Window:").halign(gtk::Align::Start).build();
        middle_grid.attach(&window, 0, 1, 1, 1);
        let checksum = Label::builder().label("Checksum:").halign(gtk::Align::Start).build();
        middle_grid.attach(&checksum, 0, 2, 1, 1);
        let urgent = Label::builder().label("Urgent:").halign(gtk::Align::Start).build();
        middle_grid.attach(&urgent, 0, 3, 1, 1);
        for row in 0..4 {
            let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            auto_entry_box.append(&CheckButton::builder().label("Auto").active(true).build());
            auto_entry_box.append(&Entry::new());
            middle_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
        }

        /* Right grid with flags. */
        let right_inner_grid = Grid::builder().halign(gtk::Align::Center)
            .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();
        right_inner_grid.attach(&CheckButton::with_label("ACK"), 0, 0, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("SYN"), 1, 0, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("PSH"), 0, 1, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("FIN"), 1, 1, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("RST"), 0, 2, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("URG"), 1, 2, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("ECE"), 0, 3, 1, 1);
        right_inner_grid.attach(&CheckButton::with_label("CWR"), 1, 3, 1, 1);

        /* Frame with flags check buttons. */
        let right_frame = Frame::builder().label("Flags").child(&right_inner_grid).build();

        /* Upper box. */
        let upper_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();
        upper_box.append(&left_grid); upper_box.append(&middle_grid); upper_box.append(&right_frame);

        main_box.append(&upper_box);
    }

    /* Lower box. */
    {
        let data_label = Label::new(Some("Data (Various):"));

        let data_text_entry = Entry::builder().placeholder_text("Enter data").build();

        let reserved_bits_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
            .halign(gtk::Align::Center).margin_end(24).valign(gtk::Align::Center).spacing(24).build();
        reserved_bits_box.append(&CheckButton::with_label("1"));
        reserved_bits_box.append(&CheckButton::with_label("2"));
        reserved_bits_box.append(&CheckButton::with_label("3"));
        reserved_bits_box.append(&CheckButton::with_label("4"));

        let reserved_bits_frame = Frame::builder().label("Reserved bits").child(&reserved_bits_box).build();

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();
        lower_box.append(&data_label); lower_box.append(&data_text_entry); lower_box.append(&reserved_bits_frame);

        main_box.append(&lower_box);
    }

    let frame = Frame::builder().name("Section 5").label("TCP options").build();
    frame.set_child(Some(&main_box));
    frame
}

fn get_current_configuration(container: &gtk::Box) {
    let first = container.first_child().unwrap();
    let ip_table = first.find_property("Section 1").unwrap();
}

fn main() {
    let application = Application::builder()
        .application_id("Network Packet Generator")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder().application(app)
            .title("Network Packet Generator").default_width(900).default_height(500).build();

        let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24).margin_bottom(24)
            .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        let ip_protocol_table = generate_interface_protocol_section();
        container.append(&ip_protocol_table);

        let mac_address_table = generate_address_table();
        container.append(&mac_address_table);

        let buttons = generate_utility_buttons();
        container.append(&buttons);

        let ip_options = generate_ip_section();
        container.append(&ip_options);

        let tcp_options = generate_tcp_section();
        container.append(&tcp_options);

        window.set_child(Some(&container));
        window.show();
    });

    application.run();
}