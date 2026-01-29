use crate::{
    alloc::{GlobalAlloc, Layout, System},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(align(4096))]
struct HeapData([u8; 65536]);

static mut HEAP_DATA: HeapData = HeapData([0; 65536]);
static HEAP_USED: AtomicUsize = AtomicUsize::new(0);

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > 16 {
            return ptr::null_mut();
        }
        let num_blocks = if layout.size() % 16 == 0 {
            layout.size() / 16
        } else {
            (layout.size() / 16) + 1
        };
        let index = HEAP_USED.fetch_add(num_blocks, Ordering::Relaxed) * 16;

        unsafe { (&raw mut HEAP_DATA as *mut u8).offset(index as isize) }
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

