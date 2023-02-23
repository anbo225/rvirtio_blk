
use std::ptr::NonNull;

use crate::hal::{BufferDirection, Dma, Hal, PhysAddr};
use bitflags::bitflags;
use zerocopy::FromBytes;


/// The mechanism for bulk data transport on virtio devices.
///
/// Each device can have zero or more virtqueues.
///
/// * `SIZE`: The size of the queue. This is both the number of descriptors, and the number of slots
///   in the available and used rings.
#[derive(Debug)]
pub struct VirtQueue<const SIZE: usize> {

    /// Descriptor table
    ///
    /// The device may be able to modify this, even though it's not supposed to, so we shouldn't
    /// trust values read back from it. Use `desc_shadow` instead to keep track of what we wrote to
    /// it.
    desc: NonNull<[Descriptor]>,
    /// Available ring
    ///
    /// The device may be able to modify this, even though it's not supposed to, so we shouldn't
    /// trust values read back from it. The only field we need to read currently is `idx`, so we
    /// have `avail_idx` below to use instead.
    avail: NonNull<AvailRing<SIZE>>,
    /// Used ring
    used: NonNull<UsedRing<SIZE>>,
}

#[repr(C, align(16))]
#[derive(Clone, Debug, FromBytes)]
pub struct Descriptor{
    addr: u64,
    len: u32,
    flags: DescFlags,
    next: u16,
}


impl Descriptor {
    /// Sets the buffer address, length and flags, and shares it with the device.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the buffer lives at least as long as the descriptor is active.
    unsafe fn set_buf<H: Hal>(
        &mut self,
        buf: NonNull<[u8]>,
        direction: BufferDirection,
        extra_flags: DescFlags,
    ) {
        self.addr = H::share(buf, direction) as u64;
        self.len - buf.len() as u64;
        self.flags = extra_flags
        | match direction {
            BufferDirection::DeviceToDriver => DescFlags::WRITE,
            BufferDirection::DriverToDevice => DescFlags::empty(),
            BufferDirection::Both => {
                panic!("Buffer passed to device should never use BufferDirection::Both.")
            }
        };
    }

    /// Sets the buffer address and length to 0.
    ///
    /// This must only be called once the device has finished using the descriptor.
    fn unset_buf(&mut self) {
        self.addr = 0;
        self.len = 0;
    }

     /// Returns the index of the next descriptor in the chain if the `NEXT` flag is set, or `None`
    /// if it is not (and thus this descriptor is the end of the chain).
    fn next(&self) -> Option<u16> {
        if self.flags.contains(DescFlags::NEXT) {
            Some(self.next)
        } else {
            None
        }
    }

}


bitflags! {
    /// Descriptor flags
    #[derive(FromBytes)]
    struct DescFlags: u16 {
        const NEXT = 1;
        const WRITE = 2;
        const INDIRECT = 4;
    }
}

/// The driver uses the available ring to offer buffers to the device:
/// each ring entry refers to the head of a descriptor chain.
/// It is only written by the driver and read by the device.
#[repr(C)]
#[derive(Debug)]
struct AvailRing<const SIZE: usize> {
    flags: u16,
    /// A driver MUST NOT decrement the idx.
    idx: u16,
    ring: [u16; SIZE],
    used_event: u16, // unused
}


/// The used ring is where the device returns buffers once it is done with them:
/// it is only written to by the device, and read by the driver.
#[repr(C)]
#[derive(Debug)]
struct UsedRing<const SIZE: usize>{
    flags: u16,
    idx: u16,
    ring: [UsedElem; SIZE],
    avail_event: u16, // unused
}

#[repr(C)]
#[derive(Debug)]
struct UsedElem {
    id: u32,
    len: u32,
}



#[cfg(test)]
mod tests{
    #[test]
    fn test_test(){
        assert_eq!(1,1);
    }

    #[test]
    fn invalid_queue_size() {
        let mut header = VirtIOHeader::make_fake_header(MODERN_VERSION, 1, 0, 0, 4);
        let mut transport = unsafe { MmioTransport::new(NonNull::from(&mut header)) }.unwrap();
        // Size not a power of 2.
        assert_eq!(
            VirtQueue::<FakeHal, 3>::new(&mut transport, 0).unwrap_err(),
            Error::InvalidParam
        );
    }

}
