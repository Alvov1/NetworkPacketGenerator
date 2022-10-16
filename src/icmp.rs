use gtk::{Button, DropDown, Entry, Grid, Label, Window};
use gtk::prelude::{BoxExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
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
pub struct ICMPWindow<'a> {
    widgets: ICMPWidgets,
    window: Window,
    packet: Option<MutableIcmpPacket<'a>>,
    ok_button: Button,
    cancel_button: Button
}
impl ICMPWindow<'_> {
    fn new() -> ICMPWindow<'static> {
        let widgets = ICMPWidgets::new();
        let window = Window::builder().title("ICMP settings").default_width(700).default_height(500).build();

        let upper_grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        let icmp_type_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        icmp_type_box.append(&Label::new(Some("Type"))); icmp_type_box.append(&widgets.type_dropdown);
        upper_grid.attach(&icmp_type_box, 0, 0, 1, 1);

        let icmp_code = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        icmp_code.append(&Label::new(Some("Code"))); icmp_code.append(&widgets.code_entry);
        upper_grid.attach(&icmp_code, 1, 0, 1, 1);

        let icmp_checksum = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        icmp_checksum.append(&Label::new(Some("Checksum"))); icmp_checksum.append(&widgets.checksum_entry);
        upper_grid.attach(&icmp_checksum, 0, 1, 1, 1);

        let icmp_rest = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        icmp_rest.append(&Label::new(Some("Rest of the header"))); icmp_rest.append(&widgets.rest_entry);
        upper_grid.attach(&icmp_rest, 1, 1, 1, 1);

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::End).valign(gtk::Align::Center).spacing(24).margin_start(24).margin_end(24).build();
        let ok_button = Button::with_label("OK"); let cancel_button = Button::with_label("Cancel");
        lower_box.append(&ok_button); lower_box.append(&cancel_button);

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        main_box.append(&upper_grid); main_box.append(&lower_box);

        window.set_child(Some(&main_box));
        ICMPWindow { widgets, window, packet: None, ok_button, cancel_button }
    }
    pub fn show() {
        let window = ICMPWindow::new();
        window.window.show();
    }
}