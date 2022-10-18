use std::cell::RefCell;
use std::rc::Rc;
use pnet::packet::ethernet::MutableEthernetPacket;
use queues::Queue;

struct Database<'a> {
    queue: Queue<Vec<u8>>,
    pub current: Rc<RefCell<MutableEthernetPacket<'a>>>
}
impl Database<'_> {
    fn new() -> Self {
        Self {
            queue: Queue::new(),
            current: Rc::new(
                RefCell::new(
                    MutableEthernetPacket::owned(
                        vec![0u8; MutableEthernetPacket::minimum_packet_size()]).unwrap()))
        }
    }
}