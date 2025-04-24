use std::{alloc::Layout, cell::Cell, fmt::Debug, ptr::NonNull};
pub mod r#box;

#[expect(dead_code)]
#[repr(align(4096))]
struct PageLayout([u8; 4096]);

pub struct Jallocator {
    //memory: Vec<AllocBlock>,
    mem: Cell<NonNull<u8>>,
    pub size: Cell<usize>,
    pub capacity: Cell<usize>,
}

impl<'a> Jallocator {
    pub fn new() -> Self {
        let mem = unsafe { std::alloc::alloc_zeroed(Layout::new::<PageLayout>()) };
        Self {
            mem: Cell::new(NonNull::new(mem).unwrap()),
            size: Cell::new(0),
            capacity: Cell::new(4096),
        }
    }

    pub fn jalloc<T>(&'a self, t: T) -> &'a mut T {
        let layout = Layout::new::<T>();
        if layout.size() > (self.capacity.get() - self.size.get()) {
            panic!("OOM");
        }

        let addr = unsafe { (*self.mem.as_ptr()).as_ptr() };

        let offset_align = layout.align() - (addr as usize % layout.align());

        let ptr = unsafe { addr.add(self.size.get() + offset_align) } as *mut T;
        unsafe {
            ptr.write(t);
            self.size.set(self.size.get() + layout.size() + offset_align);
            &mut *ptr
        }
    }
}

impl Drop for Jallocator {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc((*self.mem.as_ptr()).as_ptr(), Layout::new::<PageLayout>());
        }
    }
}

impl Debug for Jallocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = unsafe {
            ((*self.mem.as_ptr()).as_ptr() as *mut [u8; 4096]).read()
        };
        write!(f, "{bytes:x?}")
    }
}
