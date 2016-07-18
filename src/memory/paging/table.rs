use core::ops::{Index, IndexMut};
use core::marker::PhantomData;
use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use memory::FrameAllocator;

/// The level 4 table.
pub const LEVEL4_TABLE: *mut Table<Level4> = 0xfffffffffffff000 as *mut _;

/// The `TableLevel` trait.
pub trait TableLevel {}

/// The `Level4` type.
///
/// Represents a level-4 page table.
pub enum Level4 {}

/// The `Level3` type.
///
/// Represents a level-3 page table.
pub enum Level3 {}

/// The `Level2` type.
///
/// Represents a level-2 page table.
pub enum Level2 {}

/// The `Level1` type.
///
/// Represents a level-1 page table.
pub enum Level1 {}

/// The `TableLevel` implementation for `Level4`.
impl TableLevel for Level4 {}

/// The `TableLevel` implementation for `Level3`.
impl TableLevel for Level3 {}

/// The `TableLevel` implementation for `level2`.
impl TableLevel for Level2 {}

/// The `TableLevel` implementation for `level1`.
impl TableLevel for Level1 {}

/// The `HierarchicalLevel` trait.
pub trait HierarchicalLevel: TableLevel {
    /// The next level.
    type NextLevel: TableLevel;
}

/// The `HierarchicalLevel` implementation for `Level4`.
impl HierarchicalLevel for Level4 {
    /// The next level.
    type NextLevel = Level3;
}

/// The `HierarchicalLevel` implementation for `Level3`.
impl HierarchicalLevel for Level3 {
    /// The next level.
    type NextLevel = Level2;
}

/// The `HierarchicalLevel` implementation for `Level2`.
impl HierarchicalLevel for Level2 {
    /// The next level.
    type NextLevel = Level1;
}

/// The `Table` type.
///
/// Represents a page table.
pub struct Table<L: TableLevel> {
    /// The entries.
    entries: [Entry; ENTRY_COUNT],

    /// The table level.
    level: PhantomData<L>,
}

/// The `Index` implementation for `Table`.
impl<L> Index<usize> for Table<L>
    where L: TableLevel
{
    type Output = Entry;
    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

/// The `IndexMut` implementation for `Table`.
impl<L> IndexMut<usize> for Table<L>
    where L: TableLevel
{
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

/// The `Table` implemenation.
impl<L> Table<L>
    where L: TableLevel
{
    /// Zero-fill the entries.
    pub fn zero_fill(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.mark_unused();
        }
    }
}

/// The `Table` implementation.
impl<L> Table<L>
    where L: HierarchicalLevel
{
    /// Creates the next table.
    pub fn create_next_table<A>(&mut self,
                                index: usize,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE));
            let frame = allocator.alloc_frame().unwrap();
            self.entries[index].set_flags(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero_fill();
        }
        self.next_table_mut(index).unwrap()
    }

    /// Gets the address of the next table.
    pub fn next_table_addr(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_addr = self as *const _ as usize;
            Some((table_addr << 9) | (index << 12))
        } else {
            None
        }
    }

    /// Gets the next table.
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_addr(index).map(|addr| unsafe { &*(addr as *mut _) })
    }

    /// Gets the next table as mutable.
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_addr(index).map(|addr| unsafe { &mut *(addr as *mut _) })
    }
}
