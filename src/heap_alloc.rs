#![cfg(not(feature="no-stdlib"))]
use std;


use super::{SliceWrapper, SliceWrapperMut, Allocator};

use core;

use std::boxed::Box;
use std::vec::Vec;
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
define_index_ops_mut!(T, WrapBox<T>);

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

pub struct HeapAlloc<T : core::clone::Clone>{
   pub default_value : T,
}
impl<T : core::clone::Clone> HeapAlloc<T> {
   pub fn new(data : T) -> HeapAlloc<T> {
      return HeapAlloc::<T>{default_value : data};
   }
}

impl<T : core::clone::Clone> super::Allocator<T> for HeapAlloc<T> {
   type AllocatedMemory = WrapBox<T>;
   fn alloc_cell(self : &mut HeapAlloc<T>, len : usize) -> WrapBox<T> {

       let v : std::vec::Vec<T> = vec![self.default_value.clone();len];
       let b = v.into_boxed_slice();
       return WrapBox::<T>{b : b};
   }
   fn free_cell(self : &mut HeapAlloc<T>, _data : WrapBox<T>) {

   }
}

#[cfg(feature="unsafe")]
pub struct HeapAllocUninitialized<T>{
   #[allow(dead_code)]
   default_value : Option<T>,
}

#[cfg(feature="unsafe")]
impl<T> HeapAllocUninitialized<T>{
   pub unsafe fn new() -> HeapAllocUninitialized<T> {
       return HeapAllocUninitialized::<T>{default_value:None};
   }
}

#[cfg(feature="unsafe")]
impl<T> super::Allocator<T> for HeapAllocUninitialized<T> {
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


pub struct HeapPrealloc<'a, T : 'a> {
   freelist : std::boxed::Box<[&'a mut [T]]>,
}
define_stack_allocator_traits!(HeapPrealloc, heap);

impl<'a, T : core::clone::Clone+'a> HeapPrealloc<'a, T> {
    fn make_freelist(freelist_size : usize) -> std::boxed::Box<[&'a mut[T]]> {
        let mut retval = Vec::<&'a mut[T]>::with_capacity(freelist_size);
        for _i in 0..freelist_size {
            retval.push(&mut[]);
        }
        return retval.into_boxed_slice();
    }
    pub fn new_allocator(freelist_size : usize,
                     memory_pool : &'a mut Box<[T]>,
                     initializer : fn(&mut[T])) -> super::StackAllocator<'a, T, HeapPrealloc<'a, T> > {
        let mut retval = super::StackAllocator::<T, HeapPrealloc<T> > {
            nop : &mut [],
            system_resources : HeapPrealloc::<T> {
                freelist : Self::make_freelist(freelist_size),
            },
            free_list_start : freelist_size,
            free_list_overflow_count : 0,
            initialize : initializer,
        };
        retval.free_cell(super::AllocatedStackMemory::<T>{mem:&mut*memory_pool});
        return retval;
    }
    #[cfg(feature="unsafe")]
    pub unsafe fn new_uninitialized_memory_pool(len : usize) -> Box<[T]> {
        let mut v : std::vec::Vec<T> = std::vec::Vec::with_capacity(len);
        v.set_len(len);
        return v.into_boxed_slice();
    }
}

