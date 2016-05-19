#![no_std]

#[macro_use]
extern crate std;
mod allocated_memory;
mod stack_allocator;
mod allocated_stack_memory;
pub mod init;

pub use allocated_memory::SliceWrapper;
pub use allocated_memory::SliceWrapperMut;
pub use allocated_memory::AllocatedSlice;

pub use allocated_stack_memory::AllocatedStackMemory;
pub use stack_allocator::Allocator;
pub use stack_allocator::StackAllocator;
use core::default::Default;
pub fn bzero<T : Default> (data : &mut [T]) {
    for iter in data.iter_mut() {
        *iter = T::default();
    }
}

pub fn uninitialized<T> (_data : &mut[T]) {}

#[derive(Debug)]
pub struct CallocBackingStore<'a, T : 'a> {
    pub raw_data : *mut u8,
    pub data : &'a mut[T],
}
extern {
  fn calloc(n_elem : usize, el_size : usize) -> *mut u8;
}
extern {
  fn free(ptr : *mut u8);
}
impl<'a, T : 'a> CallocBackingStore<'a, T> {
  pub fn new(num_elements : usize, should_free : bool) -> Self{
     let retval : *mut u8 = unsafe{calloc(num_elements, core::mem::size_of::<T>())};
     let mut raw_data : *mut T = unsafe{core::mem::transmute(retval)};
     if should_free {
       return CallocBackingStore::<'a, T>{
         raw_data : retval,
         data : unsafe{core::slice::from_raw_parts_mut(raw_data,
                                                           num_elements)},
       };
     } else {
       let mut null_ptr : *const u8 = core::ptr::null();
       return CallocBackingStore::<'a, T>{
         raw_data : unsafe{core::mem::transmute(null_ptr)},//retval,
         data : unsafe{core::slice::from_raw_parts_mut(raw_data,
                                                           num_elements)},
       };
    }
  }
}
impl<'a, T:'a> Drop for CallocBackingStore<'a, T> {
  fn drop(self :&mut Self) {
//      core::mem::forget(core::mem::replace(self.data, &mut[]));
    core::mem::forget(core::mem::replace(&mut self.data, &mut[]));
    if !self.raw_data.is_null() {
      unsafe {
        free(self.raw_data);
      }
    }
  }
}