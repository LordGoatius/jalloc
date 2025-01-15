use std::{marker::PhantomData, ops::{Deref, DerefMut}, ptr::{drop_in_place, NonNull}};

use super::Jallocator;

pub struct Box<T> {
    data: NonNull<T>,
    _phd: PhantomData<T>
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.data.as_ptr());
        }
    }
}

impl<T> Box<T> {
    pub fn new(j: Jallocator, t: T) -> Self {
        Self { data: NonNull::new(j.jalloc(t)).unwrap(), _phd: PhantomData }
    }
}

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.data.as_ref()
        }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.data.as_mut()
        }
    }
}
