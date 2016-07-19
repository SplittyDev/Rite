use core::ptr::Unique;
use super::{VirtualAddress, PhysicalAddress, Page, ENTRY_COUNT};
use super::entry::*;
use super::table::{self, Table, Level4, Level1};
use memory::{PAGE_SIZE, Frame, FrameAllocator};

/// The `Mapper` type.
pub struct Mapper {
    /// The level4 page table.
    p4: Unique<Table<Level4>>,
}

/// The `Mapper` implementation.
impl Mapper {
    /// Constructs a new `Mapper`.
    pub unsafe fn new() -> Mapper {
        Mapper { p4: Unique::new(table::LEVEL4_TABLE) }
    }

    /// Gets the level4 page table.
    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.get() }
    }

    /// Gets the level4 page table as mutable.
    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.get_mut() }
    }

    /// Translates a virtual address into a physical address.
    pub fn translate(&self, virtual_addr: VirtualAddress) -> Option<PhysicalAddress> {
        let off = virtual_addr % PAGE_SIZE;
        self.translate_page(Page::get_page_at_address(virtual_addr))
            .map(|frame| frame.index * PAGE_SIZE + off)
    }

    /// Translates a page into a frame.
    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        use super::entry::HUGE_PAGE;
        let p3 = self.p4().next_table(page.p4_index());
        let huge_page = || unimplemented!();
        p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].frame())
    }

    /// Maps a page to a frame using the specified allocator.
    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let mut p3 = self.p4_mut().create_next_table(page.p4_index(), allocator);
        let mut p2 = p3.create_next_table(page.p3_index(), allocator);
        let mut p1 = p2.create_next_table(page.p2_index(), allocator);
        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set_flags(frame, flags | PRESENT);
    }

    /// Maps the next free page using the specified allocator.
    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.alloc_frame().unwrap();
        self.map_to(page, frame, flags, allocator)
    }

    /// Identity maps a frame using the specified allocator.
    pub fn identitiy_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::get_page_at_address(frame.get_start_address());
        self.map_to(page, frame, flags, allocator)
    }

    /// Unmaps a page using the specified allocator.
    pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        assert!(self.translate(page.address()).is_some());
        let p1 = self.p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .unwrap();
        let frame = p1[page.p1_index()].frame().unwrap();
        p1[page.p1_index()].mark_unused();
        unsafe {
            // Flush translation lookaside buffer
            asm!("invlpg ($0)" :: "r" (page.address()) : "memory");
        }
        allocator.dealloc_frame(frame);
    }
}
