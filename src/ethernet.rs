use pnet::packet::ethernet::{ MutableEthernetPacket };
use pnet::datalink::DataLinkSender;
use pnet::packet::Packet;
use pnet::util::MacAddr;

use crate::ipv4::IPv4Packet;

struct EthernetHeader {
    source_mac: MacAddr,
    destination_mac: MacAddr,
    ether_type: u16,            /* 16 bits. */
}
impl EthernetHeader {}

struct EthernetTail {
    fcs: u8                     /* 4 bits. */
}
impl EthernetTail {}

pub(crate) struct EthernetFrame {
    header: EthernetHeader,
    data: IPv4Packet,
    tail: EthernetTail
}
impl EthernetFrame {
    pub(crate) fn send(&self, sender: &mut Box<dyn DataLinkSender>) {
        let header = &self.header;
        let data = &self.data;
        let tail = &self.tail;

        /* TODO: Which size? */
        let mut vec: Vec<u8> = vec![0; 42];
        let mut new_packet = MutableEthernetPacket::new(&mut vec[..]).unwrap();

        new_packet.set_source(header.source_mac);
        new_packet.set_destination(header.destination_mac);
        new_packet.set_ethertype(pnet::packet::ethernet::EtherType::new(2));

        let ipv4_packet = data.construct();
        new_packet.set_payload(ipv4_packet.packet());
    }
    pub(crate) fn new() -> EthernetFrame {
        todo!()
    }
}
impl Clone for EthernetFrame {
    fn clone(&self) -> Self {
        todo!()
    }
}