# NetworkPacketGenerator

Packet generator for the TCP/IP Internet stack. Allows to generate IPv4, TCP, UDP and ICMP packets, configure packet fields, send pre-generated packets in one sequence, send multiple pre-generated packets at the same time. Source MAC address and empty packet fields are filled in automatically.

The development was carried out in the Rust with the usage of libpnet. The gtk library was used to develop the graphical interface. Application's main window interface is listed below:

<img width="769" alt="Screenshot 2022-12-09 at 19 48 43" src="https://user-images.githubusercontent.com/39829227/206752675-43f221ed-c4f0-4617-b3c2-6811a100b215.png">

UDP and ICMP protocol packets are generated using additional windows. UDP window's interface is listed below:

<img width="633" alt="Screenshot 2022-12-09 at 19 53 34" src="https://user-images.githubusercontent.com/39829227/206754126-6d745919-e952-498f-b04e-79b5d86050e7.png">

ICMP protocol's additional window has the folowing interface:

<img width="578" alt="Screenshot 2022-12-09 at 19 53 59" src="https://user-images.githubusercontent.com/39829227/206754316-e6ccca6b-9a98-44f5-8b9a-59b12a732204.png">

ICMP protocol generation supports sending ICMP request and ICMP response packets:

<img width="578" alt="Screenshot 2022-12-09 at 19 54 02" src="https://user-images.githubusercontent.com/39829227/206754837-a6fb4798-f8b1-4e4f-bdf5-9f13e0b6b52b.png">

Application has the folowing libraty dependencies:

- gtk = { version = "0.4.8", package = "gtk4" }
- pnet = "0.30.0"
- mac_address = "1.1.4"
- rand = "0.8.5"
