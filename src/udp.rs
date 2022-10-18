use gtk::prelude::*;
use pnet::packet::udp::MutableUdpPacket;
use crate::error_window::error;
use crate::icmp::IcmpOptions;

pub(crate) struct UdpOptions {
    src_port: gtk::Entry,
    dest_port: gtk::Entry,
    length: gtk::Entry,
    checksum: gtk::Entry,
    data: gtk::Entry
}
impl UdpOptions {
    pub(crate) fn show_window() {
        let widgets = UdpOptions::new();

        let dialog = gtk::Dialog::with_buttons(
            Some("UDP options"),
            Some(&gtk::Window::new()),
            gtk::DialogFlags::USE_HEADER_BAR,
            &[("Ok", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        dialog.content_area().append(&Self::generate_ui(&widgets));

        dialog.connect_response(move |dialog, response| {
            match response {
                gtk::ResponseType::Ok => {
                    let packet = widgets.build_packet();
                    dialog.close();
                },
                gtk::ResponseType::Cancel => {
                    dialog.close();
                },
                _ => {}
            }
        });

        dialog.show();
    }

    fn generate_ui(&self) -> gtk::Box {
        let fields_grid = self.prepare_ui_fields();

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_start(24).margin_end(24).build();
        lower_box.append(&gtk::Label::new(Some("Data"))); lower_box.append(&self.data);

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_top(24).margin_bottom(24).build();
        main_box.append(&fields_grid); main_box.append(&lower_box);

        main_box
    }
    pub(crate) fn prepare_ui_fields(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).margin_top(24).margin_bottom(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&gtk::Label::builder().label("Source port").halign(gtk::Align::Start).build(),       0, 0, 1, 1);
        grid.attach(&self.src_port,                          1, 0, 1, 1);
        grid.attach(&gtk::Label::builder().label("Destination port").halign(gtk::Align::Start).build(),  2, 0, 1, 1);
        grid.attach(&self.dest_port,                         3, 0, 1, 1);
        grid.attach(&gtk::Label::builder().label("Length").halign(gtk::Align::Start).build(),            0, 1, 1, 1);
        grid.attach(&self.length,                            1, 1, 1, 1);
        grid.attach(&gtk::Label::builder().label("Checksum").halign(gtk::Align::Start).build(),          2, 1, 1, 1);
        grid.attach(&self.checksum,                          3, 1, 1, 1);
        
        grid
    }
    
    pub(crate) fn new() -> UdpOptions {
        UdpOptions {
            src_port: gtk::Entry::builder().placeholder_text("Port..").build(),
            dest_port: gtk::Entry::builder().placeholder_text("Port..").build(),
            length: gtk::Entry::builder().placeholder_text("Length..").build(),
            checksum: gtk::Entry::builder().placeholder_text("Checksum..").build(),
            data: gtk::Entry::builder().placeholder_text("Data..").build()
        }
    }
    fn build_packet(&self) -> Option<MutableUdpPacket> {
        let mut packet = MutableUdpPacket::owned(vec![0u8; MutableUdpPacket::minimum_packet_size()]).unwrap();

        match self.src_port.text().parse::<u16>() {
            Ok(port) => packet.set_source(port),
            Err(_) => { error("Bad udp source port value."); return None; }
        }
        match self.dest_port.text().parse::<u16>() {
            Ok(port) => packet.set_destination(port),
            Err(_) => { error("Bad udp destination port value."); return None; }
        }
        match self.length.text().parse::<u16>() {
            Ok(length) => packet.set_length(length),
            Err(_) => { error("Bad udp length value."); return None; }
        }
        match self.checksum.text().parse::<u16>() {
            Ok(checksum) => packet.set_checksum(checksum),
            Err(_) => { error("Bad udp checksum value."); return None; }
        }
        packet.set_payload(self.data.text().as_bytes());

        return Some(packet);
    }
}