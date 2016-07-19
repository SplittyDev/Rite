use core::ptr::Unique;
use core::ops::{Deref, DerefMut};

mod mapper;
mod entry;
mod table;
mod temp_page;

pub use self::entry::*;
pub use self::table::{Level1, Table};

use super::{Frame, PAGE_SIZE};
use super::FrameAllocator;
use self::table::{Level4, LEVEL4_TABLE};
use self::temp_page::TemporaryPage;
use self::mapper::Mapper;

/// The number of entries.
const ENTRY_COUNT: usize = 512;

/// The `PhysicalAddress` type.
pub type PhysicalAddress = usize;

/// The `VirtualAddress` type.
pub type VirtualAddress = usize;

/// The `ActivePageTable` type.
pub struct ActivePageTable {
    /// The mapper.
    mapper: Mapper,
}

/// The `Deref` implementation for `ActivePageTable`.
impl Deref for ActivePageTable {
    /// The target.
    type Target = Mapper;

    /// Dereferences the `ActivePageTable` into `Mapper`.
    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

/// The `DerefMut` implementation for `ActivePageTable`.
impl DerefMut for ActivePageTable {
    /// Dereferences the `ActivePageTable` into mutable `Mapper`.
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

/// The `ActivePageTable` implementation.
impl ActivePageTable {
    /// Constructs a new `ActivePageTable`.
    unsafe fn new() -> ActivePageTable {
        ActivePageTable { mapper: Mapper::new() }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temp_page::TemporaryPage,
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        let flush_tlb = || unsafe {
            // Invalidate the translation lookaside buffer
            let cr3: usize;
            asm!("mov %cr3, $0" : "=r" (cr3));
            asm!("mov $0, %cr3" :: "r" (cr3) : "memory");
        };
        {
            let backup = Frame::get_frame_for_address(unsafe {
                let cr3: usize;
                asm!("mov %cr3, $0" : "=r" (cr3));
                cr3
            });
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);
            self.p4_mut()[511].set_flags(table.p4_frame.clone(), PRESENT | WRITABLE);
            flush_tlb();
            f(self);
            p4_table[511].set_flags(backup, PRESENT | WRITABLE);
            flush_tlb();
        }
        temporary_page.unmap(self);
    }
}

/// The `InactivePageTable` type.
pub struct InactivePageTable {
    /// The level 4 page frame.
    p4_frame: Frame,
}

/// The `InactivePageTable` implementation.
impl InactivePageTable {
    /// Constructs a new `InactivePageTable`.
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero_fill();
            table[511].set_flags(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);
        InactivePageTable { p4_frame: frame }
    }
}

/// The `Page` type.
#[derive(Debug, Copy, Clone)]
pub struct Page {
    /// The index.
    index: usize,
}

/// The `Page` implementation.
impl Page {
    /// Gets the page at the specified address.
    pub fn get_page_at_address(addr: VirtualAddress) -> Page {
        Page { index: addr / PAGE_SIZE }
    }

    /// Gets the address.
    pub fn address(&self) -> usize {
        self.index * PAGE_SIZE
    }

    /// Gets the P4 index.
    fn p4_index(&self) -> usize {
        (self.index >> 27) & 0o777
    }

    /// Gets the P3 index.
    fn p3_index(&self) -> usize {
        (self.index >> 18) & 0o777
    }

    /// Gets the P2 index.
    fn p2_index(&self) -> usize {
        (self.index >> 9) & 0o777
    }

    /// Gets the P1 index.
    fn p1_index(&self) -> usize {
        self.index & 0o777
    }
}
