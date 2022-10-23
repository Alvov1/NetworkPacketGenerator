use std::sync::Arc;
use std::sync::Mutex;

use gtk::prelude::*;
use pnet::datalink;

use crate::icmp;
use crate::udp;
use crate::ip::IPWidgets;
use crate::tcp::TCPWidgets;
use crate::udp::UdpOptions;
use crate::icmp::IcmpOptions;

pub struct MainWindowWidgets {
    interface_list: gtk::DropDown,

    ip_button: Arc<Mutex<gtk::CheckButton>>,
    icmp_button: gtk::CheckButton,
    tcp_button: gtk::CheckButton,
    udp_button: gtk::CheckButton,

    src_mac_entry: gtk::Entry,
    dest_mac_entry: gtk::Entry,

    ip_widgets: IPWidgets,
    tcp_widgets: TCPWidgets,

    main_button: gtk::Button
}
impl MainWindowWidgets {
    fn generate_ui(&self) -> gtk::Box {
        let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_top(24).margin_bottom(24)
            .margin_start(24).margin_end(24).halign(gtk::Align::Center).valign(gtk::Align::Center).spacing(24).build();

        /* First section. */ {
            let section_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Center)
                .margin_start(24).margin_end(24).valign(gtk::Align::Center).spacing(24).build();

            /* Initialize first section. */
            section_box.append(&gtk::Label::new(Some("Interface:")));
            section_box.append(&self.interface_list);
            section_box.append(&self.get_protocol_table());
            section_box.append(&self.ip_widgets.prepare_address_section());

            /* Add main button. */
            let button = self.ip_button.clone();
            self.main_button.connect_clicked(move |_| {
                if button.lock().unwrap().is_active() {}
                println!("{:?}", icmp::IcmpOptions::show_window());
            });
            section_box.append(&self.main_button);

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

        self.tcp_button.set_group(Some(&self.udp_button));
        self.icmp_button.set_group(Some(&self.tcp_button));
        self.ip_button.lock().unwrap().set_group(Some(&self.icmp_button));

        protocol_table.attach(&*self.ip_button.lock().unwrap(), 0, 0, 1, 1);
        protocol_table.attach(&self.icmp_button, 1, 0, 1, 1);
        protocol_table.attach(&self.tcp_button, 0, 1, 1, 1);
        protocol_table.attach(&self.udp_button, 1, 1, 1, 1);

        protocol_table
    }
    fn get_mac_address_table(&self) -> gtk::Grid {
        let grid = gtk::Grid::builder().margin_start(24).margin_end(24).halign(gtk::Align::Center)
            .valign(gtk::Align::Center).row_spacing(24).column_spacing(24).build();

        let source_lable = gtk::Label::builder().label("Source MAC").halign(gtk::Align::Start).build();
        grid.attach(&source_lable, 0, 0, 1, 1);
        grid.attach(&self.src_mac_entry, 1, 0, 1, 1);

        let destination_lable = gtk::Label::builder().label("Destination MAC").halign(gtk::Align::Start).build();
        grid.attach(&destination_lable, 2, 0, 1, 1);
        grid.attach(&self.dest_mac_entry, 3, 0, 1, 1);

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
        let interfaces: Vec<_> = binding.iter().map(|v| &*v.name).collect();
        Self {
            interface_list: gtk::DropDown::from_strings(&interfaces),

            ip_button: Arc::new(Mutex::new(gtk::CheckButton::builder().label("IP").active(true).build())),
            icmp_button: gtk::CheckButton::with_label("ICMP"),
            tcp_button: gtk::CheckButton::with_label("TCP"),
            udp_button: gtk::CheckButton::with_label("UDP"),

            src_mac_entry: gtk::Entry::builder().placeholder_text("Source MAC").build(),
            dest_mac_entry: gtk::Entry::builder().placeholder_text("Destination MAC").build(),

            ip_widgets: IPWidgets::new(),
            tcp_widgets: TCPWidgets::new(),

            main_button: gtk::Button::with_label("Collect")
        }
    }
    fn build_packet(&self) {
        if self.udp_button.is_active() {
            let packet = UdpOptions::show_window();
        }
        if self.icmp_button.is_active() {
            let packet = IcmpOptions::show_window();
        }
    }
}

pub struct MainWindow {
    widgets: MainWindowWidgets,
    window: gtk::ApplicationWindow
}
impl MainWindow {
    pub(crate) fn new(app: &gtk::Application) -> Self {
        let widgets = MainWindowWidgets::new();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Network Packet Generator")
            .default_width(900)
            .default_height(500)
            .child(&widgets.generate_ui())
            .build();

        Self { widgets, window }
    }
    pub(crate) fn show(&self) { self.window.show(); }
}