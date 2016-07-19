/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

mod area_alloc;
pub use self::area_alloc::AreaFrameAllocator;
mod paging;
use self::paging::PhysicalAddress;

/// The `Frame` type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    /// The index of the page.
    ///
    /// Calculated in the following way:
    /// ```
    /// index = addr / PAGE_SIZE;
    /// ```
    pub index: usize,
}

/// The `Frame` implementation.
impl Frame {
    /// Gets the frame that corresponds to
    /// the specified physical address.
    fn get_frame_for_address(addr: usize) -> Frame {
        Frame { index: addr / PAGE_SIZE }
    }

    /// Clones the frame.
    fn clone(&self) -> Frame {
        Frame { index: self.index }
    }

    /// Gets the start address of the frame.
    ///
    /// Calculated in the following way:
    /// ```
    /// addr = index * PAGE_SIZE
    /// ```
    fn get_start_address(&self) -> PhysicalAddress {
        self.index * PAGE_SIZE
    }
}

/// The `FrameAllocator` trait.
pub trait FrameAllocator {
    /// Allocates a frame.
    fn alloc_frame(&mut self) -> Option<Frame>;

    /// Deallocates a frame.
    fn dealloc_frame(&mut self, frame: Frame);
}
