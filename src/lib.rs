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
