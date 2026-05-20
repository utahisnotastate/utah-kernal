use linked_list_allocator::LockedHeap;

// This creates a global memory manager.
// Think of it as a librarian that hands out blank pages of memory when programs ask for them.
#[global_allocator]
static SYSTEM_MEMORY_ALLOCATOR: LockedHeap = LockedHeap::empty();

// We decide where our usable memory begins and how big it is.
// We start at the 1 Megabyte (MB) mark and reserve 100 MB of total space.
pub const STARTING_MEMORY_ADDRESS: usize = 0x0010_0000;
pub const TOTAL_MEMORY_SIZE: usize = 100 * 1024 * 1024;

/// This function opens the memory library so programs can start borrowing memory.
pub fn initialize_system_heap() {
    unsafe {
        SYSTEM_MEMORY_ALLOCATOR
            .lock()
            .init(STARTING_MEMORY_ADDRESS as *mut u8, TOTAL_MEMORY_SIZE);
    }
}

/// If a program asks for memory but the computer is full, this function handles the error.
#[alloc_error_handler]
fn handle_out_of_memory_error(memory_layout: core::alloc::Layout) -> ! {
    panic!(
        "The system ran out of memory! Requested size: {:?}",
        memory_layout
    );
}
