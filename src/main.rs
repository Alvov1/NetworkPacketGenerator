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

fn generate_first_section() -> gtk::Box {
    let result = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    /* Left "Interface:" label. */
    let interfaces_title = gtk::Label::builder()
        .label("Interface:")
        .halign(gtk::Align::Start)
        .build();
    interfaces_title.add_css_class("ifaces-title");
    result.append(&interfaces_title);

    /* Dropdown list in the middle. */
    let interfaces = datalink::interfaces();
    let iface_list = gtk::ComboBoxText::new();
    interfaces.iter().for_each(|iface| {
        iface_list.append(Some(&*iface.name), &*iface.name);
    });
    iface_list.set_active(Some(0));
    result.append(&iface_list);

    /* Sending button on the right. */
    let main_button = gtk::Button::with_label("Generate");
    main_button.connect_clicked(move |button| {
        let iface_name = iface_list.active_text().unwrap();
        for iface in &interfaces {
            if iface.name == iface_name { send_packet(iface); break; }
        }
    });
    result.append(&main_button);

    result
}
fn generate_second_section() -> gtk::Box {
    let result = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    let ip_button = gtk::CheckButton::with_label("IP");
    let icmp_button = gtk::CheckButton::with_label("ICMP");
    let tcp_button = gtk::CheckButton::with_label("TCP");
    let udp_button = gtk::CheckButton::with_label("UDP");

    tcp_button.set_group(Some(&udp_button));
    icmp_button.set_group(Some(&tcp_button));
    ip_button.set_group(Some(&icmp_button));

    result.append(&ip_button);
    result.append(&icmp_button);
    result.append(&tcp_button);
    result.append(&udp_button);

    result
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.Project")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Network Packet Generator")
            .default_width(350)
            .default_height(70)
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

        let first_section = generate_first_section();
        main_container.append(&first_section);

        let second_section = generate_second_section();
        main_container.append(&second_section);

        window.set_child(Some(&main_container));
        window.show();
    });

    application.run();
}