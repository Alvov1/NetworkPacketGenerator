use std::net::Ipv4Addr;
use std::str::FromStr;
use glib::ObjectExt;
use gtk::{Button, CheckButton, ComboBoxText, DropDown, Entry, Label, Window};
use gtk::prelude::{ButtonExt, CheckButtonExt, EditableExt};

use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::icmp::MutableIcmpPacket;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::error_window::error;
use crate::icmp::ICMPWindow;

pub struct MyWidgets {
    pub interface_list: ComboBoxText,

    pub ip_button: CheckButton,
    pub icmp_button: CheckButton,
    pub tcp_button: CheckButton,
    pub udp_button: CheckButton,

    pub src_ip_entry: Entry,
    pub dest_ip_entry: Entry,

    pub src_mac_entry: Entry,
    pub dest_mac_entry: Entry,

    pub ip_version: (CheckButton, Entry),
    pub ip_ihl: (CheckButton, Entry),
    pub ip_type_of_service: (CheckButton, Entry),
    pub ip_header_length: (CheckButton, Entry),
    pub ip_checksum: (CheckButton, Entry),
    pub ip_packet_id: (CheckButton, Entry),
    pub ip_next_protocol: (CheckButton, Entry),
    pub ip_offset: (CheckButton, Entry),
    pub ip_ttl: (CheckButton, Entry),

    pub ip_flags: (CheckButton, CheckButton, CheckButton),

    pub tcp_source_port: (CheckButton, Entry),
    pub tcp_dest_port: (CheckButton, Entry),
    pub tcp_sequence_number: (CheckButton, Entry),
    pub tcp_acknowledgement: (CheckButton, Entry),
    pub tcp_offset: (CheckButton, Entry),
    pub tcp_window: (CheckButton, Entry),
    pub tcp_checksum: (CheckButton, Entry),
    pub tcp_urgent: (CheckButton, Entry),

    pub tcp_flags: (CheckButton, CheckButton, CheckButton,
                    CheckButton, CheckButton, CheckButton,
                    CheckButton, CheckButton),

    pub tcp_data: Entry,

    pub tcp_reserved_bits: (CheckButton, CheckButton, CheckButton, CheckButton)
}

impl MyWidgets {
    pub fn new() -> MyWidgets {
        MyWidgets {
            interface_list: ComboBoxText::new(),
            ip_button: CheckButton::builder().label("IP").active(true).build(),
            icmp_button: CheckButton::with_label("ICMP"),
            tcp_button: CheckButton::with_label("TCP"),
            udp_button: CheckButton::with_label("UDP"),

            src_ip_entry: Entry::builder().placeholder_text("Source IPv4").build(),
            dest_ip_entry: Entry::builder().placeholder_text("Destination IPv4").build(),
            src_mac_entry: Entry::builder().placeholder_text("Source MAC").build(),
            dest_mac_entry: Entry::builder().placeholder_text("Destination MAC").build(),

            ip_version: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_ihl: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_type_of_service: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_header_length: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_checksum: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_packet_id: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_next_protocol: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_offset: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            ip_ttl: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),

            ip_flags: (CheckButton::with_label("DF"),
                       CheckButton::with_label("MF"),
                       CheckButton::with_label("Reserved bit")),

            tcp_source_port: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_dest_port: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_sequence_number: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_acknowledgement: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_offset: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_window: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_checksum: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),
            tcp_urgent: (CheckButton::builder().label("Auto").active(true).build(), Entry::new()),

            tcp_flags: (CheckButton::with_label("ACK"), CheckButton::with_label("SYN"),
                        CheckButton::with_label("PSH"), CheckButton::with_label("FIN"),
                        CheckButton::with_label("RST"), CheckButton::with_label("URG"),
                        CheckButton::with_label("ECE"), CheckButton::with_label("CWR")),

            tcp_data: Entry::builder().placeholder_text("Enter data").build(),

            tcp_reserved_bits: (CheckButton::with_label("1"), CheckButton::with_label("2"),
                                CheckButton::with_label("3"), CheckButton::with_label("4"))
        }
    }

    pub fn collect(&self) -> Option<MutableEthernetPacket> {
        let mut buffer: Option<&[u8]> = None;
        if self.icmp_button.is_active() {
            ICMPWindow::show();

        }
        let mut ip_packet = MutableIpv4Packet::owned(vec![0u8; MutableIpv4Packet::minimum_packet_size()]).unwrap();

        /* Set ip addresses. */ {
            match Ipv4Addr::from_str(&*self.src_ip_entry.text()) {
                Ok(address) => ip_packet.set_source(address),
                Err(_) => {
                    error("Incorrect source ip address.");
                    return None;
                }
            }
            match Ipv4Addr::from_str(&*self.dest_ip_entry.text()) {
                Ok(address) => ip_packet.set_destination(address),
                Err(_) => {
                    error("Incorrect destination ip address.");
                    return None;
                }
            }
        }

        if self.tcp_button.is_active() {
            let packet = MutableTcpPacket::owned(vec![0u8; MutableTcpPacket::minimum_packet_size()]);
        }

        if self.udp_button.is_active() {
            let packet = MutableUdpPacket::owned(vec![0u8; MutableUdpPacket::minimum_packet_size()]);
        }

        panic!("Unknown protocol value.");
    }
}