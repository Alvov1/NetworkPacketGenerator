struct TCPHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement: u32,
    offset: u8,
    reserved: u8,
    flags: [bool; 9],
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Option<[u8; 36]>
}
impl TCPHeader {}

struct TCPSegment {
    header: TCPHeader,
    data: String
}
impl TCPSegment {}