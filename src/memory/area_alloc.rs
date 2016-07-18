use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};

/// The `AreaFrameAllocator` type.
pub struct AreaFrameAllocator {
    /// The next free frame.
    next_frame: Frame,

    /// The current memory area.
    area: Option<&'static MemoryArea>,

    /// All memory areas.
    areas: MemoryAreaIter,

    /// The start address of the kernel.
    kernel_start: Frame,

    /// The end address of the kernel.
    kernel_end: Frame,

    /// The start address of the multiboot2 data.
    mb2_start: Frame,

    /// The end address of the multiboot2 data.
    mb2_end: Frame,
}

/// The `FrameAllocator` implementation for `AreaFrameAllocator`.
impl FrameAllocator for AreaFrameAllocator {
    /// Allocates a frame.
    fn alloc_frame(&mut self) -> Option<Frame> {
        // Test if the current area is invalid
        if self.area.is_none() {
            return None;
        }

        // Get the current area and the next free frame
        let area = self.area.unwrap();
        let frame = Frame { index: self.next_frame.index };

        // Get the last frame of the current area
        let last_frame = {
            let addr = area.base_addr + area.length - 1;
            Frame::get_frame_for_address(addr as usize)
        };

        // Test if the frame exceeds the bounds of the current area
        if frame > last_frame {
            self.find_free_area();
        }
        // Test if the frame is within the bounds of the kernel
        else if frame >= self.kernel_start && frame <= self.kernel_end {
            self.next_frame = Frame { index: self.kernel_end.index + 1 };
        }
        // Test if the frame is within the bounds of the multiboot2 data
        else if frame >= self.mb2_start && frame <= self.mb2_end {
            self.next_frame = Frame { index: self.mb2_end.index + 1 };
        } else {
            self.next_frame.index += 1;
            return Some(frame);
        }

        // Try allocating a new frame
        self.alloc_frame()
    }

    /// Deallocates a frame.
    fn dealloc_frame(&mut self, frame: Frame) {
        unimplemented!();
    }
}

/// The `AreaFrameAllocator` implementation.
impl AreaFrameAllocator {
    /// Constructs a new `AreaFrameAllocator`.
    pub fn new(kernel_start: usize,
               kernel_end: usize,
               mb2_start: usize,
               mb2_end: usize,
               areas: MemoryAreaIter)
               -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_frame: Frame::get_frame_for_address(0),
            area: None,
            areas: areas,
            kernel_start: Frame::get_frame_for_address(kernel_start),
            kernel_end: Frame::get_frame_for_address(kernel_end),
            mb2_start: Frame::get_frame_for_address(mb2_start),
            mb2_end: Frame::get_frame_for_address(mb2_end),
        };
        allocator.find_free_area();
        allocator
    }

    /// Finds a free memory area.
    fn find_free_area(&mut self) {
        self.area = self.areas
            .clone()
            .filter(|area| {
                let addr = area.base_addr + area.length - 1;
                Frame::get_frame_for_address(addr as usize) >= self.next_frame
            })
            .min_by_key(|area| area.base_addr);
        match self.area {
            Some(area) => {
                let start_frame = Frame::get_frame_for_address(area.base_addr as usize);
                if self.next_frame < start_frame {
                    self.next_frame = start_frame;
                }
            }
            _ => (),
        }
    }
}
