use std::cell::RefCell;
use std::net::Ipv4Addr;
use std::rc::Rc;
use gtk::prelude::*;
use pnet::packet::Packet;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::icmp::IcmpCode;
use pnet::packet::icmp::MutableIcmpPacket;
use pnet::packet::ip::IpNextHeaderProtocol;
use crate::database::Database;

use crate::udp::UdpOptions;
use crate::error_window::error;
use crate::show_packet::show;
use crate::widgets::MainWindowWidgets;

pub(crate) struct IcmpOptions {
    type_dropdown: gtk::DropDown,
    code_entry: gtk::Entry,
    checksum_entry: gtk::Entry,
    data_entry: gtk::Entry,
}
impl IcmpOptions {
    pub(crate) fn show_window(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) {
        let icmp_widgets = IcmpOptions::new();
        let dialog = gtk::Dialog::with_buttons(
            Some("ICMP options"),
            Some(&gtk::Window::new()),
            gtk::DialogFlags::USE_HEADER_BAR,
            &[("Ok", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        dialog.content_area().append(&icmp_widgets.generate_ui());

        dialog.connect_response(move |dialog, response| {
            match response {
                gtk::ResponseType::Ok => {
                    let icmp_packet = match icmp_widgets.build_packet() {
                        Some(value) => value,
                        None => { dialog.close(); return; }
                    };

                    show("ICMP packet", &icmp_packet);

                    let ip_packet = match widgets.borrow().ip_widgets.build_packet(IpNextHeaderProtocol::new(1), &icmp_packet) {
                        Some(value) => value,
                        None => { dialog.close(); return; }
                    };

                    MainWindowWidgets::build_frame(widgets.clone(), &ip_packet, database.clone(), "ICMP");
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
        let icmp_grid = self.prepare_ui_fields();

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        main_box.append(&icmp_grid);

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
    fn build_packet(&self) -> Option<Vec<u8>> {
        let packet_size = match self.data_entry.text().is_empty() {
            true => MutableIcmpPacket::minimum_packet_size() + "ICMP request".to_string().bytes().len(),
            false => MutableIcmpPacket::minimum_packet_size() + self.data_entry.text().bytes().len()
        };
        let mut packet = MutableIcmpPacket::owned(vec![0u8; packet_size]).unwrap();

        let payload: Vec<_> = match self.data_entry.text().is_empty() {
            true => "ICMP request".to_string().bytes().collect(),
            false => self.data_entry.text().bytes().collect()
        };

        packet.set_payload(&payload);

        match self.type_dropdown.selected() {
            0 => packet.set_icmp_type(IcmpTypes::EchoRequest),
            1 => packet.set_icmp_type(IcmpTypes::EchoReply),
            _ => { error("Unsupported ICMP message type"); return None; }
        }

        if self.code_entry.text_length() > 0 {
            match self.code_entry.text().parse::<u8>() {
                Ok(value) => packet.set_icmp_code(IcmpCode::new(value)),
                _ => {
                    error("Bad ICMP code value");
                    return None;
                }
            }
        } else { packet.set_icmp_code(IcmpCode::new(8)); }



        if self.checksum_entry.text_length() > 0 {
            match self.checksum_entry.text().parse::<u16>() {
                Ok(value) => packet.set_checksum(value),
                _ => {
                    error("Bad ICMP checksum value");
                    return None;
                }
            }
        } else {
            packet.set_checksum(pnet::packet::icmp::checksum(&packet.to_immutable()));
        }

        Some(Vec::from(packet.packet()))
    }
}
