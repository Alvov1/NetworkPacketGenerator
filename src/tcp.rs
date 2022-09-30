use pnet::packet::tcp::{MutableTcpPacket, TcpOption};

struct TCPHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement: u32,
    offset: u8,
    reserved: u8,
    flags: u16,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Option<Vec<TcpOption>>
}
impl TCPHeader {
    fn new(values: &[u32; 9], options: Option<Vec<TcpOption>>) -> TCPHeader {
        TCPHeader {
            source_port: values[0] as u16,
            destination_port: values[1] as u16,
            sequence_number: values[2],
            acknowledgement: values[3],
            offset: values[4] as u8,
            reserved: values[5] as u8,
            flags: values[6] as u16,
            window_size: values[7] as u16,
            checksum: values[8] as u16,
            urgent_pointer: values[9] as u16,
            options
        }
    }
}

struct TCPSegment {
    header: TCPHeader,
    data: String
}
impl TCPSegment {
    fn construct(&self) -> MutableTcpPacket {
        let header = &self.header;
        let data = &self.data;

        /* TODO: Which size? */
        let mut vec: Vec<u8> = vec![0; 42];
        let mut packet = MutableTcpPacket::owned(vec).unwrap();

        packet.set_source(header.source_port);
        packet.set_destination(header.destination_port);
        packet.set_sequence(header.sequence_number);
        packet.set_acknowledgement(header.acknowledgement);
        packet.set_data_offset(header.offset);
        packet.set_reserved(header.reserved);
        packet.set_flags(header.flags);
        packet.set_window(header.window_size);
        packet.set_checksum(header.checksum);
        packet.set_urgent_ptr(header.urgent_pointer);

        if header.options.is_some() {
            packet.set_options(&header.options.as_ref().unwrap());
        }

        packet.set_payload(&data.clone().into_bytes());

        packet
    }
}