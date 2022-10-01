extern crate core;

mod networking;
mod ipv4;
mod ethernet;
mod tcp;
mod udp;
mod arp;
mod frame;
mod icmp;
mod database;
mod model;
mod view;
mod controller;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use pnet::datalink;
use pnet::datalink::NetworkInterface;

use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow, Button,
           CheckButton, Frame, Grid, Label, ComboBoxText, Entry };

use crate::database::Database;
use crate::database::Protocol;

fn send_packet(database: &Rc<RefCell<Database>>) {
    println!("Using iface: {}", database.borrow().get_iface());
}

fn generate_interface_protocol_section(database: &mut Rc<RefCell<Database>>) -> gtk::Box {
    let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
        .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

    /* Left "Interface:" label. */
    {
        let interfaces_title = Label::builder().label("Interface").halign(gtk::Align::Start).build();
        interfaces_title.add_css_class("ifaces-title");
        common_box.append(&interfaces_title);
    }

    /* Dropdown list in the middle. */
    {
        let interfaces = datalink::interfaces();
        let iface_list = ComboBoxText::builder().name("Interface-list").build();
        interfaces.iter().for_each(|iface| {
            iface_list.append(Some(&*iface.name), &*iface.name);
        });

        iface_list.set_active(Some(0));
        let database_for_ifaces = database.clone();
        database_for_ifaces.borrow_mut().set_iface(&iface_list.active_text().unwrap().to_string());
        iface_list.connect_changed(move |iface_list| {
            println!("Interface changed to {}", iface_list.active_text().unwrap());
            database_for_ifaces.borrow_mut().set_iface(&iface_list.active_text().unwrap().to_string());
        });

        common_box.append(&iface_list);
    }

    /* Protocol grid table. */
    {
        let protocol_table = Grid::builder().margin_start(6).margin_end(6).row_spacing(6)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(6).name("protocol-table").build();

        let ip_button = CheckButton::builder().label("IP").active(true).build();
        ip_button.connect_toggled(|_| { println!("Protocol to IP."); });
        let icmp_button = CheckButton::with_label("ICMP");
        icmp_button.connect_toggled(|_| { println!("Protocol to ICMP."); });
        let tcp_button = CheckButton::with_label("TCP");
        tcp_button.connect_toggled(|_| { println!("Protocol to TCP."); });
        let udp_button = CheckButton::with_label("UDP");
        udp_button.connect_toggled(|_| { println!("Protocol to UDP."); });

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
        let grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&Label::new(Some("Source IP")), 0, 0, 1, 1);
        let source_entry = Entry::builder().placeholder_text("Source IPv4").build();
        source_entry.connect_changed(move |source_entry| {
            println!("Source IP changed to {}", source_entry.text());
        });
        grid.attach(&source_entry, 1, 0, 1, 1);
        grid.attach(&Label::new(Some("Destination IP")), 0, 1, 1, 1);
        let destination_entry = Entry::builder().placeholder_text("Destination IPv4").build();
        grid.attach(&destination_entry, 1, 1, 1, 1);
        destination_entry.connect_changed(move |destination_entry| {
            println!("Destination IP changed to {}", destination_entry.text());
        });

        common_box.append(&grid);
    }

    /* Sending button on the right. */
    {
        let main_button = Button::builder().label("Generate").build();
        let database_for_button = database.clone();
        main_button.connect_clicked(move |button| {
            println!("Packet is sent.");
            // send_packet(&database_for_button);
        });
        common_box.append(&main_button);
    }

    common_box
}
fn generate_address_table(database: &Rc<RefCell<Database>>) -> Grid {
    let grid = Grid::builder().margin_start(24).margin_end(24).halign(gtk::Align::Center)
        .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

    let source_lable = Label::new(Some("Source MAC")); source_lable.set_halign(gtk::Align::Start);
    grid.attach(&source_lable, 0, 0, 1, 1);
    let source_entry = Entry::builder().placeholder_text("Source MAC").build();
    grid.attach(&source_entry, 1, 0, 1, 1);
    source_entry.connect_changed(move |source_entry| {
        println!("Source mac changed to {}", source_entry.text());
    });
    let destination_lable = Label::new(Some("Destination MAC")); destination_lable.set_halign(gtk::Align::Start);
    grid.attach(&destination_lable, 2, 0, 1, 1);
    let destination_entry = Entry::builder().placeholder_text("Destination MAC").build();
    grid.attach(&destination_entry, 3, 0, 1, 1);
    destination_entry.connect_changed(move |destination_entry| {
        println!("Destination mac changed to {}", destination_entry.text());
    });

    grid
}
fn generate_utility_buttons(database: &Rc<RefCell<Database>>) -> gtk::Box {
    let main_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
        .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

    let save = Button::with_label("Save Packet");
    main_box.append(&save); save.connect_clicked(move |_| { println!("Packet saved.") });
    let sequence = Button::with_label("Send Sequence");
    main_box.append(&sequence); sequence.connect_clicked(move |_| { println!("Sequence sent.") });
    let open_file = Button::with_label("Open File...");
    main_box.append(&open_file); open_file.connect_clicked(move |_| { println!("File opened.") });
    let delete_packet = Button::with_label("Delete Packet");
    main_box.append(&delete_packet); delete_packet.connect_clicked(move |_| { println!("Packet deleted.") });
    let delete_file = Button::with_label("Delete File");
    main_box.append(&delete_file); delete_file.connect_clicked(move |_| { println!("File deleted.") });
    let create_file = Button::with_label("Create File");
    main_box.append(&create_file); create_file.connect_clicked(move |_| { println!("File created.") });

    main_box
}
fn generate_ip_section(database: &Rc<RefCell<Database>>) -> Frame {
    /* Result box. */
    let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
        .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

    /* Left side */ {
        /* Left grid. Five rows. Each row consists of label, checkbox 'auto', text entry. */
        let left_grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        /* Left grid labels */ {
            /* Version */ {
                let version = Label::new(Some("Version:"));
                version.set_halign(gtk::Align::Start);
                left_grid.attach(&version, 0, 0, 1, 1);
            }
            /* IHL */ {
                let ihl = Label::new(Some("IHL:"));
                ihl.set_halign(gtk::Align::Start);
                left_grid.attach(&ihl, 0, 1, 1, 1);
            }
            /* Type of service */ {
                let tos = Label::new(Some("Type of Service:"));
                tos.set_halign(gtk::Align::Start);
                left_grid.attach(&tos, 0, 2, 1, 1);
            }
            /* Header length */ {
                let length = Label::new(Some("Header Length:"));
                length.set_halign(gtk::Align::Start);
                left_grid.attach(&length, 0, 3, 1, 1);
            }
            /* Header checksum */ {
                let checksum = Label::new(Some("Header Checksum:"));
                checksum.set_halign(gtk::Align::Start);
                left_grid.attach(&checksum, 0, 4, 1, 1);
            }
        }

        /* Left grid auto-entry boxes */ {
            /* Version */ {
                let version_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let version_button = CheckButton::builder().label("Auto").active(true).build();
                version_box.append(&version_button);
                version_button.connect_toggled(move |_| println!("Version auto activated."));
                let version_entry = Entry::new();
                version_box.append(&version_entry);
                version_entry.connect_changed(move |version_entry| println!("Version changed to {}", version_entry.text()));
                left_grid.attach(&(version_box.clone()), 1, 0, 1, 1);
            }
            /* IHL */ {
                let ihl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let ihl_button = CheckButton::builder().label("Auto").active(true).build();
                ihl_box.append(&ihl_button);
                ihl_button.connect_toggled(move |_| println!("IHL auto activated."));
                let ihl_entry = Entry::new();
                ihl_box.append(&ihl_entry);
                ihl_entry.connect_changed(move |ihl_entry| println!("IHL changed to {}", ihl_entry.text()));
                left_grid.attach(&(ihl_box.clone()), 1, 1, 1, 1);
            }
            /* Type of service */ {
                let type_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let type_button = CheckButton::builder().label("Auto").active(true).build();
                type_box.append(&type_button);
                type_button.connect_toggled(move |_| println!("Type of service auto activated."));
                let type_entry = Entry::new();
                type_box.append(&type_entry);
                type_entry.connect_changed(move |type_entry| println!("Type of service changed to {}", type_entry.text()));
                left_grid.attach(&(type_box.clone()), 1, 2, 1, 1);
            }
            /* Header length */ {
                let length_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let length_button = CheckButton::builder().label("Auto").active(true).build();
                length_box.append(&length_button);
                length_button.connect_toggled(move |_| println!("Header length auto activated."));
                let length_entry = Entry::new();
                length_box.append(&length_entry);
                length_entry.connect_changed(move |type_entry| println!("Header length changed to {}", type_entry.text()));
                left_grid.attach(&(length_box.clone()), 1, 3, 1, 1);
            }
            /* Header checksum */ {
                let checksum_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let checksum_button = CheckButton::builder().label("Auto").active(true).build();
                checksum_box.append(&checksum_button);
                checksum_button.connect_toggled(move |_| println!("Header length auto activated."));
                let checksum_entry = Entry::new();
                checksum_box.append(&checksum_entry);
                checksum_entry.connect_changed(move |type_entry| println!("Header length changed to {}", type_entry.text()));
                left_grid.attach(&(checksum_box.clone()), 1, 4, 1, 1);
            }
        }

        common_box.append(&left_grid);
    }

    /* Right side */ {
        /* Right box. Gathers right grid and bottom box together. */
        let right_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        /* Right grid. */ {
            /* Right grid. Four rows. Each row consists of label, checkbox 'auto', text entry. */
            let right_grid = Grid::builder().halign(gtk::Align::Center)
                .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

            /* Right grid labels */ {
                /* Packet ID */ {
                    let packet_id = Label::new(Some("Packet ID:"));
                    packet_id.set_halign(gtk::Align::Start);
                    right_grid.attach(&packet_id, 0, 0, 1, 1);
                }
                /* Protocol */ {
                    let protocol = Label::new(Some("Protocol:"));
                    protocol.set_halign(gtk::Align::Start);
                    right_grid.attach(&protocol, 0, 1, 1, 1);
                }
                /* Offset */ {
                    let offset = Label::new(Some("Fragment offset:"));
                    offset.set_halign(gtk::Align::Start);
                    right_grid.attach(&offset, 0, 2, 1, 1);
                }
                /* Time to live */ {
                    let ttl = Label::new(Some("Time to Live:"));
                    ttl.set_halign(gtk::Align::Start);
                    right_grid.attach(&ttl, 0, 3, 1, 1);
                }
            }

            /* Right grid auto-entry boxes */ {
                /* Packet ID */ {
                    let packet_id_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let packet_id_button = CheckButton::builder().label("Auto").active(true).build();
                    packet_id_box.append(&packet_id_button);
                    packet_id_button.connect_toggled(move |_| println!("Packet ID auto activated."));
                    let packet_id_entry = Entry::new();
                    packet_id_box.append(&packet_id_entry);
                    packet_id_entry.connect_changed(move |packet_id_entry| println!("Packet ID changed to {}", packet_id_entry.text()));
                    right_grid.attach(&(packet_id_box.clone()), 1, 0, 1, 1);
                }
                /* Protocol */ {
                    let protocol_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let protocol_button = CheckButton::builder().label("Auto").active(true).build();
                    protocol_box.append(&protocol_button);
                    protocol_button.connect_toggled(move |_| println!("Protocol auto activated."));
                    let protocol_entry = Entry::new();
                    protocol_box.append(&protocol_entry);
                    protocol_entry.connect_changed(move |packet_id_entry| println!("Protocol changed to {}", packet_id_entry.text()));
                    right_grid.attach(&(protocol_box.clone()), 1, 1, 1, 1);
                }
                /* Offset */ {
                    let offset_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let offset_button = CheckButton::builder().label("Auto").active(true).build();
                    offset_box.append(&offset_button);
                    offset_button.connect_toggled(move |_| println!("Offset auto activated."));
                    let offset_entry = Entry::new();
                    offset_box.append(&offset_entry);
                    offset_entry.connect_changed(move |packet_id_entry| println!("Offset changed to {}", packet_id_entry.text()));
                    right_grid.attach(&(offset_box.clone()), 1, 2, 1, 1);
                }
                /* Time to live */ {
                    let ttl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let ttl_button = CheckButton::builder().label("Auto").active(true).build();
                    ttl_box.append(&ttl_button);
                    ttl_button.connect_toggled(move |_| println!("Time to live auto activated."));
                    let ttl_entry = Entry::new();
                    ttl_box.append(&ttl_entry);
                    ttl_entry.connect_changed(move |packet_id_entry| println!("Time to live changed to {}", packet_id_entry.text()));
                    right_grid.attach(&(ttl_box.clone()), 1, 3, 1, 1);
                }
            }

            right_box.append(&right_grid);
        }

        /* Right bottom box */ {
            /* Right box in the bottom. Specifies flags DF, MF and reserved bit. */
            let right_down_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
                .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

            /* Right grid flags and reserved bits */ {
                right_down_box.append(&Label::new(Some("Flags:")));
                let df_button = CheckButton::builder().label("DF").build();
                right_down_box.append(&df_button);
                df_button.connect_toggled(move |_| println!("DF enabled."));
                let mf_button = CheckButton::builder().label("MF").build();
                right_down_box.append(&mf_button);
                mf_button.connect_toggled(move |_| println!("MF enabled."));
                let reserved = CheckButton::builder().label("Reserved bit").build();
                right_down_box.append(&reserved);
                reserved.connect_toggled(move |_| println!("Reserved bit changed."));
            }

            right_box.append(&right_down_box);
        }

        common_box.append(&right_box);
    }

    let box_frame = Frame::builder().label("IP options").build();
    box_frame.set_child(Some(&common_box));

    box_frame
}
fn generate_tcp_section(database: &Rc<RefCell<Database>>) -> Frame {
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

    let frame = Frame::builder().label("TCP options").build();
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

        let mut packet_database = Rc::new(RefCell::new(Database::new()));

        let ip_protocol_table = generate_interface_protocol_section(&mut packet_database);
        container.append(&ip_protocol_table);

        let mac_address_table = generate_address_table(&mut packet_database);
        container.append(&mac_address_table);

        let buttons = generate_utility_buttons(&mut packet_database);
        container.append(&buttons);

        let ip_options = generate_ip_section(&mut packet_database);
        container.append(&ip_options);

        let tcp_options = generate_tcp_section(&mut packet_database);
        container.append(&tcp_options);

        window.set_child(Some(&container));
        window.show();
    });

    application.run();
}