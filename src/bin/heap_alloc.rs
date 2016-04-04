use alloc_no_stdlib;
use core::ops;
use core;

pub struct Rebox<T> {
   b : Box<[T]>,
}

impl<T> ops::Index<usize> for Rebox<T>{
    type Output = T;
    fn index(&self, index : usize) -> &T {
        return &(*self.b)[index]
    }
}

impl<T> ops::IndexMut<usize> for Rebox<T>{
    fn index_mut(&mut self, index : usize) -> &mut T {
        return &mut (*self.b)[index]
    }
}

impl<T> alloc_no_stdlib::SliceWrapper<T> for Rebox<T> {
    fn slice(&self) -> & [T] {
       return &*self.b
    }
}

impl<T> alloc_no_stdlib::SliceWrapperMut<T> for Rebox<T> {
    fn slice_mut(&mut self) -> &mut [T] {
       return &mut*self.b
    }
}

pub struct HeapAllocator<T : core::clone::Clone>{
   pub default_value : T,
}

impl<T : core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
   type AllocatedMemory = Rebox<T>;
   fn alloc_cell(self : &mut HeapAllocator<T>, len : usize) -> Rebox<T> {

       let v : Vec<T> = vec![self.default_value.clone();len];
       let b = v.into_boxed_slice();
       return Rebox::<T>{b : b};
   }
   fn free_cell(self : &mut HeapAllocator<T>, _data : Rebox<T>) {

   }
}
