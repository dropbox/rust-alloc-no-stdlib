extern crate core;
use alloc::Allocator;
use super::HeapAllocator;


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
