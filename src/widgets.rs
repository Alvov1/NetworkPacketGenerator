use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{Arc, LockResult};
use std::sync::Mutex;

use gtk::prelude::*;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use pnet::util::MacAddr;
use pnet::packet::MutablePacket;
use pnet::packet::ethernet::{EtherType, MutableEthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocol;

use crate::icmp;
use crate::udp;
use crate::ip::IPWidgets;
use crate::tcp::TCPWidgets;
use crate::udp::UdpOptions;
use crate::icmp::IcmpOptions;
use crate::error_window::error;

struct NetworkInterfaceWidget {
    list: gtk::DropDown,
    interfaces: Vec<String>
}
impl NetworkInterfaceWidget {
    fn new(names: &[&str]) -> Self {
        let list = gtk::DropDown::from_strings(names);
        let mut interfaces = Vec::with_capacity(names.len());
        for name in names { interfaces.push(name.to_string()); }
        Self { list, interfaces }
    }
    fn set_active(&self, value: u32) { self.list.set_selected(value); }
    fn get_active(&self) -> String { self.interfaces[self.list.selected() as usize].clone() }
}

struct MacAddressesWidgets {
    source: gtk::Entry,
    destination: gtk::Entry
}
impl MacAddressesWidgets {
    fn new() -> MacAddressesWidgets {
        Self {
            source: gtk::Entry::builder().placeholder_text("Source MAC").text("FF.FF.FF.FF.FF.FF").build(),
            destination: gtk::Entry::builder().placeholder_text("Destination MAC").text("FF.FF.FF.FF.FF.FF").build()
        }
    }
    fn get(&self) -> Option<(MacAddr, MacAddr)> {
        let source = match MacAddr::from_str(self.source.text().as_str()) {
            Ok(address) => address,
            Err(_) => { error("Bad source mac address value."); return None }
        };
        let destination = match MacAddr::from_str(self.destination.text().as_str()) {
            Ok(address) => address,
            Err(_) => { error("Bad destination mac address value."); return None }
        };

        Some((source, destination))
    }
}

pub struct MainWindowWidgets {
    interface_list: NetworkInterfaceWidget,

    buttons: (gtk::CheckButton, gtk::CheckButton, gtk::CheckButton, gtk::CheckButton),
    macs: MacAddressesWidgets,

    ip_widgets: IPWidgets,
    tcp_widgets: TCPWidgets
}
impl MainWindowWidgets {
    fn generate_ui(&self, button: &gtk::Button) -> gtk::Box {
        let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24).margin_bottom(24)
            .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        /* First section. */ {
            let section_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

            /* Initialize first section. */
            section_box.append(&gtk::Label::new(Some("Interface:")));
            section_box.append(&self.interface_list.list);
            section_box.append(&self.get_protocol_table());
            section_box.append(&self.ip_widgets.prepare_address_section());

            /* Add main button. */
            section_box.append(button);

            container.append(&section_box);
        }

        /* Second section. */
        container.append(&self.get_mac_address_table());

        /* Third section. */
        container.append(&self.get_utility_buttons());

        /* Forth section. */
        container.append(&self.ip_widgets.prepare_options_section());

        /* Fifth section. */
        container.append(&self.tcp_widgets.prepare_ui_fields());

        container
    }
    fn get_protocol_table(&self) -> gtk::Grid {
        let protocol_table = gtk::Grid::builder().margin_start(6).margin_end(6).row_spacing(6)
            .halign(gtk::Align::Center).valign(gtk::Align::Center).column_spacing(6).name("protocol-table").build();

        /* Ip - 0, tcp - 1, udp - 2, icmp - 3 */
        self.buttons.1.set_group(Some(&self.buttons.2));
        self.buttons.3.set_group(Some(&self.buttons.1));
        self.buttons.0.set_group(Some(&self.buttons.3));

        protocol_table.attach(&self.buttons.0, 0, 0, 1, 1);
        protocol_table.attach(&self.buttons.3, 1, 0, 1, 1);
        protocol_table.attach(&self.buttons.1, 0, 1, 1, 1);
        protocol_table.attach(&self.buttons.2, 1, 1, 1, 1);

        protocol_table
    }
    fn get_mac_address_table(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).halign(gtk::Align::Center)
            .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

        let source_lable = gtk::Label::builder().label("Source MAC").halign(gtk::Align::Start).build();
        grid.attach(&source_lable, 0, 0, 1, 1);
        grid.attach(&self.macs.source, 1, 0, 1, 1);

        let destination_lable = gtk::Label::builder().label("Destination MAC").halign(gtk::Align::Start).build();
        grid.attach(&destination_lable, 2, 0, 1, 1);
        grid.attach(&self.macs.destination, 3, 0, 1, 1);

        grid
    }
    fn get_utility_buttons(&self) -> gtk::Box {
        let main_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
            .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

        let save = gtk::Button::with_label("Save Packet");
        main_box.append(&save); save.connect_clicked(move |_| { println!("Packet saved.") });
        let sequence = gtk::Button::with_label("Send Sequence");
        main_box.append(&sequence); sequence.connect_clicked(move |_| { println!("Sequence sent.") });
        let open_file = gtk::Button::with_label("Open File...");
        main_box.append(&open_file); open_file.connect_clicked(move |_| { println!("File opened.") });
        let delete_packet = gtk::Button::with_label("Delete Packet");
        main_box.append(&delete_packet); delete_packet.connect_clicked(move |_| { println!("Packet deleted.") });
        let delete_file = gtk::Button::with_label("Delete File");
        main_box.append(&delete_file); delete_file.connect_clicked(move |_| { println!("File deleted.") });
        let create_file = gtk::Button::with_label("Create File");
        main_box.append(&create_file); create_file.connect_clicked(move |_| { println!("File created.") });

        main_box
    }

    fn new() -> Self {
        let binding = datalink::interfaces();
        let names: Vec<_> = binding.iter().map(|v| &*v.name).collect();
        Self {
            interface_list: NetworkInterfaceWidget::new(&names),

            buttons: ( gtk::CheckButton::builder().label("IP").active(true).build(), gtk::CheckButton::with_label("TCP"),
                       gtk::CheckButton::with_label("UDP"), gtk::CheckButton::with_label("ICMP") ),

            macs: MacAddressesWidgets::new(),
            ip_widgets: IPWidgets::new(),
            tcp_widgets: TCPWidgets::new()
        }
    }
    fn build_packet(widgets: Arc<Mutex<MainWindowWidgets>>) {
        if widgets.lock().unwrap().buttons.3.is_active() {
            Self::build_icmp_packet(widgets.clone());
        }
        if widgets.lock().unwrap().buttons.2.is_active() {
            Self::build_udp_packet(widgets.clone());
        }
        if widgets.lock().unwrap().buttons.1.is_active() {
            Self::build_tcp_packet(widgets.clone());
        }
        if widgets.lock().unwrap().buttons.0.is_active() {
            Self::build_ip_packet(widgets, Vec::new());
        }
    }
    fn build_icmp_packet(widgets: Arc<Mutex<MainWindowWidgets>>) {

    }
    fn build_udp_packet(widgets: Arc<Mutex<MainWindowWidgets>>) {


    }
    fn build_tcp_packet(widgets: Arc<Mutex<MainWindowWidgets>>) {
        let addresses = match widgets.lock().unwrap().ip_widgets.get_addresses() {
            Some(addresses) => addresses,
            None => { error("Bad src or destination IP address value."); return }
        };

        let packet = match widgets.lock().unwrap().tcp_widgets.build_packet(addresses) {
            Some(packet) => packet,
            None => { return }
        };

        Self::build_ip_packet(widgets, packet);
    }
    fn build_ip_packet(widgets: Arc<Mutex<MainWindowWidgets>>, data: Vec<u8>) {
        let packet = match widgets.lock().unwrap().ip_widgets.build_packet(IpNextHeaderProtocol::new(0), &data) {
            Some(packet) => packet,
            None => {}
        };
        packet
        Self::build_frame(widgets.clone(), &packet);
    }
    fn build_frame(widgets: Arc<Mutex<MainWindowWidgets>>, data: &Vec<u8>) {
        let mut frame = MutableEthernetPacket::owned(vec![0u8; MutableEthernetPacket::minimum_packet_size()]).unwrap();

        match widgets.lock() {
            Ok(widgets) => {
                match widgets.macs.get() {
                    Some(addresses) => { frame.set_source(addresses.0); frame.set_destination(addresses.1); },
                    None => return
                }
            }
            Err(_) => { return }
        };

        frame.set_ethertype(EtherType::new(0x0800));
        frame.set_payload(data);

        let interface = widgets.lock().unwrap().interface_list.get_active();

        Self::send_frame(frame.payload_mut(), &interface);
    }

    fn send_frame(payload: &[u8], iface: &str) {
        let interfaces = datalink::interfaces();
        let interface = interfaces.into_iter()
            .filter(|interface: &NetworkInterface| {
                interface.name == iface
            })
            .next()
            .unwrap();

        let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type."),
            Err(e) => panic!("Failed to create datalink channel."),
        };

        match tx.send_to(payload, Some(interface)) {
            Some(_) => {},
            None => error("Failed to send packet.")
        }
    }
}

pub struct MainWindow {
    widgets: Arc<Mutex<MainWindowWidgets>>,
    window: gtk::ApplicationWindow
}
impl MainWindow {
    pub(crate) fn new(app: &gtk::Application) -> Self {
        let widgets = Arc::new(Mutex::new(MainWindowWidgets::new()));

        let button = gtk::Button::with_label("Collect");
        let ui = widgets.lock().unwrap().generate_ui(&button);

        let clone = widgets.clone();
        button.connect_clicked(move |_| {
            println!("{:?}", MainWindowWidgets::build_packet(clone.clone()));
        });

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Network Packet Generator")
            .default_width(900)
            .default_height(500)
            .child(&ui)
            .build();

        Self { widgets, window }
    }
    pub(crate) fn show(&self) { self.window.show(); }
}