
use crate::hal::Hal;
use crate::transport::Transport;


const QUEUE: u16 = 0;
const QUEUE_SIZE: u16 = 16;

pub struct VirtioBlk<H: Hal, T: Transport>{
    transport: T,
    queue: VirtQueue<H, { QUEUE_SIZE as usize }>,
    capacity: u64,
    readonly: bool,
}


impl<H: Hal, T: Transport> VirtIOBlk<H, T> {
    
}
