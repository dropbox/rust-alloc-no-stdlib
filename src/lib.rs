#![no_std]

//#[macro_use]
//extern crate std;
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


pub struct CallocBackingStore<'a, T : 'a> {
    pub raw_data : *mut T,
    pub data : &'a mut[T],
}
extern {
  fn calloc(n_elem : usize, el_size : usize) -> *mut u8;
}
extern {
  fn free(ptr : *mut u8);
}
impl<'a, T : 'a> CallocBackingStore<'a, T> {
  pub fn new(num_elements : usize) -> Self{
     unsafe {
        let retval = calloc(num_elements, core::mem::size_of::<T>());
        let mut raw_data : *mut T = core::mem::transmute(retval);
        return CallocBackingStore::<'a, T>{
           raw_data : raw_data,
           data : unsafe{core::slice::from_raw_parts_mut(raw_data,
                                                         num_elements)},
        };
     }
  }
}
impl<'a, T:'a> Drop for CallocBackingStore<'a, T> {
  fn drop(self :&mut Self) {
    unsafe {
      let to_be_freed : *mut u8 = core::mem::transmute(self.raw_data);
      free(to_be_freed);
    }
  }
}