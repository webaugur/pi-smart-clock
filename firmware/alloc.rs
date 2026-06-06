//! Simple bump allocator for RP2040 (Cortex-M0+ — single executor thread).

use ::core::alloc::{GlobalAlloc, Layout};
use ::core::ptr::null_mut;

static mut HEAP_START: usize = 0;
static mut HEAP_END: usize = 0;
static mut HEAP_NEXT: usize = 0;

pub const HEAP_SIZE: usize = 128 * 1024;

pub fn init_heap(buffer: &mut [u8; HEAP_SIZE]) {
    let start = buffer.as_ptr() as usize;
    unsafe {
        HEAP_START = start;
        HEAP_END = start + HEAP_SIZE;
        HEAP_NEXT = start;
    }
}

struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size().max(1);
        let mut next = HEAP_NEXT;
        next = (next + align - 1) & !(align - 1);
        let end = next + size;
        if end > HEAP_END {
            return null_mut();
        }
        HEAP_NEXT = end;
        next as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;