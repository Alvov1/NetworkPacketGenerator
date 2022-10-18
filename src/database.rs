use std::cell::RefCell;
use std::rc::Rc;
use pnet::packet::ethernet::MutableEthernetPacket;
use queues::Queue;

struct Database {
    queue: Queue<MutableEthernetPacket>,
    pub current: Rc<RefCell<MutableEthernetPacket>>
}
impl Database() {
    fn new() -> Self {
        Self {
            queue: Queue::new(),
            current: Rc::new(
                RefCell::new(
                    MutableEthernetPacket::owned(
                        vec![0u8; MutableEthernetPacket::minimum_packet_size()])))
        }
    }
}