use super::{Page, Table, Level1, ActivePageTable, VirtualAddress};
use memory::{Frame, FrameAllocator};

/// The `TinyAllocator` type.
struct TinyAllocator([Option<Frame>; 3]);

/// The `FrameAllocator` implementation for `TinyAllocator`.
impl FrameAllocator for TinyAllocator {
    /// Allocates the frame.
    fn alloc_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    /// Deallocates the frame.
    fn dealloc_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("TinyAllocator can only hold three frames.");
    }
}

/// The `TinyAllocator` implementation.
impl TinyAllocator {
    /// Constructs a new `TinyAllocator`.
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
    {
        let mut f = || allocator.alloc_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}

/// The `TemporaryPage` type.
pub struct TemporaryPage {
    /// The page.
    page: Page,

    /// The allocator.
    allocator: TinyAllocator,
}

/// The `TemporaryPage` implementation.
impl TemporaryPage {
    /// Constructs a new `TemporaryPage`.
    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator
    {
        TemporaryPage {
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }
    /// Maps the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::WRITABLE;
        assert!(active_table.translate_page(self.page).is_none());
        active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);
        self.page.address()
    }

    /// Unmaps the temporary page.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator);
    }

    /// Maps a temporary page to the specified frame.
    pub fn map_table_frame(&mut self,
                           frame: Frame,
                           active_table: &mut ActivePageTable)
                           -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }
}
