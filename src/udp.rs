use gtk::{Align, Button, Entry, Grid, Label, Window};
use gtk::prelude::{BoxExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
use pnet::packet::udp::MutableUdpPacket;
use crate::error_window::error;

struct UDPWidgets {
    src_port: Entry,
    dest_port: Entry,
    length: Entry,
    checksum: Entry,
    data: Entry
}
impl UDPWidgets {
    fn new() -> UDPWidgets {
        UDPWidgets {
            src_port: Entry::builder().placeholder_text("Port").build(),
            dest_port: Entry::builder().placeholder_text("Port").build(),
            length: Entry::builder().placeholder_text("Length").build(),
            checksum: Entry::builder().placeholder_text("Checksum").build(),
            data: Entry::builder().placeholder_text("Data").build()
        }
    }

    fn collect(&self) -> Option<MutableUdpPacket> {
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

pub struct UDPWindow {
    widgets: UDPWidgets,
    window: Window
}
impl UDPWindow {
    pub(crate) fn full() -> UDPWindow {
        let widgets = UDPWidgets::new();

        let window = Window::builder().title("UDP settings")
            .default_width(400).default_height(200).build();

        let grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&Label::builder().label("Source port").halign(Align::Start).build(),       0, 0, 1, 1);
        grid.attach(&widgets.src_port,                          1, 0, 1, 1);
        grid.attach(&Label::builder().label("Destination port").halign(Align::Start).build(),  2, 0, 1, 1);
        grid.attach(&widgets.dest_port,                         3, 0, 1, 1);
        grid.attach(&Label::builder().label("Length").halign(Align::Start).build(),            0, 1, 1, 1);
        grid.attach(&widgets.length,                            1, 1, 1, 1);
        grid.attach(&Label::builder().label("Checksum").halign(Align::Start).build(),          2, 1, 1, 1);
        grid.attach(&widgets.checksum,                          3, 1, 1, 1);

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).margin_start(24).margin_end(24).build();
        let send = Button::with_label("Send"); let cancel = Button::with_label("Cancel");
        lower_box.append(&Label::new(Some("Data"))); lower_box.append(&widgets.data);
        lower_box.append(&send); lower_box.append(&cancel);

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        main_box.append(&grid); main_box.append(&lower_box);

        window.set_child(Some(&main_box));

        UDPWindow { widgets, window }
    }
    pub(crate) fn icmp() -> UDPWindow {
        let widgets = UDPWidgets::new();

        let window = Window::builder().title("UDP settings")
            .default_width(400).default_height(200).build();

        let grid = Grid::builder().margin_start(24).margin_end(24).row_spacing(24)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(24).build();

        grid.attach(&Label::builder().label("Source port").halign(Align::Start).build(),       0, 0, 1, 1);
        grid.attach(&widgets.src_port,                          1, 0, 1, 1);
        grid.attach(&Label::builder().label("Destination port").halign(Align::Start).build(),  2, 0, 1, 1);
        grid.attach(&widgets.dest_port,                         3, 0, 1, 1);
        grid.attach(&Label::builder().label("Length").halign(Align::Start).build(),            0, 1, 1, 1);
        grid.attach(&widgets.length,                            1, 1, 1, 1);
        grid.attach(&Label::builder().label("Checksum").halign(Align::Start).build(),          2, 1, 1, 1);
        grid.attach(&widgets.checksum,                          3, 1, 1, 1);

        let lower_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::End).valign(gtk::Align::Center).spacing(24).margin_start(24).margin_end(24).build();
        let next = Button::with_label("Next"); let cancel = Button::with_label("Cancel");
        lower_box.append(&next); lower_box.append(&cancel);

        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();
        main_box.append(&grid); main_box.append(&lower_box);

        window.set_child(Some(&main_box));

        UDPWindow { widgets, window }
    }
    pub(crate) fn show(&self) { self.window.show(); }
}