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

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, gdk};

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

fn choose_iface() -> NetworkInterface {
    let interfaces = datalink::interfaces();
    for interface in &interfaces {
        let name = &interface.name;
        let mac = interface.mac.unwrap_or(MacAddr(0, 0, 0, 0, 0, 0));
        let desc = &interface.description;
        println!("{}: {}, {}", name, desc, mac);
    }

    interfaces.into_iter().last().unwrap_or_else(|| panic!("Iterating over interfaces failed."))
}

fn main() {
    // choose_iface();

    // //trace_macros!(true);
    // let if_name = env::args()
    //     .nth(1)
    //     .expect("Usage: ./sendpacket <interface name>");
    //
    // let interfaces = datalink::interfaces();
    // let interface = interfaces
    //     .into_iter()
    //     .filter(|iface: &NetworkInterface| iface.name == if_name)
    //     .next()
    //     .unwrap_or_else(|| panic!("No such network interface: {}", if_name));
    // //
    // let (mut sender, mut _receiver) = match datalink::channel(&interface, Default::default()) {
    //     Ok(Ethernet(tx, rx)) => (tx, rx),
    //     Ok(_) => panic!("packetdump: unhandled channel type"),
    //     Err(e) => panic!("packetdump: unable to create channel: {}", e),
    // };

    let application = Application::builder()
        .application_id("com.example.Project")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.show();
    });

    application.run();
}