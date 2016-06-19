#![allow(unused_imports)]
#[cfg(test)]
extern crate core;
use alloc_no_stdlib::Allocator;
use super::HeapAllocator;
#[cfg(feature="stdlib")]
use alloc_no_stdlib::StdAlloc;
#[cfg(all(feature="unsafe", feature="stdlib"))]
use alloc_no_stdlib::StdAllocUninitialized;
#[test]
fn heap_test() {
  let mut halloc : HeapAllocator<u8> = HeapAllocator::<u8>{default_value: 0};
  for _i in 1..10 { // heap test
      let mut x = halloc.alloc_cell(100000);
      x[0] = 4;
      let mut y = halloc.alloc_cell(110000);
      y[0] = 5;
      let mut z = halloc.alloc_cell(120000);
      z[0] = 6;
      assert_eq!(y[0], 5);
      halloc.free_cell(y);
      assert_eq!(x[0], 4);
      assert_eq!(x[9], 0);
      assert_eq!(z[0], 6);
  }

}


#[cfg(all(feature="unsafe", feature="stdlib"))]
#[test]
fn std_unsafe_heap_test() {
  let mut halloc = unsafe{StdAllocUninitialized::<u8>::new()};
  for _i in 1..10 { // heap test
      let mut x = halloc.alloc_cell(100000);
      x[0] = 4;
      let mut y = halloc.alloc_cell(110000);
      y[0] = 5;
      let mut z = halloc.alloc_cell(120000);
      z[0] = 6;
      assert_eq!(y[0], 5);
      halloc.free_cell(y);
      assert_eq!(x[0], 4);
      assert_eq!(x[9], 0);
      assert_eq!(z[0], 6);
  }

}

#[cfg(feature="stdlib")]
#[test]
fn std_heap_test() {
  let mut halloc = unsafe{StdAllocUninitialized::<u8>::new()};
  for _i in 1..10 { // heap test
      let mut x = halloc.alloc_cell(100000);
      x[0] = 4;
      let mut y = halloc.alloc_cell(110000);
      y[0] = 5;
      let mut z = halloc.alloc_cell(120000);
      z[0] = 6;
      assert_eq!(y[0], 5);
      halloc.free_cell(y);
      assert_eq!(x[0], 4);
      assert_eq!(x[9], 0);
      assert_eq!(z[0], 6);
  }

}
