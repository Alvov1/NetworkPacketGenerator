extern crate packet_builder;
extern crate pnet;

use packet_builder::payload::PayloadData;
use packet_builder::*;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, DataLinkSender, NetworkInterface};
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::tcp::TcpFlags;
use pnet::packet::tcp::TcpOption;
use pnet::packet::Packet;
use pnet::util::MacAddr;
use std::env;
use glib::GString;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, gdk};
use crate::gdk::gio;
use crate::gdk::glib::clone;

fn list_ifaces(interfaces: &Vec<NetworkInterface>) {
    for iface in interfaces {
        println!("{}", iface.name)
    }
}

fn send_icmp(sender: &mut Box<dyn DataLinkSender>) {
    // Generate a destination unreachable ICMP packet
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_source => MacAddr(10,1,1,1,1,1)}) /
             ipv4({set_source => ipv4addr!("127.0.0.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             icmp_dest_unreach({set_icmp_type => IcmpTypes::DestinationUnreachable}) /
             ipv4({set_source => ipv4addr!("10.8.0.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             udp({set_source => 53, set_destination => 5353}) /
             payload({"hello".to_string().into_bytes()})
        );

    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}

fn send_tcp(sender: &mut Box<dyn DataLinkSender>) {
    // Generate a TCP PSH|ACK packet with data
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_destination => MacAddr(1,2,3,4,5,6), set_source => MacAddr(10,1,1,1,1,1)}) /
             ipv4({set_source => ipv4addr!("192.168.1.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             tcp({set_source => 43455, set_destination => 80, set_flags => (TcpFlags::PSH | TcpFlags::ACK)}) /
             payload({"hello".to_string().into_bytes()})
        );

    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}

fn send_udp(sender: &mut Box<dyn DataLinkSender>) {
    // Generate a UDP packet with data
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_destination => MacAddr(1,2,3,4,5,6), set_source => MacAddr(10,1,1,1,1,1)}) /
             ipv4({set_source => ipv4addr!("127.0.0.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             udp({set_source => 12312, set_destination => 143}) /
             payload({"hello".to_string().into_bytes()})
        );
    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}

fn send_arp(sender: &mut Box<dyn DataLinkSender>) {
    // Generate an ARP request
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_destination => MacAddr(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF)}) /
             arp({set_target_proto_addr => ipv4addr!("192.168.1.1"), set_sender_proto_addr => ipv4addr!("192.168.1.245")})
        );
    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}

fn send_tcp_2(sender: &mut Box<dyn DataLinkSender>) {
    // Generate a TCP SYN packet with mss and wscale options specified over VLAN ID 10
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_destination => MacAddr(1,2,3,4,5,6), set_source => MacAddr(10,1,1,1,1,1)}) /
             vlan({set_vlan_identifier => 10}) /
             ipv4({set_source => ipv4addr!("192.168.1.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             tcp({set_source => 43455, set_destination => 80, set_options => &[TcpOption::mss(1200), TcpOption::wscale(2)]}) /
             payload({[0; 0]})
        );

    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}

fn send_icmp_2(sender: &mut Box<dyn DataLinkSender>) {
    // Generate an ICMP echo request
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
             pkt_buf,
             ether({set_destination => MacAddr(1,2,3,4,5,6), set_source => MacAddr(10,1,1,1,1,1)}) /
             ipv4({set_source => ipv4addr!("127.0.0.1"), set_destination => ipv4addr!("127.0.0.1") }) /
             icmp_echo_req({set_icmp_type => IcmpTypes::EchoRequest}) /
             payload({"hello".to_string().into_bytes()})
        );

    sender.send_to(pkt.packet(), None).unwrap().unwrap();
}


fn send_packet(iface: &NetworkInterface) {
    println!("Package is sent through interface {}.", iface.name);
}

fn generate_interface_protocol_section() -> gtk::Box {
    let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_top(24).margin_bottom(24)
        .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

    /* Left "Interface:" label. */
    {
        let interfaces_title = gtk::Label::builder().label("Interface")
            .halign(gtk::Align::Start).build();
        interfaces_title.add_css_class("ifaces-title");
        common_box.append(&interfaces_title);
    }

    /* Dropdown list in the middle. */
    {
        let interfaces = datalink::interfaces();
        let iface_list = gtk::ComboBoxText::new();
        interfaces.iter().for_each(|iface| {
            iface_list.append(Some(&*iface.name), &*iface.name);
        });
        iface_list.set_active(Some(0));
        common_box.append(&iface_list);
    }

    /* Protocol grid table. */
    {
        let protocol_table = gtk::Grid::builder().margin_start(6).margin_end(6)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).row_spacing(6).column_spacing(6).build();

        let ip_button = gtk::CheckButton::with_label("IP");
        let icmp_button = gtk::CheckButton::with_label("ICMP");
        let tcp_button = gtk::CheckButton::with_label("TCP");
        let udp_button = gtk::CheckButton::with_label("UDP");

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
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

        grid.attach(&gtk::Label::new(Some("Source IP")), 0, 0, 1, 1);
        grid.attach(&gtk::Entry::builder().placeholder_text("Source IPv4").build(), 1, 0, 1, 1);
        grid.attach(&gtk::Label::new(Some("Destination IP")), 0, 1, 1, 1);
        grid.attach(&gtk::Entry::builder().placeholder_text("Destination IPv4").build(), 1, 1, 1, 1);

        common_box.append(&grid);
    }

    /* Sending button on the right. */
    {
        let main_button = gtk::Button::with_label("Generate");
        main_button.connect_clicked(move |button| {
            println!("Packet is sent.");
        });
        common_box.append(&main_button);
    }

    common_box
}

fn generate_address_table() -> gtk::Grid {
    let grid = gtk::Grid::builder().margin_start(24).margin_end(24).margin_top(24).margin_bottom(24)
        .halign(gtk::Align::Center).valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

    let source_lable = gtk::Label::new(Some("Source MAC")); source_lable.set_halign(gtk::Align::Start);
    grid.attach(&source_lable, 0, 0, 1, 1);
    grid.attach(&gtk::Entry::builder().placeholder_text("Source MAC").build(), 1, 0, 1, 1);
    let destination_lable = gtk::Label::new(Some("Destination MAC")); destination_lable.set_halign(gtk::Align::Start);
    grid.attach(&destination_lable, 2, 0, 1, 1);
    grid.attach(&gtk::Entry::builder().placeholder_text("Destination MAC").build(), 3, 0, 1, 1);

    grid
}

fn generate_ip_settings_section() -> gtk::Frame {
    /* Left grid. Five rows. Each row consists of label, checkbox 'auto', text entry. */
    let left_grid = gtk::Grid::builder().margin_start(24).margin_end(24).margin_top(24).margin_bottom(24)
        .halign(gtk::Align::Center).valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();
    let version = gtk::Label::new(Some("Version:")); version.set_halign(gtk::Align::Start);
    left_grid.attach(&version, 0, 0, 1, 1);
    let ihl = gtk::Label::new(Some("IHL:")); ihl.set_halign(gtk::Align::Start);
    left_grid.attach(&ihl, 0, 1, 1, 1);
    let tos = gtk::Label::new(Some("Type of Service:")); tos.set_halign(gtk::Align::Start);
    left_grid.attach(&tos, 0, 2, 1, 1);
    let length = gtk::Label::new(Some("Header Length:")); length.set_halign(gtk::Align::Start);
    left_grid.attach(&length, 0, 3, 1, 1);
    let checksum = gtk::Label::new(Some("Header Checksum:")); checksum.set_halign(gtk::Align::Start);
    left_grid.attach(&checksum, 0, 4, 1, 1);
    for row in 0..5 {
        let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        auto_entry_box.append(&gtk::CheckButton::builder().label("Auto").build());
        auto_entry_box.append(&gtk::Entry::new());
        left_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
    }

    /* Right grid. Four rows. Each row consists of label, checkbox 'auto', text entry. */
    let right_grid = gtk::Grid::builder().margin_top(24).halign(gtk::Align::Center)
        .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();
    let packet_id = gtk::Label::new(Some("Packet ID:")); packet_id.set_halign(gtk::Align::Start);
    right_grid.attach(&packet_id, 0, 0, 1, 1);
    let protocol = gtk::Label::new(Some("Protocol:")); protocol.set_halign(gtk::Align::Start);
    right_grid.attach(&protocol, 0, 1, 1, 1);
    let offset= gtk::Label::new(Some("Fragment offset:")); offset.set_halign(gtk::Align::Start);
    right_grid.attach(&offset, 0, 2, 1, 1);
    let ttl = gtk::Label::new(Some("Time to Live:")); ttl.set_halign(gtk::Align::Start);
    right_grid.attach(&ttl, 0, 3, 1, 1);
    for row in 0..4 {
        let auto_entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        auto_entry_box.append(&gtk::CheckButton::builder().label("Auto").build());
        auto_entry_box.append(&gtk::Entry::new());
        right_grid.attach(&(auto_entry_box.clone()), 1, row, 1, 1);
    }

    /* Right box in the bottom. Specifies flags DF, MF and reserved bit. */
    let right_down_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_bottom(24)
        .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
    right_down_box.append(&gtk::Label::new(Some("Flags:")));
    right_down_box.append(&gtk::CheckButton::builder().label("DF").build());
    right_down_box.append(&gtk::CheckButton::builder().label("MF").build());
    right_down_box.append(&gtk::CheckButton::builder().label("Reserved bit").build());

    /* Right box. Gathers right grid and bottom box together. */
    let right_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24)
        .margin_bottom(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
    right_box.append(&right_grid); right_box.append(&right_down_box);

    /* Result box. */
    let common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_top(24).margin_bottom(24)
        .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).name("IP Settings").build();
    common_box.append(&left_grid); common_box.append(&right_box); common_box.create_pango_layout(Some("IP Settings"));

    let box_frame = gtk::Frame::new(Some("IP Settings"));
    box_frame.set_child(Some(&common_box));

    box_frame
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.Project")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Network Packet Generator")
            .default_width(900)
            .default_height(500)
            .build();

        let main_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .spacing(24)
            .build();

        let first_section = generate_interface_protocol_section();
        main_container.append(&first_section);

        let third_section = generate_address_table();
        main_container.append(&third_section);

        let new_section = generate_ip_settings_section();
        main_container.append(&new_section);

        window.set_child(Some(&main_container));
        window.show();
    });

    application.run();
}