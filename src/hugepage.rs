use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::{null_mut, NonNull},
};

use libc::{c_void, MAP_ANONYMOUS, MAP_FAILED, MAP_HUGETLB, MAP_PRIVATE, PROT_READ, PROT_WRITE};

const PAGE_SIZE: usize = 2 << 20;

pub struct HugeTlbAllocator;

unsafe impl Allocator for HugeTlbAllocator {
    fn allocate(&self, layout: Layout) -> Result<std::ptr::NonNull<[u8]>, AllocError> {
        let size = layout.size();

        if size == 0 {
            return Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0));
        }

        let ptr = unsafe {
            let raw_ptr = libc::mmap(
                null_mut(),
                size.next_multiple_of(PAGE_SIZE),
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB,
                -1,
                0,
            );

            if raw_ptr == MAP_FAILED {
                return Err(AllocError);
            }

            let raw_ptr = raw_ptr as *mut u8;

            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;

            NonNull::slice_from_raw_parts(ptr, size)
        };


        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        if layout.size() != 0 {
            libc::munmap(ptr.as_ptr() as *mut c_void, layout.size());
        }
    }
}
