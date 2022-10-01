use gtk::pango::Gravity::Auto;

struct Addresses {
    source: Option<String>,
    destination: Option<String>
}
impl Addresses {
    fn new() -> Addresses { Addresses { source: None, destination: None } }
}

struct AutoEntry {
    value: Option<String>
}
impl AutoEntry {
    fn new() -> AutoEntry { AutoEntry { value: None } }
    fn using_auto(&self) -> bool { self.value == None }
}

struct IPOptions {
    version: AutoEntry,
    ihl: AutoEntry,
    service_type: AutoEntry,
    header_length: AutoEntry,
    header_checksum: AutoEntry,
    packet_id: AutoEntry,
    protocol: AutoEntry,
    offset: AutoEntry,
    ttl: AutoEntry,
    flags: Option<u8>
}
impl IPOptions {
    fn new() -> IPOptions {
        IPOptions {
            version: AutoEntry::new(),
            ihl: AutoEntry::new(),
            service_type: AutoEntry::new(),
            header_length: AutoEntry::new(),
            header_checksum: AutoEntry::new(),
            packet_id: AutoEntry::new(),
            protocol: AutoEntry::new(),
            offset: AutoEntry::new(),
            ttl: AutoEntry::new(),
            flags: Some(0),
        }
    }
}

struct TCPOptions {
    source_port: AutoEntry,
    destination_port: AutoEntry,
    sequence_number: AutoEntry,
    acknowledgement: AutoEntry,
    offset: AutoEntry,
    window: AutoEntry,
    checksum: AutoEntry,
    urgent: AutoEntry,
    flags: Option<u8>,
    data: Option<u8>,
    bits: Option<u8>
}
impl TCPOptions {
    fn new() -> TCPOptions {
        TCPOptions {
            source_port: AutoEntry::new(),
            destination_port: AutoEntry::new(),
            sequence_number: AutoEntry::new(),
            acknowledgement: AutoEntry::new(),
            offset: AutoEntry::new(),
            window: AutoEntry::new(),
            checksum: AutoEntry::new(),
            urgent: AutoEntry::new(),
            /* TODO: Change these values. */
            flags: Some(0),
            data: Some(0),
            bits: Some(0),
        }
    }
}

pub(crate) enum Protocol { IP, TCP, UDP, ICMP }

pub(crate) struct Database {
    interface: Option<String>,
    protocol: Option<Protocol>,
    ips: Addresses,
    macs: Addresses,
    ip_options: IPOptions,
    tcp_options: TCPOptions
}
impl Database {
    pub(crate) fn new() -> Database {
        Database {
            interface: None,
            protocol: None,
            ips: Addresses::new(),
            macs: Addresses::new(),
            ip_options: IPOptions::new(),
            tcp_options: TCPOptions::new(),
        }
    }
    pub(crate) fn set_iface(&mut self, iface_name: &str) {
        self.interface = Some(iface_name.to_string());
    }
    pub(crate) fn get_iface(&self) -> String {
        self.interface.as_ref().unwrap().clone()
    }
    pub(crate) fn set_protocol(&mut self, protocol: Protocol) {
        self.protocol = Some(protocol);
    }
}