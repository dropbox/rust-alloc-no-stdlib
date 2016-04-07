extern crate core;
use super::allocated_memory::SliceWrapper;
use super::allocated_memory::SliceWrapperMut;
use core::ops;
pub struct AllocatedStackMemory<'a, T:'a> {
    pub mem : &'a mut [T],
}

impl<'a, T: 'a> core::default::Default for AllocatedStackMemory<'a, T> {
    fn default() -> Self {
        return AllocatedStackMemory::<'a, T>{mem : &mut[]};
    }
}

impl<'a, T: 'a> ops::Index<usize> for AllocatedStackMemory<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, _index : usize) -> &'b T {
        return &self.mem[_index];
    }
}

impl<'a, T: 'a> ops::IndexMut<usize> for AllocatedStackMemory<'a, T> {
    fn index_mut<'b>(&'b mut self, _index : usize) -> &'b mut T {
        return &mut self.mem[_index];
    }
}

impl<'a, T: 'a> SliceWrapper<T> for AllocatedStackMemory<'a, T> {
    fn slice(& self) -> & [T] {
        return & self.mem;
    }
}

impl<'a, T: 'a> SliceWrapperMut<T> for AllocatedStackMemory<'a, T> {
    fn slice_mut(& mut self) ->& mut [T] {
        return &mut self.mem;
    }
}



