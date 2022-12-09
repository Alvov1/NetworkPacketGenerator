use gtk::prelude::*;

use rand::Rng;
use std::net::Ipv4Addr;
use pnet::packet::Packet;
use pnet::packet::FromPacket;
use pnet::packet::tcp::TcpOption;
use pnet::packet::tcp::TcpOptionNumbers;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::tcp::MutableTcpOptionPacket;

use crate::error_window::error;

pub(crate) struct TCPWidgets {
    source_port: (gtk::CheckButton, gtk::Entry),
    dest_port: (gtk::CheckButton, gtk::Entry),
    sequence_number: (gtk::CheckButton, gtk::Entry),
    acknowledgement: (gtk::CheckButton, gtk::Entry),
    offset: (gtk::CheckButton, gtk::Entry),
    window: (gtk::CheckButton, gtk::Entry),
    checksum: (gtk::CheckButton, gtk::Entry),
    urgent: (gtk::CheckButton, gtk::Entry),

    /* ACK SYN PSH FIN RST URG ECE CWR */
    flags: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton,
            gtk::CheckButton, gtk::CheckButton, gtk::CheckButton,
            gtk::CheckButton, gtk::CheckButton, gtk::CheckButton),

    data: gtk::Entry,

    reserved_bits: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton),

    options: gtk::Entry
}
impl TCPWidgets {
    pub(crate) fn new() -> Self {
        Self {
            source_port: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Port").build()),
            dest_port: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Port").build()),
            sequence_number: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Sequence number").build()),
            acknowledgement: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Acknowledgement").build()),
            offset: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Data offset").build()),
            window: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Window size").build()),
            checksum: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Checksum").build()),
            urgent: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Urgent pointer").build()),

            flags: (gtk::CheckButton::builder().label("NS (AE)").halign(gtk::Align::Center).build(),
                    gtk::CheckButton::with_label("ACK"), gtk::CheckButton::with_label("SYN"),
                    gtk::CheckButton::with_label("PSH"), gtk::CheckButton::with_label("FIN"),
                    gtk::CheckButton::with_label("RST"), gtk::CheckButton::with_label("URG"),
                    gtk::CheckButton::with_label("ECE"), gtk::CheckButton::with_label("CWR")),

            data: gtk::Entry::builder().placeholder_text("Enter data").margin_end(6).margin_start(6).margin_top(6).margin_bottom(6).build(),

            reserved_bits: (gtk::CheckButton::with_label("1"), gtk::CheckButton::with_label("2"), gtk::CheckButton::with_label("3")),
            options: gtk::Entry::builder().placeholder_text("Option 1, Option 2 ...").margin_end(6).margin_start(6).margin_top(6).margin_bottom(6).build()
        }
    }

    pub(crate) fn prepare_ui_fields(&self) -> gtk::Frame {
        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

        /* Upper box. */ {
            let upper_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(12).margin_end(12).valign(gtk::Align::Center).spacing(24).build();

            /* Left grid */ {
                /* Left grid. Consists of 4 rows. Each one is label - checkbox auto - text entry. */
                let left_grid = gtk::Grid::builder().margin_start(12).margin_end(12).row_spacing(24)
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
                        source_port_box.append(&self.source_port.0); source_port_box.append(&self.source_port.1);
                        left_grid.attach(&(source_port_box.clone()), 1, 0, 1, 1);
                    }
                    /* Destination port */ {
                        let destination_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        destination_box.append(&self.dest_port.0); destination_box.append(&self.dest_port.1);
                        left_grid.attach(&(destination_box.clone()), 1, 1, 1, 1);
                    }
                    /* Sequence number */ {
                        let sequence_number_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        sequence_number_box.append(&self.sequence_number.0); sequence_number_box.append(&self.sequence_number.1);
                        left_grid.attach(&(sequence_number_box.clone()), 1, 2, 1, 1);
                    }
                    /* Acknowledgement */ {
                        let acknowledgement_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        acknowledgement_box.append(&self.acknowledgement.0); acknowledgement_box.append(&self.acknowledgement.1);
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
                        offset_box.append(&self.offset.0); offset_box.append(&self.offset.1);
                        middle_grid.attach(&(offset_box.clone()), 1, 0, 1, 1);
                    }
                    /* Window size */ {
                        let window_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        window_box.append(&self.window.0); window_box.append(&self.window.1);
                        middle_grid.attach(&(window_box.clone()), 1, 1, 1, 1);
                    }
                    /* Checksum */ {
                        let checksum_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        checksum_box.append(&self.checksum.0); checksum_box.append(&self.checksum.1);
                        middle_grid.attach(&(checksum_box.clone()), 1, 2, 1, 1);
                    }
                    /* Urgent pointer */ {
                        let urgent_ptr_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        urgent_ptr_box.append(&self.urgent.0); urgent_ptr_box.append(&self.urgent.1);
                        middle_grid.attach(&(urgent_ptr_box.clone()), 1, 3, 1, 1);
                    }
                }

                upper_box.append(&middle_grid);
            }

            /* Right grid */ {
                /* Right grid with flags. */
                let right_inner_grid = gtk::Grid::builder().halign(gtk::Align::Center)
                    .valign(gtk::Align::Center).row_spacing(16).column_spacing(24).build();

                /* Right grid buttons */ {
                    right_inner_grid.attach(&self.flags.1, 0, 0, 1, 1);
                    right_inner_grid.attach(&self.flags.2, 1, 0, 1, 1);
                    right_inner_grid.attach(&self.flags.3, 0, 1, 1, 1);
                    right_inner_grid.attach(&self.flags.4, 1, 1, 1, 1);
                    right_inner_grid.attach(&self.flags.5, 0, 2, 1, 1);
                    right_inner_grid.attach(&self.flags.6, 1, 2, 1, 1);
                    right_inner_grid.attach(&self.flags.7, 0, 3, 1, 1);
                    right_inner_grid.attach(&self.flags.8, 1, 3, 1, 1);
                }

                let right_inner_box = gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(16)
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .build();
                right_inner_box.append(&right_inner_grid);
                right_inner_box.append(&self.flags.0);

                /* Right grid frame with flags check buttons. */
                let right_frame = gtk::Frame::builder().label("Flags").child(&right_inner_box).build();

                upper_box.append(&right_frame);
            }

            main_box.append(&upper_box);
        }

        /* Lower box. */ {
            let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(30).build();

            /* Options */ {
                let options_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(6).build();
                options_box.append(&gtk::Label::new(Some("Options: ")));
                options_box.append(&self.options);
                lower_box.append(&options_box);
            }

            /* Reserved bits frame */ {
                let reserved_bits_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
                    .halign(gtk::Align::Center).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

                reserved_bits_box.append(&self.reserved_bits.0);
                reserved_bits_box.append(&self.reserved_bits.1);
                reserved_bits_box.append(&self.reserved_bits.2);

                let reserved_bits_frame = gtk::Frame::builder().label("Reserved bits").child(&reserved_bits_box).build();
                lower_box.append(&reserved_bits_frame);
            }

            /* Data */ {
                let data_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(6).build();
                data_box.append(&gtk::Label::new(Some("Data (Various):")));
                data_box.append(&self.data);
                lower_box.append(&data_box);
            }

            main_box.append(&lower_box);
        }

        let frame = gtk::Frame::builder().label("TCP options").build();
        frame.set_child(Some(&main_box));
        frame
    }

    fn get_flags(&self) -> u16 {
        let mut result = 0u16;

        /* NS ACK SYN PSH FIN RST URG ECE CWR */
        if self.flags.0.is_active() { result |= pnet::packet::tcp::TcpFlags::NS; }
        if self.flags.1.is_active() { result |= pnet::packet::tcp::TcpFlags::ACK; }
        if self.flags.2.is_active() { result |= pnet::packet::tcp::TcpFlags::SYN; }
        if self.flags.3.is_active() { result |= pnet::packet::tcp::TcpFlags::PSH; }
        if self.flags.4.is_active() { result |= pnet::packet::tcp::TcpFlags::FIN; }
        if self.flags.5.is_active() { result |= pnet::packet::tcp::TcpFlags::RST; }
        if self.flags.6.is_active() { result |= pnet::packet::tcp::TcpFlags::URG; }
        if self.flags.7.is_active() { result |= pnet::packet::tcp::TcpFlags::ECE; }
        if self.flags.8.is_active() { result |= pnet::packet::tcp::TcpFlags::CWR; }

        result
    }
    fn get_options(&self) -> Option<Vec<TcpOption>> {
        let mut options: Vec<TcpOption> = Vec::new();
        for option in self.options.text().split(',').map(|v| v.trim()) {
            match option {
                "EOL" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::EOL);
                    options.push(packet.from_packet());
                },
                "MSS" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::MSS);
                    options.push(packet.from_packet());
                },
                "NOP" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::NOP);
                    options.push(packet.from_packet());
                },
                "SACK" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::SACK);
                    options.push(packet.from_packet());
                },
                "SACK_PERMITTED" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::SACK_PERMITTED);
                    options.push(packet.from_packet());
                },
                "TIMESTAMPS" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::TIMESTAMPS);
                    options.push(packet.from_packet());
                },
                "WSCALE" => {
                    let mut packet = MutableTcpOptionPacket::owned(vec![0u8; MutableTcpOptionPacket::minimum_packet_size()]).unwrap();
                    packet.set_number(TcpOptionNumbers::WSCALE);
                    options.push(packet.from_packet());
                },
                _ => { }
            }
        }
        return Some(options);
    }
    pub(crate) fn build_packet(&self, addresses: (Ipv4Addr, Ipv4Addr)) -> Option<Vec<u8>> {
        let packet_size = MutableTcpPacket::minimum_packet_size() + self.data.text().len();

        let mut packet = MutableTcpPacket::owned(vec![0u8; packet_size]).unwrap();
        packet.set_payload(self.data.text().as_bytes());

        if self.source_port.0.is_active() {
            let mut rng = rand::thread_rng();
            packet.set_source(rng.gen_range(49152..65535));
        } else {
            match self.source_port.1.text().parse::<u16>() {
                Ok(value) => packet.set_source(value),
                Err(_) => { error("Bad tcp source port number"); return None; }
            }
        }

        if self.dest_port.0.is_active() {
            let mut rng = rand::thread_rng();
            packet.set_destination(rng.gen_range(49152..65535));
        } else {
            match self.dest_port.1.text().parse::<u16>() {
                Ok(value) => packet.set_destination(value),
                Err(_) => { error("Bad tcp destination port number"); return None; }
            }
        }

        if self.sequence_number.0.is_active() {
            packet.set_sequence(0)
        } else {
            match self.sequence_number.1.text().parse::<u32>() {
                Ok(value) => packet.set_sequence(value),
                Err(_) => { error("Bad tcp sequence number"); return None; }
            }
        }

        if self.acknowledgement.0.is_active() {
            packet.set_acknowledgement(0)
        } else {
            match self.acknowledgement.1.text().parse::<u32>() {
                Ok(value) => packet.set_acknowledgement(value),
                Err(_) => { error("Bad tcp acknowledgement number"); return None; }
            }
        }

        if self.offset.0.is_active() {
            packet.set_data_offset(5 as u8)
        } else {
            match self.offset.1.text().parse::<u8>() {
                Ok(value) => packet.set_data_offset(value),
                Err(_) => { error("Bad tcp fragment offset number"); return None; }
            }
        }

        let mut reserved = 0u8;
        if self.reserved_bits.0.is_active() { reserved |= 0b0000_0001; }
        if self.reserved_bits.1.is_active() { reserved |= 0b0000_0010; }
        if self.reserved_bits.2.is_active() { reserved |= 0b0000_0100; }
        packet.set_reserved(reserved);

        packet.set_flags(self.get_flags());

        if self.window.0.is_active() {
            packet.set_window(0)
        } else {
            match self.window.1.text().parse::<u16>() {
                Ok(value) => packet.set_window(value),
                Err(_) => { error("Bad tcp window size number"); return None; }
            }
        }

        if self.urgent.0.is_active() {
            packet.set_urgent_ptr(0)
        } else {
            match self.urgent.1.text().parse::<u16>() {
                Ok(value) => packet.set_urgent_ptr(value),
                Err(_) => { error("Bad tcp urgent pointer number"); return None; }
            }
        }

        match self.get_options() {
            Some(options) => packet.set_options(&options),
            None => { error("Bad tcp options value"); return None; }
        }

        if self.checksum.0.is_active() {
            packet.set_checksum(pnet::packet::tcp::ipv4_checksum(
                &packet.to_immutable(), &addresses.0, &addresses.1));
        } else {
            match self.checksum.1.text().parse::<u16>() {
                Ok(value) => packet.set_checksum(value),
                Err(_) => { error("Bad tcp checksum value"); return None; }
            }
        }

        return Some(Vec::from(packet.packet()));
    }
    pub(crate) fn give_payload(&self) -> Option<Vec<u8>> {
        if self.data.text().is_empty() {
            return None;
        }

        return Some(self.data.text().bytes().collect())
    }
}
