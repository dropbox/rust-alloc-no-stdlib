#![no_std]

mod allocated_memory;
mod stack_allocator;
pub mod init;
mod tests;
pub use allocated_memory::traits::SliceWrapper;
pub use allocated_memory::traits::SliceWrapperMut;
pub use allocated_memory::traits::AllocatedSlice;

pub use allocated_memory::AllocatedStackMemory;
pub use stack_allocator::Allocator;
pub use stack_allocator::StackAllocator;
pub use stack_allocator::AllocatorStackState;

