
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use gtk::prelude::*;
use pnet::datalink;
use pnet::util::MacAddr;
use pnet::packet::Packet;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocol;

use mac_address::get_mac_address;
use mac_address::MacAddress;
use mac_address::MacAddressError;

use crate::ip::IPWidgets;
use crate::tcp::TCPWidgets;
use crate::udp::UdpOptions;
use crate::icmp::IcmpOptions;
use crate::error_window::error;
use crate::show_packet::show;
use crate::database::Database;

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
    pub(crate) fn get_active(&self) -> String { self.interfaces[self.list.selected() as usize].clone() }
}

struct MacAddressesWidgets {
    source: gtk::Entry,
    destination: gtk::Entry
}
impl MacAddressesWidgets {
    fn new(mac_address: Result<Option<MacAddress>, MacAddressError>) -> MacAddressesWidgets {
        let source = match mac_address {
            Ok(Some(address)) => {
                let mut address_string = String::new();
                address_string += &format!("{:2x}", address.bytes()[0]);
                for byte in &address.bytes()[1..] {
                    address_string += &*(".".to_string() + &format!("{:2x}", byte));
                }
                gtk::Entry::builder().placeholder_text("Source MAC").text(&address_string).build()
            },
            _ => gtk::Entry::builder().placeholder_text("Source MAC").text("aa.bb.cc.dd.ee.ff").build()
        };
        Self {
            source,
            destination: gtk::Entry::builder().placeholder_text("Destination MAC").text("aa.bb.cc.dd.ee.ff").build(),
        }
    }
    fn get(&self) -> Option<(MacAddr, MacAddr)> {
        let source = match MacAddr::from_str(self.source.text().replace('.', ":").as_str()) {
            Ok(address) => address,
            Err(what) => { error(&("Bad source mac address value: ".to_owned() + &what.to_string())); return None }
        };
        let destination = match MacAddr::from_str(self.destination.text().replace('.', ":").as_str()) {
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

    pub(crate) ip_widgets: IPWidgets,
    tcp_widgets: TCPWidgets
}
impl MainWindowWidgets {
    fn generate_ui(&self, button: &gtk::Button, database: &gtk::Box) -> gtk::Box {
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
        container.append(database);

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

    pub(crate) fn get_active_interface(&self) -> String {
        self.interface_list.get_active()
    }

    fn new() -> Self {
        let binding = datalink::interfaces();
        let names: Vec<_> = binding.iter().map(|v| &*v.name).collect();
        let source_mac = get_mac_address();

        Self {
            interface_list: NetworkInterfaceWidget::new(&names),

            buttons: ( gtk::CheckButton::builder().label("IP").active(true).build(), gtk::CheckButton::with_label("TCP"),
                       gtk::CheckButton::with_label("UDP"), gtk::CheckButton::with_label("ICMP") ),

            macs: MacAddressesWidgets::new(source_mac),
            ip_widgets: IPWidgets::new(),
            tcp_widgets: TCPWidgets::new()
        }
    }
    fn build_packet(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) {
        if widgets.borrow().buttons.3.is_active() {
            Self::build_icmp_packet(widgets.clone(), database.clone());
        }
        if widgets.borrow().buttons.2.is_active() {
            Self::build_udp_packet(widgets.clone(), database.clone());
        }
        if widgets.borrow().buttons.1.is_active() {
            Self::build_tcp_packet(widgets.clone(), database.clone());
        }
        if widgets.borrow().buttons.0.is_active() {
            match widgets.clone().borrow().tcp_widgets.give_payload() {
                Some(value) => Self::build_ip_packet(widgets, value, IpNextHeaderProtocol::new(0), database, "IP"),
                None => Self::build_ip_packet(widgets, Vec::new(), IpNextHeaderProtocol::new(0), database, "IP")
            }
        }
    }
    fn build_icmp_packet(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) {
        IcmpOptions::show_window(widgets, database);
    }
    fn build_udp_packet(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) {
        let addresses = match widgets.clone().borrow().ip_widgets.get_addresses() {
            Some(addresses) => addresses,
            None => { error("Bad src or destination IP address value."); return }
        };

        UdpOptions::show_window(widgets.clone(), addresses, database);
    }
    fn build_tcp_packet(widgets: Rc<RefCell<MainWindowWidgets>>, database: Rc<RefCell<Database>>) {
        let addresses = match widgets.borrow().ip_widgets.get_addresses() {
            Some(addresses) => addresses,
            None => { error("Bad src or destination IP address value."); return }
        };

        let packet = match widgets.borrow().tcp_widgets.build_packet(addresses) {
            Some(packet) => packet,
            None => { return }
        };
        show("TCP packet", &packet);
        Self::build_ip_packet(widgets, packet, IpNextHeaderProtocol::new(6), database, "TCP");
    }
    fn build_ip_packet(widgets: Rc<RefCell<MainWindowWidgets>>, data: Vec<u8>, next_protocol: IpNextHeaderProtocol, database: Rc<RefCell<Database>>, label: &str) {
        let packet = match widgets.borrow().ip_widgets.build_packet(next_protocol, &data) {
            Some(packet) => packet,
            None => { return }
        };
        Self::build_frame(widgets.clone(), &packet, database, label);
    }
    pub(crate) fn build_frame(widgets: Rc<RefCell<MainWindowWidgets>>, data: &Vec<u8>, database: Rc<RefCell<Database>>, label: &str) {
        let mut frame = MutableEthernetPacket::owned(vec![0u8; MutableEthernetPacket::minimum_packet_size() + data.len()]).unwrap();

        match widgets.borrow().macs.get() {
            Some(addresses) => { frame.set_source(addresses.0); frame.set_destination(addresses.1); },
            None => return
        }

        frame.set_ethertype(EtherType::new(0x0800));
        frame.set_payload(data);

        let interface = widgets.borrow().interface_list.get_active();

        let payload = Vec::from(frame.packet());
        show("Ethernet frame", &payload);
        database.borrow_mut().push(payload.clone(), label);
        Self::send_frame(&payload, &interface);
    }

    fn send_frame(payload: &Vec<u8>, iface: &str) {
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

        match tx.send_to(&payload, None) {
            Some(_) => {},
            None => { error("Failed to send packet."); return }
        }
    }
}

pub struct MainWindow {
    widgets: Rc<RefCell<MainWindowWidgets>>,
    window: gtk::ApplicationWindow
}
impl MainWindow {
    pub(crate) fn new(app: &gtk::Application) -> Self {
        let widgets = Rc::new(RefCell::new(MainWindowWidgets::new()));
        let database = Rc::new(RefCell::new(Database::new()));

        let database_ui = Database::get_ui_section(widgets.clone(), database.clone());

        let button = gtk::Button::with_label("Collect");
        let ui = widgets.borrow().generate_ui(&button, &database_ui);

        let clone = widgets.clone();
        let database_clone = database.clone();
        button.connect_clicked(move |_| {
            MainWindowWidgets::build_packet(clone.clone(), database_clone.clone());
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