enum UDPData {
    String,
    ICMPPacket
}
impl UDPData {}

struct UDPHeader {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16
}
impl UDPHeader {}

struct UDPSegment {
    header: UDPHeader,
    data: String
}
impl UDPSegment {}