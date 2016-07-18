use memory::Frame;

// The page table entry bit flags
bitflags! {
    pub flags EntryFlags: u64 {
        const PRESENT =         1 << 0,
        const WRITABLE =        1 << 1,
        const USER_ACCESSIBLE = 1 << 2,
        const WRITE_THROUGH =   1 << 3,
        const NO_CACHE =        1 << 4,
        const ACCESSED =        1 << 5,
        const DIRTY =           1 << 6,
        const HUGE_PAGE =       1 << 7,
        const GLOBAL =          1 << 8,
        const NO_EXECUTE =      1 << 63,
    }
}

/// The `Entry` type.
pub struct Entry(u64);

/// The `Entry` implementation.
impl Entry {
    /// Test if the entry is unused.
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    /// Mark the entry as unused.
    pub fn mark_unused(&mut self) {
        self.0 = 0;
    }

    /// Gets the entry bit flags.
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    /// Gets the frame.
    pub fn frame(&self) -> Option<Frame> {
        if self.flags().contains(PRESENT) {
            Some(Frame::get_frame_for_address(self.0 as usize & 0x000ffffffffff000))
        } else {
            None
        }
    }

    /// Sets entry flags for the specified frame.
    pub fn set_flags(&mut self, frame: Frame, flags: EntryFlags) {
        assert!(frame.get_start_address() & !0x000ffffffffff000 == 0);
        self.0 = (frame.get_start_address() as u64) | flags.bits();
    }
}
