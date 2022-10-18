use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use glib::ObjectExt;
use gtk::prelude::*;

use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::icmp::MutableIcmpPacket;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::ethernet::MutableEthernetPacket;

use crate::error_window::error;
use crate::udp::UdpOptions;
use crate::icmp::IcmpOptions;
use pnet::datalink;

struct IPWidgets {
    src_ip_entry: gtk::Entry,
    dest_ip_entry: gtk::Entry,

    ip_version: (gtk::CheckButton, gtk::Entry),
    ip_ihl: (gtk::CheckButton, gtk::Entry),
    ip_type_of_service: (gtk::CheckButton, gtk::Entry),
    ip_header_length: (gtk::CheckButton, gtk::Entry),
    ip_checksum: (gtk::CheckButton, gtk::Entry),
    ip_packet_id: (gtk::CheckButton, gtk::Entry),
    ip_next_protocol: (gtk::CheckButton, gtk::Entry),
    ip_offset: (gtk::CheckButton, gtk::Entry),
    ip_ttl: (gtk::CheckButton, gtk::Entry),

    ip_flags: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton),
}
impl IPWidgets {
    fn new() -> Self {
        Self {
            src_ip_entry: gtk::Entry::builder().placeholder_text("Source IPv4").build(),
            dest_ip_entry: gtk::Entry::builder().placeholder_text("Destination IPv4").build(),

            ip_version: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_ihl: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_type_of_service: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_header_length: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_checksum: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_packet_id: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_next_protocol: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_offset: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            ip_ttl: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),

            ip_flags: (gtk::CheckButton::with_label("DF"),
                       gtk::CheckButton::with_label("MF"),
                       gtk::CheckButton::with_label("Reserved bit")),
        }
    }

    fn prepare_address_section(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&gtk::Label::new(Some("Source IP")), 0, 0, 1, 1);
        grid.attach(&self.src_ip_entry, 1, 0, 1, 1);
        grid.attach(&gtk::Label::new(Some("Destination IP")), 0, 1, 1, 1);
        grid.attach(&self.dest_ip_entry, 1, 1, 1, 1);

        grid
    }
    fn prepare_options_section(&self) -> gtk::Frame {
        /* Result box. */
        let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
            .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

        /* Left side */ {
            /* Left grid. Five rows. Each row consists of label, checkbox 'auto', text entry. */
            let left_grid = gtk::Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
                .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

            /* Left grid labels */ {
                /* Version */ {
                    let version = gtk::Label::builder().label("Version:").halign(gtk::Align::Start).build();
                    left_grid.attach(&version, 0, 0, 1, 1);
                }
                /* IHL */ {
                    let ihl = gtk::Label::builder().label("IHL:").halign(gtk::Align::Start).build();
                    left_grid.attach(&ihl, 0, 1, 1, 1);
                }
                /* Type of service */ {
                    let tos = gtk::Label::builder().label("Type of Service:").halign(gtk::Align::Start).build();
                    left_grid.attach(&tos, 0, 2, 1, 1);
                }
                /* Header length */ {
                    let length = gtk::Label::builder().label("Header Length:").halign(gtk::Align::Start).build();
                    left_grid.attach(&length, 0, 3, 1, 1);
                }
                /* Header checksum */ {
                    let checksum = gtk::Label::builder().label("Header Checksum:").halign(gtk::Align::Start).build();
                    left_grid.attach(&checksum, 0, 4, 1, 1);
                }
            }

            /* Left grid auto-entry boxes */ {
                /* Version */ {
                    let version_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    version_box.append(&self.ip_version.0); version_box.append(&self.ip_version.1);
                    left_grid.attach(&(version_box.clone()), 1, 0, 1, 1);
                }
                /* IHL */ {
                    let ihl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    ihl_box.append(&self.ip_ihl.0); ihl_box.append(&self.ip_ihl.1);
                    left_grid.attach(&(ihl_box.clone()), 1, 1, 1, 1);
                }
                /* Type of service */ {
                    let type_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    type_box.append(&self.ip_type_of_service.0); type_box.append(&self.ip_type_of_service.1);
                    left_grid.attach(&(type_box.clone()), 1, 2, 1, 1);
                }
                /* Header length */ {
                    let length_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    length_box.append(&self.ip_header_length.0); length_box.append(&self.ip_header_length.1);
                    left_grid.attach(&(length_box.clone()), 1, 3, 1, 1);
                }
                /* Header checksum */ {
                    let checksum_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    checksum_box.append(&self.ip_checksum.0); checksum_box.append(&self.ip_checksum.1);
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
                let right_grid = gtk::Grid::builder().halign(gtk::Align::Center)
                    .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

                /* Right grid labels */ {
                    /* Packet ID */ {
                        let packet_id = gtk::Label::builder().label("Packet ID:").halign(gtk::Align::Start).build();
                        right_grid.attach(&packet_id, 0, 0, 1, 1);
                    }
                    /* Protocol */ {
                        let protocol = gtk::Label::builder().label("Protocol:").halign(gtk::Align::Start).build();
                        right_grid.attach(&protocol, 0, 1, 1, 1);
                    }
                    /* Offset */ {
                        let offset = gtk::Label::builder().label("Fragment offset:").halign(gtk::Align::Start).build();
                        right_grid.attach(&offset, 0, 2, 1, 1);
                    }
                    /* Time to live */ {
                        let ttl = gtk::Label::builder().label("Time to Live:").halign(gtk::Align::Start).build();
                        right_grid.attach(&ttl, 0, 3, 1, 1);
                    }
                }

                /* Right grid auto-entry boxes */ {
                    /* Packet ID */ {
                        let packet_id_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        packet_id_box.append(&self.ip_packet_id.0); packet_id_box.append(&self.ip_packet_id.1);
                        right_grid.attach(&(packet_id_box.clone()), 1, 0, 1, 1);
                    }
                    /* Protocol */ {
                        let protocol_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        protocol_box.append(&self.ip_next_protocol.0); protocol_box.append(&self.ip_next_protocol.1);
                        right_grid.attach(&(protocol_box.clone()), 1, 1, 1, 1);
                    }
                    /* Offset */ {
                        let offset_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        offset_box.append(&self.ip_offset.0); offset_box.append(&self.ip_offset.1);
                        right_grid.attach(&(offset_box.clone()), 1, 2, 1, 1);
                    }
                    /* Time to live */ {
                        let ttl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        ttl_box.append(&self.ip_ttl.0); ttl_box.append(&self.ip_ttl.1);
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
                    right_down_box.append(&gtk::Label::new(Some("Flags:")));
                    right_down_box.append(&self.ip_flags.0);
                    right_down_box.append(&self.ip_flags.1);
                    right_down_box.append(&self.ip_flags.2);
                }

                right_box.append(&right_down_box);
            }

            common_box.append(&right_box);
        }

        let box_frame = gtk::Frame::builder().label("IP options").build();
        box_frame.set_child(Some(&common_box));

        box_frame
    }

    // fn build_packet() -> MutableIpPacket<'static> {
    //
    // }
}

struct TCPWidgets {
    tcp_source_port: (gtk::CheckButton, gtk::Entry),
    tcp_dest_port: (gtk::CheckButton, gtk::Entry),
    tcp_sequence_number: (gtk::CheckButton, gtk::Entry),
    tcp_acknowledgement: (gtk::CheckButton, gtk::Entry),
    tcp_offset: (gtk::CheckButton, gtk::Entry),
    tcp_window: (gtk::CheckButton, gtk::Entry),
    tcp_checksum: (gtk::CheckButton, gtk::Entry),
    tcp_urgent: (gtk::CheckButton, gtk::Entry),

    tcp_flags: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton,
                    gtk::CheckButton, gtk::CheckButton, gtk::CheckButton,
                    gtk::CheckButton, gtk::CheckButton),

    tcp_data: gtk::Entry,

    tcp_reserved_bits: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton, gtk::CheckButton)
}
impl TCPWidgets {
    fn new() -> Self {
        Self {
            tcp_source_port: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_dest_port: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_sequence_number: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_acknowledgement: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_offset: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_window: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_checksum: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),
            tcp_urgent: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::new()),

            tcp_flags: (gtk::CheckButton::with_label("ACK"), gtk::CheckButton::with_label("SYN"),
                        gtk::CheckButton::with_label("PSH"), gtk::CheckButton::with_label("FIN"),
                        gtk::CheckButton::with_label("RST"), gtk::CheckButton::with_label("URG"),
                        gtk::CheckButton::with_label("ECE"), gtk::CheckButton::with_label("CWR")),

            tcp_data: gtk::Entry::builder().placeholder_text("Enter data").build(),

            tcp_reserved_bits: (gtk::CheckButton::with_label("1"), gtk::CheckButton::with_label("2"),
                                gtk::CheckButton::with_label("3"), gtk::CheckButton::with_label("4"))

        }
    }

    fn prepare_ui_fields(&self) -> gtk::Frame {
        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

        /* Upper box. */ {
            let upper_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

            /* Left grid */ {
                /* Left grid. Consists of 4 rows. Each one is label - checkbox auto - text entry. */
                let left_grid = gtk::Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
                    .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

                /* Left grid labels */ {
                    /* Source port */ {
                        let source_port = gtk::Label::builder().label("Source port:").halign(gtk::Align::Start).build();
                        left_grid.attach(&source_port, 0, 0, 1, 1);
                    }
                    /* Destination port */ {
                        let destination_port = gtk::Label::builder().label("Destination port:").halign(gtk::Align::Start).build();
                        left_grid.attach(&destination_port, 0, 1, 1, 1);
                    }
                    /* Sequence number */ {
                        let sequence_number = gtk::Label::builder().label("Sequence number:").halign(gtk::Align::Start).build();
                        left_grid.attach(&sequence_number, 0, 2, 1, 1);
                    }
                    /* Acknowledgement */ {
                        let acknowledgement = gtk::Label::builder().label("Acknowledgement number:").halign(gtk::Align::Start).build();
                        left_grid.attach(&acknowledgement, 0, 3, 1, 1);
                    }
                }

                /* Left grid auto-entry boxes */ {
                    /* Source port */ {
                        let source_port_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        source_port_box.append(&self.tcp_source_port.0); source_port_box.append(&self.tcp_source_port.1);
                        left_grid.attach(&(source_port_box.clone()), 1, 0, 1, 1);
                    }
                    /* Destination port */ {
                        let destination_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        destination_box.append(&self.tcp_dest_port.0); destination_box.append(&self.tcp_dest_port.1);
                        left_grid.attach(&(destination_box.clone()), 1, 1, 1, 1);
                    }
                    /* Sequence number */ {
                        let sequence_number_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        sequence_number_box.append(&self.tcp_sequence_number.0); sequence_number_box.append(&self.tcp_sequence_number.1);
                        left_grid.attach(&(sequence_number_box.clone()), 1, 2, 1, 1);
                    }
                    /* Acknowledgement */ {
                        let acknowledgement_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        acknowledgement_box.append(&self.tcp_acknowledgement.0); acknowledgement_box.append(&self.tcp_acknowledgement.1);
                        left_grid.attach(&(acknowledgement_box.clone()), 1, 3, 1, 1);
                    }
                }

                upper_box.append(&left_grid);
            }

            /* Middle grid */ {
                /* Middle grid. Consists of 4 rows. Each one is label - checkbox auto - text entry. */
                let middle_grid = gtk::Grid::builder().halign(gtk::Align::Center)
                    .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

                /* Middle grid labels */ {
                    /* Offset */ {
                        let data_offset = gtk::Label::builder().label("Data offset:").halign(gtk::Align::Start).build();
                        middle_grid.attach(&data_offset, 0, 0, 1, 1);
                    }
                    /* Window size */ {
                        let window = gtk::Label::builder().label("Window size:").halign(gtk::Align::Start).build();
                        middle_grid.attach(&window, 0, 1, 1, 1);
                    }
                    /* Checksum */ {
                        let checksum = gtk::Label::builder().label("Checksum:").halign(gtk::Align::Start).build();
                        middle_grid.attach(&checksum, 0, 2, 1, 1);
                    }
                    /* Urgent pointer */ {
                        let urgent = gtk::Label::builder().label("Urgent pointer:").halign(gtk::Align::Start).build();
                        middle_grid.attach(&urgent, 0, 3, 1, 1);
                    }
                }

                /* Middle grid auto-entry boxes */ {
                    /* Offset */ {
                        let offset_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        offset_box.append(&self.tcp_offset.0); offset_box.append(&self.tcp_offset.1);
                        middle_grid.attach(&(offset_box.clone()), 1, 0, 1, 1);
                    }
                    /* Window size */ {
                        let window_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        window_box.append(&self.tcp_window.0); window_box.append(&self.tcp_window.1);
                        middle_grid.attach(&(window_box.clone()), 1, 1, 1, 1);
                    }
                    /* Checksum */ {
                        let checksum_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        checksum_box.append(&self.tcp_checksum.0); checksum_box.append(&self.tcp_checksum.1);
                        middle_grid.attach(&(checksum_box.clone()), 1, 2, 1, 1);
                    }
                    /* Urgent pointer */ {
                        let urgent_ptr_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        urgent_ptr_box.append(&self.tcp_urgent.0); urgent_ptr_box.append(&self.tcp_urgent.1);
                        middle_grid.attach(&(urgent_ptr_box.clone()), 1, 3, 1, 1);
                    }
                }

                upper_box.append(&middle_grid);
            }

            /* Right grid */ {
                /* Right grid with flags. */
                let right_inner_grid = gtk::Grid::builder().halign(gtk::Align::Center)
                    .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

                /* Right grid buttons */ {
                    right_inner_grid.attach(&self.tcp_flags.0, 0, 0, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.1, 1, 0, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.2, 0, 1, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.3, 1, 1, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.4, 0, 2, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.5, 1, 2, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.6, 0, 3, 1, 1);
                    right_inner_grid.attach(&self.tcp_flags.7, 1, 3, 1, 1);
                }

                /* Right grid frame with flags check buttons. */
                let right_frame = gtk::Frame::builder().label("Flags").child(&right_inner_grid).build();

                upper_box.append(&right_frame);
            }

            main_box.append(&upper_box);
        }

        /* Lower box. */ {
            let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

            /* Data label */
            lower_box.append(&gtk::Label::new(Some("Data (Various):")));
            /* Data text entry */
            lower_box.append(&self.tcp_data);

            /* Reserved bits frame */ {
                let reserved_bits_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
                    .halign(gtk::Align::Center).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

                reserved_bits_box.append(&self.tcp_reserved_bits.0);
                reserved_bits_box.append(&self.tcp_reserved_bits.1);
                reserved_bits_box.append(&self.tcp_reserved_bits.2);
                reserved_bits_box.append(&self.tcp_reserved_bits.3);

                let reserved_bits_frame = gtk::Frame::builder().label("Reserved bits").child(&reserved_bits_box).build();
                lower_box.append(&reserved_bits_frame);
            }

            main_box.append(&lower_box);
        }

        let frame = gtk::Frame::builder().label("TCP options").build();
        frame.set_child(Some(&main_box));
        frame
    }

    // fn build_packet() -> MutableTcpPacket<'static> {
    //
    // }
}

pub struct MainWindowWidgets {
    interface_list: gtk::DropDown,

    ip_button: gtk::CheckButton,
    icmp_button: gtk::CheckButton,
    tcp_button: gtk::CheckButton,
    udp_button: gtk::CheckButton,

    src_mac_entry: gtk::Entry,
    dest_mac_entry: gtk::Entry,

    ip_widgets: IPWidgets,
    tcp_widgets: TCPWidgets
}
impl MainWindowWidgets {
    fn generate_ui(&self) -> gtk::Box {
        let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24).margin_bottom(24)
            .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        /* First section. */ {
            let section_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

            /* Initialize first section. */
            section_box.append(&gtk::Label::new(Some("Interface:")));
            section_box.append(&self.interface_list);
            section_box.append(&self.get_protocol_table());
            section_box.append(&self.ip_widgets.prepare_address_section());

            /* Add main button. */
            let main_button = gtk::Button::with_label("Collect");
            main_button.connect_clicked(move |_| {
                self.build_packet();
            });
            section_box.append(&main_button);

            container.append(&section_box);
        }

        /* Second section. */
        container.append(&self.get_mac_address_table());

        /* Third section. */
        container.append(&self.get_utility_buttons());

        /* Forth section. */
        container.append(&self.ip_widgets.prepare_options_section());

        /* Fifth section. */
        container.append(&self.tcp_widgets.prepare_ui_fields());

        container
    }
    fn get_protocol_table(&self) -> gtk::Grid {
        let protocol_table = gtk::Grid::builder().margin_start(6).margin_end(6).row_spacing(6)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(6).name("protocol-table").build();

        self.tcp_button.set_group(Some(&self.udp_button));
        self.icmp_button.set_group(Some(&self.tcp_button));
        self.ip_button.set_group(Some(&self.icmp_button));

        protocol_table.attach(&self.ip_button, 0, 0, 1, 1);
        protocol_table.attach(&self.icmp_button, 1, 0, 1, 1);
        protocol_table.attach(&self.tcp_button, 0, 1, 1, 1);
        protocol_table.attach(&self.udp_button, 1, 1, 1, 1);

        protocol_table
    }
    fn get_mac_address_table(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).halign(gtk::Align::Center)
            .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

        let source_lable = gtk::Label::builder().label("Source MAC").halign(gtk::Align::Start).build();
        grid.attach(&source_lable, 0, 0, 1, 1);
        grid.attach(&self.src_mac_entry, 1, 0, 1, 1);

        let destination_lable = gtk::Label::builder().label("Destination MAC").halign(gtk::Align::Start).build();
        grid.attach(&destination_lable, 2, 0, 1, 1);
        grid.attach(&self.dest_mac_entry, 3, 0, 1, 1);

        grid
    }
    fn get_utility_buttons(&self) -> gtk::Box {
        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

        let save = gtk::Button::with_label("Save Packet");
        main_box.append(&save); save.connect_clicked(move |_| { println!("Packet saved.") });
        let sequence = gtk::Button::with_label("Send Sequence");
        main_box.append(&sequence); sequence.connect_clicked(move |_| { println!("Sequence sent.") });
        let open_file = gtk::Button::with_label("Open File...");
        main_box.append(&open_file); open_file.connect_clicked(move |_| { println!("File opened.") });
        let delete_packet = gtk::Button::with_label("Delete Packet");
        main_box.append(&delete_packet); delete_packet.connect_clicked(move |_| { println!("Packet deleted.") });
        let delete_file = gtk::Button::with_label("Delete File");
        main_box.append(&delete_file); delete_file.connect_clicked(move |_| { println!("File deleted.") });
        let create_file = gtk::Button::with_label("Create File");
        main_box.append(&create_file); create_file.connect_clicked(move |_| { println!("File created.") });

        main_box
    }

    fn new() -> Self {
        let binding = datalink::interfaces();
        let interfaces: Vec<_> = binding.iter().map(|v| &*v.name).collect();
        Self {
            interface_list: gtk::DropDown::from_strings(&interfaces),

            ip_button: gtk::CheckButton::builder().label("IP").active(true).build(),
            icmp_button: gtk::CheckButton::with_label("ICMP"),
            tcp_button: gtk::CheckButton::with_label("TCP"),
            udp_button: gtk::CheckButton::with_label("UDP"),

            src_mac_entry: gtk::Entry::builder().placeholder_text("Source MAC").build(),
            dest_mac_entry: gtk::Entry::builder().placeholder_text("Destination MAC").build(),

            ip_widgets: IPWidgets::new(),
            tcp_widgets: TCPWidgets::new()
        }
    }
    fn build_packet(&self) {
        if self.udp_button.is_active() {
            let packet = UdpOptions::show_window();
        }
        if self.icmp_button.is_active() {
            let packet = IcmpOptions::show_window();
        }
    }
}

pub struct MainWindow {
    widgets: MainWindowWidgets,
    window: gtk::ApplicationWindow
}
impl MainWindow {
    pub(crate) fn new(app: &gtk::Application) -> Self {
        let widgets = MainWindowWidgets::new();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Network Packet Generator")
            .default_width(900)
            .default_height(500)
            .child(&widgets.generate_ui())
            .build();

        Self { widgets, window }
    }
    pub(crate) fn show(&self) { self.window.show(); }
}