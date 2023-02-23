// use std::{ptr::NonNull, sync::{Mutex, Arc}};
// use crate::{
//     // queue::{fake_read_write_queue, Descriptor},
//     PhysAddr, Result,
// };
// use super::{DeviceStatus, DeviceType, Transport};




// /// A fake implementation of [`Transport`] for unit tests.
// #[derive(Debug)]
// pub struct FakeTransport<C:'static>{
//     pub device_type: DeviceType,
//     pub max_queue_size: u32,
//     pub device_features: u64,
//     pub config_space: NonNull<C>,
//     pub state: Arc<Mutex<State>>,
// }


// impl<C> Transport for FakeTransport<C> {
//     fn device_type(&self) -> DeviceType {
//         self.device_type
//     }

//     fn read_device_features(&mut self) -> u64 {
//         self.device_features  
//       }

//     fn write_driver_features(&mut self, driver_features: u64) {
//         self.state.lock().unwrap().driver_features = driver_features;
//     }

//     fn max_queue_size(&self) -> u32 {
//         self.max_queue_size
//     }

//     fn notify(&mut self, queue: u16) {
//         self.state.lock().unwrap().queues[queue as usize].notified = true;
//     }

//     fn set_status(&mut self, status: DeviceStatus) {
//         self.state.lock().unwrap().status = status;

//     }

//     fn set_guest_page_size(&mut self, guest_page_size: u32) {
//         self.state.lock().unwrap().guest_page_size = guest_page_size;


//     }

//     fn requires_legacy_layout(&self) -> bool {
//         false
//     }

//     fn queue_set(
//         &mut self,
//         queue: u16,
//         size: u32,
//         descriptors: crate::PhysAddr,
//         driver_area: crate::PhysAddr,
//         device_area: crate::PhysAddr,
//     ) {
//         todo!()
//     }

//     fn queue_unset(&mut self, queue: u16) {
//         todo!()
//     }

//     fn queue_used(&mut self, queue: u16) -> bool {
//         todo!()
//     }

//     fn ack_interrupt(&mut self) -> bool {
//         todo!()
//     }

//     fn config_space<T: 'static>(&self) -> crate::Result<NonNull<T>> {
//         todo!()
//     }
// }

// #[derive(Debug, Default)]
// pub struct State {
//     pub status: DeviceStatus,
//     pub driver_features: u64,
//     pub guest_page_size: u32,
//     pub interrupt_pending: bool,
//     pub queues: Vec<QueueStatus>,
// }

// impl State {
//     /// Simulates the device writing to the given queue.
//     ///
//     /// The fake device always uses descriptors in order.
//     pub fn write_to_queue<const QUEUE_SIZE: usize>(&mut self, queue_index: u16, data: &[u8]) {
//         let queue = &self.queues[queue_index as usize];
//         assert_ne!(queue.descriptors, 0);
//         // fake_read_write_queue(
//         //     queue.descriptors as *const [Descriptor; QUEUE_SIZE],
//         //     queue.driver_area as *const u8,
//         //     queue.device_area as *mut u8,
//         //     |input| {
//         //         assert_eq!(input, Vec::new());
//         //         data.to_owned()
//         //     },
//         // );
//     }

//     /// Simulates the device reading from the given queue.
//     ///
//     /// Data is read into the `data` buffer passed in. Returns the number of bytes actually read.
//     ///
//     /// The fake device always uses descriptors in order.
//     pub fn read_from_queue<const QUEUE_SIZE: usize>(&mut self, queue_index: u16) -> Vec<u8> {
//         let queue = &self.queues[queue_index as usize];
//         assert_ne!(queue.descriptors, 0);

//         let mut ret = None;

//         // Read data from the queue but don't write any response.
//         fake_read_write_queue(
//             queue.descriptors as *const [Descriptor; QUEUE_SIZE],
//             queue.driver_area as *const u8,
//             queue.device_area as *mut u8,
//             |input| {
//                 ret = Some(input);
//                 Vec::new()
//             },
//         );

//         ret.unwrap()
//     }

//     /// Simulates the device reading data from the given queue and then writing a response back.
//     ///
//     /// The fake device always uses descriptors in order.
//     pub fn read_write_queue<const QUEUE_SIZE: usize>(
//         &mut self,
//         queue_index: u16,
//         handler: impl FnOnce(Vec<u8>) -> Vec<u8>,
//     ) {
//         let queue = &self.queues[queue_index as usize];
//         assert_ne!(queue.descriptors, 0);
//         // fake_read_write_queue(
//         //     queue.descriptors as *const [Descriptor; QUEUE_SIZE],
//         //     queue.driver_area as *const u8,
//         //     queue.device_area as *mut u8,
//         //     handler,
//         // )
//     }
// }

// #[derive(Clone, Debug, Default, Eq, PartialEq)]
// pub struct QueueStatus {
//     pub size: u32,
//     pub descriptors: PhysAddr,
//     pub driver_area: PhysAddr,
//     pub device_area: PhysAddr,
//     pub notified: bool,
// }
