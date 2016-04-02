extern crate core;
pub mod traits;
use core::ops;


pub struct AllocatedStackMemory<'a, T:'a> {
    pub mem : &'a mut [T],
    pub next : Option<&'a mut AllocatedStackMemory <'a, T> >,
}

impl<'a, T: 'a> ops::Index<usize> for & 'a mut AllocatedStackMemory<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, _index : usize) -> &'b T {
        return &self.mem[_index];
    }
}

impl<'a, T: 'a> ops::IndexMut<usize> for &'a mut AllocatedStackMemory<'a, T> {
    fn index_mut<'b>(&'b mut self, _index : usize) -> &'b mut T {
        return &mut self.mem[_index];
    }
}

impl<'a, T: 'a> traits::SliceWrapper<T> for & 'a mut AllocatedStackMemory<'a, T> {
    fn slice(& self) -> & [T] {
        return & self.mem;
    }
}

impl<'a, T: 'a> traits::SliceWrapperMut<T> for &'a mut AllocatedStackMemory<'a, T> {
    fn slice_mut(& mut self) ->& mut [T] {
        return &mut self.mem;
    }
}



