use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4::{Ipv4Option, MutableIpv4Packet};
use std::net::Ipv4Addr;

enum Segment {
    TCPPacket,
    UDPPacket,
    ICMPPacket
}
impl Segment {
    fn get_protocol(&self) -> IpNextHeaderProtocol {
        static TCP_PROTOCOL_NUMBER: u8 = 6;
        static UDP_PROTOCOL_NUMBER: u8 = 17;
        static ICMP_PROTOCOL_NUMBER: u8 = 1;
        match self {
            TCPPacket => IpNextHeaderProtocol::new(TCP_PROTOCOL_NUMBER),
            UDPPacket => IpNextHeaderProtocol::new(UDP_PROTOCOL_NUMBER),
            IMCPPacket => IpNextHeaderProtocol::new(ICMP_PROTOCOL_NUMBER)
        }
    }
    fn to_bytes(&self) -> &[u8] {
        &[0u8; 4]
    }
}

struct IPv4Header {
    version: u8,            /* 4 bits. */
    ihl: u8,                /* 4 bits. */
    dscp: u8,               /* 6 bits. */
    ecn: u8,                /* 2 bits. */
    length: u16,            /* 16 bits. */
    identification: u16,    /* 16 bits. */
    flags: u8,              /* 3 bits. */
    offset: u16,            /* 13 bits. */
    ttl: u8,                /* 8 bits. */
    protocol: u8,           /* 8 bits. */
    header_checksum: u16,   /* 16 bits. */
    source_ip: Ipv4Addr,
    destination_ip: Ipv4Addr,
    options: Option<Vec<Ipv4Option>>
}
impl IPv4Header {}

pub(crate) struct IPv4Packet {
    header: IPv4Header,
    data: Segment
}
impl IPv4Packet {
    fn new(header: IPv4Header, data: Segment) -> IPv4Packet {
        IPv4Packet { header, data }
    }
    fn to_bytes(&self) -> &[u8] {
        let header = &self.header;
        let data = &self.data;

        /* TODO: Which size? */
        let mut vec: Vec<u8> = vec![0; 42];
        let mut packet = MutableIpv4Packet::new(&mut vec).unwrap();

        packet.set_version(header.version);
        /* TODO: ihl - auto? */
        packet.set_dscp(header.dscp);
        packet.set_ecn(header.ecn);
        packet.set_total_length(header.length);
        packet.set_identification(header.identification);
        packet.set_flags(header.flags);
        packet.set_fragment_offset(header.offset);
        packet.set_ttl(header.ttl);
        packet.set_next_level_protocol(data.get_protocol());
        packet.set_checksum(header.header_checksum);
        packet.set_source(header.source_ip);
        packet.set_destination(header.destination_ip);
        if header.options.is_some() {
            packet.set_options(&header.options.as_ref().unwrap());
        }
        packet.set_payload(data.to_bytes());
        packet.set_header_length(0); /* TODO: Set header length. */
        packet.packet()
    }
}