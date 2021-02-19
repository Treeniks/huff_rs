use std::alloc::{alloc, dealloc, Layout};
pub struct HeapArray<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> HeapArray<T> {
    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, std::mem::size_of::<T>());
            alloc(layout) as *mut T
        };
        Self { ptr, len }
    }
}