struct ARPPacket {
    hardware: u16,
    protocol: u16,
    hardware_len: u8,
    protocol_len: u8,
    operation: u16,
    sender_hardware: u32,
    sender_protocol: u32,
    target_hardware: u32,
    target_protocol: u32
}
impl ARPPacket {}