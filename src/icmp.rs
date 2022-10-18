use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use gtk::prelude::*;
use pnet::packet::icmp::{IcmpCode, IcmpTypes, MutableIcmpPacket};
use crate::error_window::error;
use crate::main;
use crate::udp::UdpOptions;
use crate::widgets::MainWindow;

pub(crate) struct IcmpOptions {
    type_dropdown: gtk::DropDown,
    code_entry: gtk::Entry,
    checksum_entry: gtk::Entry,
    data_entry: gtk::Entry,
}
impl IcmpOptions {
    pub(crate) fn show_window() {
        let icmp_widgets = IcmpOptions::new();
        let udp_widgets = UdpOptions::new();
        let packet: Arc<Mutex<Option<MutableIcmpPacket>>> = Arc::new(Mutex::new(None));

        let dialog = gtk::Dialog::with_buttons(
            Some("ICMP options"),
            Some(&gtk::Window::new()),
            gtk::DialogFlags::USE_HEADER_BAR,
            &[("Ok", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        dialog.content_area().append(&icmp_widgets.generate_ui(&udp_widgets));

        let clone = packet.clone();
        dialog.connect_response(move |dialog, response| {
            match response {
                gtk::ResponseType::Ok => {
                    match icmp_widgets.build_packet() {
                        Some(value) => *clone.lock().unwrap() = Some(value),
                        None => {}
                    }
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

    fn generate_ui(&self, udp_options: &UdpOptions) -> gtk::Box {
        let udp_grid = udp_options.prepare_ui_fields();
        let icmp_grid = self.prepare_ui_fields();

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        let icmp_frame = gtk::Frame::builder().label("ICMP options").child(&icmp_grid)
            .margin_start(24).margin_end(24).margin_top(24).build();
        let udp_frame = gtk::Frame::builder().label("UDP options").child(&udp_grid)
            .margin_start(24).margin_end(24).margin_bottom(24).build();
        main_box.append(&icmp_frame);
        main_box.append(&udp_frame);

        main_box
    }
    pub(crate) fn prepare_ui_fields(&self) -> gtk::Grid {
        let icmp_grid = gtk::Grid::builder().margin_start(24).margin_end(24).margin_top(24).margin_bottom(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        icmp_grid.attach(&gtk::Label::builder().label("Type:").halign(gtk::Align::Start).build(), 0, 0, 1, 1);
        icmp_grid.attach(&self.type_dropdown, 1, 0, 1, 1);

        icmp_grid.attach(&gtk::Label::builder().label("Code:").halign(gtk::Align::Start).build(), 2, 0, 1, 1);
        icmp_grid.attach(&self.code_entry, 3, 0, 1, 1);

        icmp_grid.attach(&gtk::Label::builder().label("Checksum:").halign(gtk::Align::Start).build(), 0, 1, 1, 1);
        icmp_grid.attach(&self.checksum_entry, 1, 1, 1, 1);

        icmp_grid.attach(&gtk::Label::builder().label("Data:").halign(gtk::Align::Start).build(), 2, 1, 1, 1);
        icmp_grid.attach(&self.data_entry, 3, 1, 1, 1);

        icmp_grid
    }

    pub(crate) fn new() -> IcmpOptions {
        IcmpOptions {
            type_dropdown: gtk::DropDown::from_strings(&["Request", "Response"]),
            code_entry: gtk::Entry::builder().placeholder_text("ICMP code..").build(),
            checksum_entry: gtk::Entry::builder().placeholder_text("ICMP checksum..").build(),
            data_entry: gtk::Entry::builder().placeholder_text("Data..").build(),
        }
    }
    fn build_packet(&self) -> Option<MutableIcmpPacket<'static>> {
        let mut packet = MutableIcmpPacket::owned(vec![0u8; MutableIcmpPacket::minimum_packet_size()]).unwrap();

        match self.type_dropdown.selected() {
        0 => packet.set_icmp_type(IcmpTypes::EchoRequest),
        1 => packet.set_icmp_type(IcmpTypes::EchoReply),
        _ => { error("Unsupported ICMP message type"); return None; }
    }

        match self.code_entry.text().parse::<u8>() {
        Ok(value) => packet.set_icmp_code(IcmpCode::new(value)),
        _ => { error("Bad ICMP code value"); return None; }
    }

        match self.checksum_entry.text().parse::<u16>() {
        Ok(value) => packet.set_checksum(value),
        _ => { error("Bad ICMP checksum value"); return None; }
    }

        packet.set_payload(self.data_entry.text().as_bytes());

        Some(packet)
    }
}
