use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::icmp::MutableIcmpPacket;
use pnet::packet::Packet;

struct UDPData {
    data: Vec<u8>
}
impl UDPData {
    fn from_string(data: &str) -> UDPData {
        UDPData { data: data.as_bytes().to_vec() }
    }
    fn from_icmp(data: &MutableIcmpPacket) -> UDPData {
        UDPData { data: data.packet().to_vec() }
    }
}

struct UDPHeader {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16
}
impl UDPHeader {
    fn new(values: &[u16; 4]) -> UDPHeader {
        UDPHeader { source_port: values[0], destination_port: values[1], length: values[2], checksum: values[3] }
    }
}

struct UDPSegment {
    header: UDPHeader,
    data: UDPData
}
impl UDPSegment {
    fn new(values: &[u16; 4], data: UDPData) -> UDPSegment {
        UDPSegment { header: UDPHeader::new(values), data }
    }
    fn construct(&self) -> MutableUdpPacket {
        let header = &self.header;
        let data = &self.data;

        /* TODO: Which size? */
        let mut vec: Vec<u8> = vec![0; 42];
        let mut packet = MutableUdpPacket::owned(vec).unwrap();

        packet.set_source(header.source_port);
        packet.set_destination(header.destination_port);
        packet.set_length(header.length);
        packet.set_checksum(header.checksum);
        packet.set_payload(&data.data);

        packet
    }
}