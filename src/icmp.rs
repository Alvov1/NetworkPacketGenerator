use std::thread;
use std::time::Duration;
use gtk::{Button, DropDown, Entry, Grid, Label, Window};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
use pnet::packet::icmp::{IcmpCode, IcmpTypes, MutableIcmpPacket};
use crate::error_window::error;
use crate::main;

struct ICMPWidgets {
    pub type_dropdown: DropDown,
    pub code_entry: Entry,
    pub checksum_entry: Entry,
    pub rest_entry: Entry,
}
impl ICMPWidgets {
    fn new() -> ICMPWidgets {
        ICMPWidgets {
            type_dropdown: gtk::DropDown::from_strings(&["Request", "Response"]),
            code_entry: Entry::builder().placeholder_text("ICMP code..").build(),
            checksum_entry: Entry::builder().placeholder_text("ICMP checksum..").build(),
            rest_entry: Entry::builder().placeholder_text("Rest of the header").build(),
        }
    }
    fn collect(&self) -> Option<MutableIcmpPacket<'static>> {
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

        packet.set_payload(self.rest_entry.text().as_bytes());

        Some(packet)
    }
}
pub struct ICMPWindow {
    widgets: ICMPWidgets,
    pub window: Window
}
impl ICMPWindow {
    pub fn new() -> ICMPWindow {
        let widgets = ICMPWidgets::new();
        let window = Window::builder().title("ICMP settings").default_width(400).default_height(200).build();

        let upper_grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        upper_grid.attach(&Label::new(Some("Type")),    0, 0, 1, 1);
        upper_grid.attach(&widgets.type_dropdown,           1, 0, 1, 1);

        upper_grid.attach(&Label::new(Some("Code")),     2, 0, 1, 1);
        upper_grid.attach(&widgets.code_entry,               3, 0, 1, 1);

        upper_grid.attach(&Label::new(Some("Checksum")), 0, 1, 1, 1);
        upper_grid.attach(&widgets.checksum_entry,           1, 1, 1, 1);

        upper_grid.attach(&Label::new(Some("Rest")),     2, 1, 1, 1);
        upper_grid.attach(&widgets.rest_entry,               3, 1, 1, 1);

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::End).valign(gtk::Align::Center).spacing(24).margin_start(24).margin_end(24).build();
        let send = Button::with_label("Send"); let cancel = Button::with_label("Cancel");
        lower_box.append(&send); lower_box.append(&cancel);

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        main_box.append(&upper_grid); main_box.append(&lower_box);

        window.set_child(Some(&main_box));
        ICMPWindow { widgets, window }
    }
    pub fn collect(&self) -> Option<MutableIcmpPacket> {
        return self.widgets.collect();
    }
}