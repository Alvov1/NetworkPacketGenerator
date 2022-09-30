use pnet::packet::icmp::{IcmpCode, IcmpType, MutableIcmpPacket};

pub(crate) struct ICMPPacket {
    type_value: u8,
    code: u8,
    checksum: u16,
    data: String
}
impl ICMPPacket {
    fn new(values: &[u16; 3], data: &str) -> ICMPPacket {
        ICMPPacket { type_value: values[0] as u8, code: values[1] as u8, checksum: values[2], data: data.to_string() }
    }
    fn construct(&self) -> MutableIcmpPacket {
        /* TODO: Which size? */
        let mut vec: Vec<u8> = vec![0; 42];
        let mut packet = MutableIcmpPacket::owned(vec).unwrap();

        packet.set_icmp_type(IcmpType::new(self.type_value));
        packet.set_icmp_code(IcmpCode::new(self.code));
        packet.set_checksum(self.checksum);
        packet.set_payload(self.data.as_bytes());

        packet
    }
}