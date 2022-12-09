use std::cell::RefCell;
use std::ops::RangeInclusive;
use std::rc::Rc;
use gtk::prelude::{BoxExt, ButtonExt, DialogExt, EditableExt, GtkWindowExt, WidgetExt};
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use crate::error_window::error;
use crate::widgets::MainWindowWidgets;

struct StoredPacket {
    widget: gtk::Box,
    packet: Vec<u8>
}
impl StoredPacket {
    pub(crate) fn new(payload: Vec<u8>, label: &str, index: usize) -> StoredPacket {
        let full_label = gtk::Label::new(Some(&(index.to_string() + " " + label)));
        let image = gtk::Image::from_file("packet_icon.png");

        let widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .margin_start(5)
            .margin_end(5)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        widget.append(&image);
        widget.append(&full_label);

        Self {  widget,  packet: payload  }
    }
}

pub(crate) struct Database {
    pub(crate) list: gtk::Box,
    packets: Vec<StoredPacket>
}
impl Database {
    pub(crate) fn new() -> Database {
        Self {
            list: gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(20).build(),
            packets: Vec::new()
        }
    }

    pub(crate) fn push(&mut self, payload: Vec<u8>, label: &str) {
        let item = StoredPacket::new(payload, label, self.packets.len());
        self.list.append(&item.widget);
        self.packets.push(item);
    }

    pub(crate) fn send_range(&self, range: RangeInclusive<usize>, iface: &str) {
        let interfaces = datalink::interfaces();
        let interface = interfaces.into_iter()
            .filter(|interface: &NetworkInterface| {
                interface.name == iface
            })
            .next()
            .unwrap();

        let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type."),
            Err(_) => panic!("Failed to create datalink channel."),
        };

        for i in range {
            if i > self.packets.len() {
                error("Wrong sequence range.");
                return
            }

            let packet = &self.packets[i];
            match tx.send_to(&packet.packet, None) {
                Some(_) => {}
                None => {
                    error(&("Failed to send packet ".to_owned() + &i.to_string()));
                    return;
                }
            }
        }
    }

    pub(crate) fn send_multiple_times(&self, index: usize, amount: usize, iface: &str) {
        let interfaces = datalink::interfaces();
        let interface = interfaces.into_iter()
            .filter(|interface: &NetworkInterface| {
                interface.name == iface
            })
            .next()
            .unwrap();

        let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type."),
            Err(_) => panic!("Failed to create datalink channel."),
        };

        if index > self.packets.len() {
            error("Bad packet number in the queue.");
            return;
        }

        let packet = &self.packets[index];
        for i in 0..amount {
            match tx.send_to(&packet.packet, None) {
                Some(_) => {}
                None => {
                    error(&("Failed to send packet ".to_owned() + &i.to_string()));
                    return;
                }
            }
        }
    }

    pub(crate) fn get_ui_section(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) -> gtk::Box {
        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();


        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Always) // Disable horizontal scrolling
            .vscrollbar_policy(gtk::PolicyType::Never)
            .child(&database.borrow().list)
            .min_content_width(360)
            .margin_start(5)
            .margin_end(5)
            .margin_top(5)
            .margin_bottom(5)
            .build();

        let frame = gtk::Frame::builder().child(&scrolled_window).build();
        main_box.append(&frame);

        let sequence = gtk::Button::with_label("Send Sequence");
        main_box.append(&sequence); sequence.connect_clicked(move |_| {
            let entry = gtk::Entry::builder()
                .placeholder_text("Packets sequence...")
                .margin_start(5)
                .margin_end(5)
                .margin_top(5)
                .margin_bottom(5)
                .build();

            let dialog = gtk::Dialog::with_buttons(
                Some("Send sequence of packets"),
                Some(&gtk::Window::new()),
                gtk::DialogFlags::USE_HEADER_BAR,
                &[("Ok", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
            dialog.content_area().append(&entry);

            let database_clone = database.clone();
            let widgets_clone = widgets.clone();
            dialog.connect_response(move |dialog, response| {
                match response {
                    gtk::ResponseType::Ok => {
                        if entry.text().contains("-") {
                            let indexes: Vec<_> = entry.text().to_string().split("-").map(|v| v.parse::<isize>().unwrap_or(-1)).collect();
                            if indexes.len() == 1 {
                                match indexes[0] {
                                    -1 => { error("Bad packets sequence"); }
                                    _ => {
                                        let iface = widgets_clone.borrow().get_active_interface();
                                        database_clone.borrow().send_range(RangeInclusive::new(indexes[0] as usize, indexes[0] as usize), &iface);
                                    }
                                }
                                dialog.close();
                                return;
                            }

                            if indexes.len() != 2 || indexes[0] == -1 || indexes[1] == -1 || indexes[0] >= indexes[1] {
                                error("Bad packet sequence. Please enter two numbers separated by '*' or '-'");
                                dialog.close();
                            }

                            let iface = widgets_clone.borrow().get_active_interface();
                            database_clone.borrow().send_range(RangeInclusive::new(indexes[0] as usize, indexes[1] as usize), &iface);
                        }

                        if entry.text().contains("*") {
                            let indexes: Vec<_> = entry.text().to_string().split("*").map(|v| v.parse::<usize>().unwrap_or(0)).collect();
                            if indexes.len() != 2 {
                                error("Bad packet sequence. Please enter two numbers separated by '*' or '-'");
                                dialog.close();
                            }
                            let iface = widgets_clone.borrow().get_active_interface();
                            database_clone.borrow().send_multiple_times(indexes[0] as usize, indexes[1] as usize, &iface);
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
        });

        main_box
    }
}