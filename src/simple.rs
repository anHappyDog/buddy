use sync::spin::Spinlock;
use core::ptr;
use core::alloc::{GlobalAlloc, Layout};

pub struct SimpleAllocator {
    memory: &'static mut [u8],
    offset: Spinlock<usize>,
}

impl SimpleAllocator {
    pub const fn new(memory: &'static mut [u8]) -> SimpleAllocator {
        SimpleAllocator {
            memory: memory,
            offset: Spinlock::<usize>::new(0),
        }
    }
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.lock();
        let start = *offset;
        let end = start + layout.size();
        *offset = end;
        if end > self.memory.len() {
            ptr::null_mut()
        } else {
            &self.memory[start] as *const u8 as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // not implemented
    }
}



