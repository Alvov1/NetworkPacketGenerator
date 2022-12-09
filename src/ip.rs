use gtk::prelude::*;

use std::net::Ipv4Addr;
use pnet::packet::Packet;
use pnet::packet::FromPacket;
use pnet::packet::ipv4::Ipv4Option;
use pnet::packet::ipv4::Ipv4OptionNumber;
use pnet::packet::ipv4::Ipv4OptionNumbers;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4::MutableIpv4OptionPacket;
use pnet::packet::ipv4::MutableIpv4Packet;

use std::str::FromStr;

use crate::error_window::error;
use crate::show_packet::show;

pub(crate) struct IPWidgets {
    src_ip: gtk::Entry,
    dest_ip: gtk::Entry,

    version: (gtk::CheckButton, gtk::Entry),
    header_length: (gtk::CheckButton, gtk::Entry),
    dscp: (gtk::CheckButton, gtk::Entry),
    ecn: (gtk::CheckButton, gtk::Entry),
    packet_length: (gtk::CheckButton, gtk::Entry),
    packet_id: (gtk::CheckButton, gtk::Entry),
    offset: (gtk::CheckButton, gtk::Entry),
    ttl: (gtk::CheckButton, gtk::Entry),
    checksum: (gtk::CheckButton, gtk::Entry),

    flags: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton),
    options: gtk::Entry
}
impl IPWidgets {
    pub(crate) fn new() -> Self {
        Self {
            src_ip: gtk::Entry::builder().placeholder_text("Source IPv4").build(),
            dest_ip: gtk::Entry::builder().placeholder_text("Destination IPv4").build(),

            version: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Version").build()),
            header_length: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Header length").build()),
            dscp: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("DSCP value").build()),
            ecn: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Explicit congestion").build()),
            packet_length: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Packet length").build()),
            packet_id: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Packet id").build()),
            offset: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Fragment offset").build()),
            ttl: (gtk::CheckButton::builder().label("Auto").active(true).build(), gtk::Entry::builder().placeholder_text("Time to live").build()),
            checksum: (gtk::CheckButton::builder().label("Auto").active(true).build(),
                       gtk::Entry::builder().placeholder_text("Header checksum").margin_end(6).margin_start(6).margin_top(6).margin_bottom(6).build()),

            flags: (gtk::CheckButton::with_label("DF"),
                    gtk::CheckButton::with_label("MF"),
                    gtk::CheckButton::with_label("Reserved bit")),

            options: gtk::Entry::builder().placeholder_text("Option 1, Option 2, ...").margin_end(6).margin_start(6).margin_top(6).margin_bottom(6).build()
        }
    }

    pub(crate) fn prepare_address_section(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&gtk::Label::new(Some("Source IP")), 0, 0, 1, 1);
        grid.attach(&self.src_ip, 1, 0, 1, 1);
        grid.attach(&gtk::Label::new(Some("Destination IP")), 0, 1, 1, 1);
        grid.attach(&self.dest_ip, 1, 1, 1, 1);

        grid
    }
    pub(crate) fn prepare_options_section(&self) -> gtk::Frame {
        /* Result box. */
        let upper_common_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).margin_start(24)
            .margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_bottom(20).build();

        /* Left side */ {
            /* Left grid. Five rows. Each row consists of label, checkbox 'auto', text entry. */
            let left_grid = gtk::Grid::builder().margin_start(12).margin_end(12).row_spacing(24)
                .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

            /* Left grid labels */ {
                /* Version */ {
                    let version = gtk::Label::builder().label("Version:").halign(gtk::Align::Start).build();
                    left_grid.attach(&version, 0, 0, 1, 1);
                }
                /* Header length */ {
                    let header_length = gtk::Label::builder().label("Header length:").halign(gtk::Align::Start).build();
                    left_grid.attach(&header_length, 0, 1, 1, 1);
                }
                /* DSCP */ {
                    let dscp = gtk::Label::builder().label("Differentiated Services CP:").halign(gtk::Align::Start).build();
                    left_grid.attach(&dscp, 0, 2, 1, 1);
                }
                /* ECN */ {
                    let ecn = gtk::Label::builder().label("Explicit Congestion Notification:").halign(gtk::Align::Start).build();
                    left_grid.attach(&ecn, 0, 3, 1, 1);
                }
            }

            /* Left grid auto-entry boxes */ {
                /* Version */ {
                    let version_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    version_box.append(&self.version.0); version_box.append(&self.version.1);
                    left_grid.attach(&(version_box.clone()), 1, 0, 1, 1);
                }
                /* Header length */ {
                    let ihl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    ihl_box.append(&self.header_length.0); ihl_box.append(&self.header_length.1);
                    left_grid.attach(&(ihl_box.clone()), 1, 1, 1, 1);
                }
                /* DSCP */ {
                    let type_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    type_box.append(&self.dscp.0); type_box.append(&self.dscp.1);
                    left_grid.attach(&(type_box.clone()), 1, 2, 1, 1);
                }
                /* ECN */ {
                    let length_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    length_box.append(&self.ecn.0); length_box.append(&self.ecn.1);
                    left_grid.attach(&(length_box.clone()), 1, 3, 1, 1);
                }
            }

            upper_common_box.append(&left_grid);
        }

        /* Right side */ {
            /* Right box. Gathers right grid and bottom box together. */
            let right_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
                .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

            /* Right grid. */ {
                /* Right grid. Four rows. Each row consists of label, checkbox 'auto', text entry. */
                let right_grid = gtk::Grid::builder().halign(gtk::Align::Center)
                    .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

                /* Right grid labels */ {
                    /* Total length */ {
                        let total_length = gtk::Label::builder().label("Packet length:").halign(gtk::Align::Start).build();
                        right_grid.attach(&total_length, 0, 0, 1, 1);
                    }
                    /* Packet ID. */ {
                        let packet_id = gtk::Label::builder().label("Identification:").halign(gtk::Align::Start).build();
                        right_grid.attach(&packet_id, 0, 1, 1, 1);
                    }
                    /* Fragment offset. */ {
                        let offset = gtk::Label::builder().label("Fragment offset:").halign(gtk::Align::Start).build();
                        right_grid.attach(&offset, 0, 2, 1, 1);
                    }
                    /* Time to live. */ {
                        let ttl = gtk::Label::builder().label("Time to live:").halign(gtk::Align::Start).build();
                        right_grid.attach(&ttl, 0, 3, 1, 1);
                    }
                }

                /* Right grid auto-entry boxes */ {
                    /* Total length */ {
                        let checksum_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        checksum_box.append(&self.packet_length.0); checksum_box.append(&self.packet_length.1);
                        right_grid.attach(&(checksum_box.clone()), 1, 0, 1, 1);
                    }
                    /* Packet ID */ {
                        let packet_id_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        packet_id_box.append(&self.packet_id.0); packet_id_box.append(&self.packet_id.1);
                        right_grid.attach(&(packet_id_box.clone()), 1, 1, 1, 1);
                    }
                    /* Offset */ {
                        let offset_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        offset_box.append(&self.offset.0); offset_box.append(&self.offset.1);
                        right_grid.attach(&(offset_box.clone()), 1, 2, 1, 1);
                    }
                    /* Time to live */ {
                        let ttl_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        ttl_box.append(&self.ttl.0); ttl_box.append(&self.ttl.1);
                        right_grid.attach(&(ttl_box.clone()), 1, 3, 1, 1);
                    }
                }

                right_box.append(&right_grid);
            }

            upper_common_box.append(&right_box);
        }


        let bottom_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .margin_bottom(12).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(30).build();

        /* IP options */ {
            let options_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(6).build();

            options_box.append(&gtk::Label::new(Some("Options:"))); options_box.append(&self.options);
            bottom_box.append(&options_box);
        }

        /* Flags */ {
            let frame_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(6).margin_start(6).margin_end(6).build();
            frame_box.append(&self.flags.0); frame_box.append(&self.flags.1); frame_box.append(&self.flags.2);
            bottom_box.append(&gtk::Frame::builder().label("Flags").child(&frame_box).build());
        }

        /* Checksum */ {
            let checksum_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(6).build();
            checksum_box.append(&gtk::Label::new(Some("Checksum:"))); checksum_box.append(&self.checksum.0);
            checksum_box.append(&self.checksum.1); bottom_box.append(&checksum_box);
        }

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).halign(gtk::Align::Center).spacing(6).build();
        main_box.append(&upper_common_box); main_box.append(&bottom_box);

        let box_frame = gtk::Frame::builder().label("IP options").build();
        box_frame.set_child(Some(&main_box));

        box_frame
    }

    fn get_options(&self) -> Option<Vec<Ipv4Option>> {
        if self.options.text().is_empty() { return None; }
        let mut options: Vec<Ipv4Option> = Vec::new();
        for option in self.options.text().split(',').map(|v| v.trim()) {
            match option {
                "ADDEXT" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::ADDEXT.0));
                    options.push(t_option.from_packet()) },
                "CIPSO" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::CIPSO.0));
                    options.push(t_option.from_packet()) },
                "DPS" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::DPS.0));
                    options.push(t_option.from_packet()) },
                "EIP" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::EIP.0));
                    options.push(t_option.from_packet()) },
                "ENCODE" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::ENCODE.0));
                    options.push(t_option.from_packet()) },
                "EOL" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::EOL.0));
                    options.push(t_option.from_packet()) },
                "ESEC" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::ESEC.0));
                    options.push(t_option.from_packet()) },
                "EXP" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::EXP.0));
                    options.push(t_option.from_packet()) },
                "FINN" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::FINN.0));
                    options.push(t_option.from_packet()) },
                "IMITD" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::IMITD.0));
                    options.push(t_option.from_packet()) },
                "LSR" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::LSR.0));
                    options.push(t_option.from_packet()) },
                "MTUP" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::MTUP.0));
                    options.push(t_option.from_packet()) },
                "MTUR" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::MTUR.0));
                    options.push(t_option.from_packet()) },
                "NOP" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::NOP.0));
                    options.push(t_option.from_packet()) },
                "QS" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::QS.0));
                    options.push(t_option.from_packet()) },
                "RR" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::RR.0));
                    options.push(t_option.from_packet()) },
                "RTRALT" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::RTRALT.0));
                    options.push(t_option.from_packet()) },
                "SDB" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::SDB.0));
                    options.push(t_option.from_packet()) },
                "SEC" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::SEC.0));
                    options.push(t_option.from_packet()) },
                "SID" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::SID.0));
                    options.push(t_option.from_packet()) },
                "SSR" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::SSR.0));
                    options.push(t_option.from_packet()) },
                "TR" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::TR.0));
                    options.push(t_option.from_packet()) },
                "TS" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::TS.0));
                    options.push(t_option.from_packet()) },
                "UMP" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::UMP.0));
                    options.push(t_option.from_packet()) },
                "VISA" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::VISA.0));
                    options.push(t_option.from_packet()) },
                "ZSU" => {
                    let mut t_option = MutableIpv4OptionPacket::owned(vec![0u8; MutableIpv4OptionPacket::minimum_packet_size()]).unwrap();
                    t_option.set_number(Ipv4OptionNumber::new(Ipv4OptionNumbers::ZSU.0));
                    options.push(t_option.from_packet()) },
                _ => { error("Bad IP option value."); return None; }
            }
        }
        return Some(options);
    }
    pub(crate) fn get_addresses(&self) -> Option<(Ipv4Addr, Ipv4Addr)> {
        let src = match Ipv4Addr::from_str(&self.src_ip.text()) {
            Ok(address) => address,
            Err(_) => { error("Bad IPv4 address value"); return None; }
        };
        let dest = match Ipv4Addr::from_str(&self.dest_ip.text()) {
            Ok(address) => address,
            Err(_) => { error("Bad IPv4 address value"); return None; }
        };

        Some((src, dest))
    }
    pub(crate) fn build_packet(&self, next_protocol: IpNextHeaderProtocol, data: &[u8]) -> Option<Vec<u8>> {
        let options = match self.get_options() {
            Some(options) => options,
            None => Vec::new()
        };

        let header_length = match self.header_length.0.is_active() {
            true => { 5 },
            false => {
                match self.header_length.1.text().parse::<u8>() {
                    Ok(value) => value,
                    Err(_) => { error("Bad IP header length value"); return None; }
                }
            }
        };
        let auto_total_len = ((header_length as usize * 4) + data.len() + options.len()) as u16;

        let mut packet = MutableIpv4Packet::owned(vec![0u8; auto_total_len as usize]).unwrap();
        packet.set_options(&options);

        packet.set_header_length(header_length);

        match self.get_addresses() {
            Some(addresses) => {
                packet.set_source(addresses.0);
                packet.set_destination(addresses.1);
            },
            None => return None
        }

        if self.version.0.is_active() {
            packet.set_version(4);
        } else {
            match self.version.1.text().parse::<u8>() {
                Ok(value) => packet.set_version(value),
                Err(_) => { error("Bad ip version value"); return None; }
            }
        }


        if self.dscp.0.is_active() {
            packet.set_dscp(0);
        } else {
            match self.dscp.1.text().parse::<u8>() {
                Ok(value) => packet.set_dscp(value),
                Err(_) => { error("Bad IP DSCP value"); return None; }
            }
        }

        if self.ecn.0.is_active() {
            packet.set_ecn(0);
        } else {
            match self.ecn.1.text().parse::<u8>() {
                Ok(value) => packet.set_ecn(value),
                Err(_) => { error("Bad IP ECN value"); return None; }
            }
        }

        if self.packet_length.0.is_active() {
            packet.set_total_length(auto_total_len);
        } else {
            match self.packet_length.1.text().parse::<u16>() {
                Ok(value) => packet.set_total_length(value),
                Err(_) => { error("Bad IP total length value"); return None; }
            }
        }

        if self.packet_id.0.is_active() {
            packet.set_identification(12345);
        } else {
            match self.packet_id.1.text().parse::<u16>() {
                Ok(value) => packet.set_identification(value),
                Err(_) => { error("Bad IP packet ID value"); return None; }
            }
        }

        let mut flags = 0;
        if self.flags.0.is_active() { flags |= pnet::packet::ipv4::Ipv4Flags::DontFragment; }
        if self.flags.1.is_active() { flags |= pnet::packet::ipv4::Ipv4Flags::MoreFragments; }
        if self.flags.2.is_active() { flags |= 0b00000100; }
        packet.set_flags(flags);

        if self.offset.0.is_active() {
            packet.set_fragment_offset(0);
        } else {
            match self.offset.1.text().parse::<u16>() {
                Ok(value) => packet.set_fragment_offset(value),
                Err(_) => { error("Bad IP fragment offset value"); return None; }
            }
        }

        if self.ttl.0.is_active() {
            packet.set_ttl(64);
        } else {
            match self.ttl.1.text().parse::<u8>() {
                Ok(value) => packet.set_ttl(value),
                Err(_) => { error("Bad IP time to live value"); return None; }
            }
        }

        packet.set_next_level_protocol(next_protocol);
        packet.set_payload(data);

        if self.checksum.0.is_active() {
            packet.set_checksum(pnet::packet::ipv4::checksum(&packet.to_immutable()));
        } else {
            match self.checksum.1.text().parse::<u16>() {
                Ok(value) => packet.set_checksum(value),
                Err(_) => { error("Bad IP checksum value"); return None; }
            }
        }

        let payload = Vec::from(packet.packet());
        show("IPv4 packet", &payload);
        Some(payload)
    }
}