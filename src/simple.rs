use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sync::spin::Spinlock;

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
        let mem_start = self.memory.as_ptr() as usize;

        let start = (*offset + mem_start + layout.align() - 1) & !(layout.align() - 1);
        let start = start - mem_start;
        let end = start + layout.size();
        *offset = end;
        if end > self.memory.len() {
            panic!("SimpleAllocator: out of memory");
        } else {
            &self.memory[start] as *const u8 as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // not implemented
    }
}
