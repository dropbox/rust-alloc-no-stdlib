#![cfg(feature="stdlib")]
use std;
use core;
use core::ops;
pub struct WrapBox<T> {
   b : std::boxed::Box<[T]>,
}

impl<T> core::default::Default for WrapBox<T> {
    fn default() -> Self {
       let v : std::vec::Vec<T> = std::vec::Vec::new();
       let b = v.into_boxed_slice();
       return WrapBox::<T>{b : b};
    }
}

impl<T> ops::Index<usize> for WrapBox<T>{
    type Output = T;
    fn index(&self, index : usize) -> &T {
        return &(*self.b)[index]
    }
}

impl<T> ops::IndexMut<usize> for WrapBox<T>{
    fn index_mut(&mut self, index : usize) -> &mut T {
        return &mut (*self.b)[index]
    }
}

impl<T> super::SliceWrapper<T> for WrapBox<T> {
    fn slice(&self) -> & [T] {
       return &*self.b
    }
}

impl<T> super::SliceWrapperMut<T> for WrapBox<T> {
    fn slice_mut(&mut self) -> &mut [T] {
       return &mut*self.b
    }
}

pub struct StdAlloc<T : core::clone::Clone>{
   pub default_value : T,
}

impl<T : core::clone::Clone> super::Allocator<T> for StdAlloc<T> {
   type AllocatedMemory = WrapBox<T>;
   fn alloc_cell(self : &mut StdAlloc<T>, len : usize) -> WrapBox<T> {

       let v : std::vec::Vec<T> = vec![self.default_value.clone();len];
       let b = v.into_boxed_slice();
       return WrapBox::<T>{b : b};
   }
   fn free_cell(self : &mut StdAlloc<T>, _data : WrapBox<T>) {

   }
}

#[cfg(feature="unsafe")]
pub struct StdAllocUninitialized<T : core::clone::Clone>{
   #[allow(dead_code)]
   default_value : Option<T>,
}

#[cfg(feature="unsafe")]
impl<T : core::clone::Clone> StdAllocUninitialized<T>{
   pub unsafe fn new() -> StdAllocUninitialized<T> {
       return StdAllocUninitialized::<T>{default_value:None};
   }
}
#[cfg(feature="unsafe")]
impl<T : core::clone::Clone> super::Allocator<T> for StdAllocUninitialized<T> {
   type AllocatedMemory = WrapBox<T>;
   fn alloc_cell(self : &mut Self, len : usize) -> WrapBox<T> {

       let mut v : std::vec::Vec<T> = std::vec::Vec::with_capacity(len);
       unsafe {v.set_len(len)};
       let b = v.into_boxed_slice();
       return WrapBox::<T>{b : b};
   }
   fn free_cell(self : &mut Self, _data : WrapBox<T>) {

   }
}
