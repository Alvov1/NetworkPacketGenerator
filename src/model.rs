
extern crate queues;
use queues::*;
use crate::ethernet::EthernetFrame;

pub(crate) struct Model {
    queue: Queue<EthernetFrame>,
    current: EthernetFrame
}
impl Model {
    fn new() -> Model {
        Model { queue: Queue::new(), current: EthernetFrame::new() }
    }
}