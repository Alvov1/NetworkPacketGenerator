use std::net::Ipv4Addr;
use pnet::datalink::MacAddr;
use pnet::packet::ipv4::{Ipv4, Ipv4Option, MutableIpv4Packet};
use pnet::packet::tcp::{MutableTcpPacket, TcpOption};
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::icmp::MutableIcmpPacket;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ethernet::MutableEthernetPacket;

pub struct IPv4Packet<'a> {
    packet: MutableIpv4Packet<'a>
}
impl IPv4Packet<'static> {
    pub fn new() -> IPv4Packet<'static> {
        IPv4Packet {
            packet: MutableIpv4Packet::owned(vec![0u8; MutableIpv4Packet::minimum_packet_size()]).unwrap()
        }
    }
    pub fn set_version(&mut self, version: u8) {
        self.packet.set_version(version);
    }
    pub fn auto_version(&mut self) {
        self.packet.set_version(4u8);
    }
    pub fn set_header_length(&mut self, length: u8) {
        self.packet.set_header_length(length);
    }
    pub fn auto_header_length(&mut self) {
        self.packet.set_header_length(self.packet.get_header_length());
    }
    pub fn set_dscp(&mut self, dscp: u8) {
        self.packet.set_dscp(dscp);
    }
    pub fn auto_dscp(&mut self) {
        self.packet.set_dscp(32u8);
    }
    pub fn set_ecn(&mut self, ecn: u8) {
        self.packet.set_ecn(ecn);
    }
    pub fn auto_ecn(&mut self) {
        self.packet.set_ecn(0u8);
    }
    pub fn set_packet_length(&mut self, length: u16) {
        self.packet.set_total_length(length);
    }
    pub fn auto_packet_length(&mut self) {
        self.packet.set_total_length(self.packet.get_total_length());
    }
    pub fn set_identifier(&mut self, identifier: u16) {
        self.packet.set_identification(identifier);
    }
    pub fn auto_identifier(&mut self) {
        self.packet.set_identification(0);
    }
    pub fn set_flags(&mut self, flags: u8) {
        self.packet.set_flags(flags);
    }
    pub fn auto_flags(&mut self) {
        self.packet.set_flags(2u8);
    }
    pub fn set_offset(&mut self, offset: u16) {
        self.packet.set_fragment_offset(offset);
    }
    pub fn auto_offset(&mut self) {
        self.packet.set_fragment_offset(0u16);
    }
    pub fn set_ttl(&mut self, ttl: u8) {
        self.packet.set_ttl(ttl)
    }
    pub fn auto_ttl(&mut self) {
        self.packet.set_ttl(64u8);
    }
    pub fn set_protocol(&mut self, protocol: IpNextHeaderProtocol) {
        self.packet.set_next_level_protocol(protocol);
    }
    pub fn auto_protocol(&mut self) {
        self.packet.set_next_level_protocol(IpNextHeaderProtocol::new(0));
    }
    pub fn set_checksum(&mut self, checksum: u16) {
        self.packet.set_checksum(checksum);
    }
    pub fn auto_checksum(&mut self) {
        self.packet.set_checksum(pnet::packet::ipv4::checksum(&self.packet.to_immutable()));
    }
    pub fn set_source(&mut self, address: Ipv4Addr) {
        self.packet.set_source(address);
    }
    pub fn set_destination(&mut self, address: Ipv4Addr) {
        self.packet.set_destination(address);
    }
    pub fn set_options(&mut self, options: &[Ipv4Option]) {
        self.packet.set_options(options);
    }
    pub fn set_payload(&mut self, payload: &[u8]) {
        self.packet.set_payload(payload);
    }
}

pub struct TCPPacket<'a> {
    packet: MutableTcpPacket<'a>
}
impl TCPPacket<'static> {
    pub fn new() -> TCPPacket<'static> {
        TCPPacket {
            packet: MutableTcpPacket::owned(vec![0u8; MutableTcpPacket::minimum_packet_size()]).unwrap()
        }
    }
    pub fn set_source_port(&mut self, port: u16) {
        self.packet.set_source(port);
    }
    pub fn auro_source_port(&mut self) {
        self.packet.set_source(0u16);
    }
    pub fn set_destination_port(&mut self, port: u16) {
        self.packet.set_destination(port);
    }
    pub fn auto_destination_port(&mut self) {
        self.packet.set_destination(0u16);
    }
    pub fn set_sn(&mut self, sn: u32) {
        self.packet.set_sequence(sn);
    }
    pub fn auto_sn(&mut self) {
        self.packet.set_sequence(0u32);
    }
    pub fn set_ack_sn(&mut self, ack_sn: u32) {
        self.packet.set_acknowledgement(ack_sn);
    }
    pub fn auto_ack_sn(&mut self) {
        self.packet.set_acknowledgement(0u32);
    }
    pub fn set_offset(&mut self, offset: u8) {
        self.packet.set_data_offset(offset);
    }
    pub fn auto_offset(&mut self) {
        self.packet.set_data_offset(0u8)
    }
    pub fn set_reserved(&mut self, reserved: u8) {
        self.packet.set_reserved(reserved);
    }
    pub fn auto_reserved(&mut self) {
        self.packet.set_reserved(0u8);
    }
    pub fn set_flags(&mut self, flags: u16) {
        self.packet.set_flags(flags);
    }
    pub fn auto_flags(&mut self) {
        self.packet.set_flags(0u16);
    }
    pub fn set_window(&mut self, window: u16) {
        self.packet.set_window(window);
    }
    pub fn auto_window(&mut self) {
        self.packet.set_window(0u16);
    }
    pub fn set_checksum(&mut self, checksum: u16) {
        self.packet.set_checksum(checksum);
    }
    pub fn auto_checksum(&mut self, source: &Ipv4Addr, destination: &Ipv4Addr) {
        self.packet.set_checksum(pnet::packet::tcp::ipv4_checksum(&self.packet.to_immutable(), source, destination));
    }
    pub fn set_urgent_ptr(&mut self, pointer: u16) {
        self.packet.set_urgent_ptr(pointer);
    }
    pub fn auto_urgent_ptr(&mut self) {
        self.packet.set_urgent_ptr(0u16);
    }
    pub fn set_options(&mut self, options: &[TcpOption]) {
        self.packet.set_options(options);
    }
    pub fn set_payload(&mut self, payload: &[u8]) {
        self.packet.set_payload(payload);
    }
}

pub struct NetworkPacket<'a> {
    ipv4: IPv4Packet<'a>,
    tcp: TCPPacket<'a>
}
impl NetworkPacket<'static> {
    pub(crate) fn new() -> NetworkPacket<'static> {
        NetworkPacket {
            ipv4: IPv4Packet::new(),
            tcp: TCPPacket::new()
        }
    }
}

pub struct DataLinkFrame<'a> {
    interface: String,
    frame: MutableEthernetPacket<'a>,
    protocol: IpNextHeaderProtocol,
    packet: NetworkPacket<'a>
}
impl DataLinkFrame<'static> {
    pub(crate) fn new() -> DataLinkFrame<'static> {
        DataLinkFrame {
            interface: "".to_string(),
            frame: MutableEthernetPacket::owned(vec![0u8; MutableEthernetPacket::minimum_packet_size()]).unwrap(),
            protocol: IpNextHeaderProtocol::new(0),
            packet: NetworkPacket::new()
        }
    }
    pub fn set_iface(&mut self, iface_name: &str) {
        self.interface = iface_name.to_string()
    }
    pub fn set_protocol(&mut self, protocol: IpNextHeaderProtocol) {
        self.protocol = protocol
    }
    pub fn set_source(&mut self, address: MacAddr) {
        self.frame.set_source(address);
    }
    pub fn set_destination(&mut self, address: MacAddr) {
        self.frame.set_destination(address);
    }
}