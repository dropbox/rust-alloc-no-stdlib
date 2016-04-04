//#![feature(trace_macros)]
#[macro_use]
extern crate alloc_no_stdlib;
extern crate core;
use core::ops;
mod heap_alloc;

pub use heap_alloc::HeapAllocator;

mod tests;


//use alloc::AllocatedSlice;
use alloc_no_stdlib::SliceWrapper;
use alloc_no_stdlib::SliceWrapperMut;
use alloc_no_stdlib::AllocatedStackMemory;
use alloc_no_stdlib::Allocator;
use alloc_no_stdlib::StackAllocator;
declare_stack_allocator_struct!(CallocAllocatedFreelist4, 4, calloc);
declare_stack_allocator_struct!(StackAllocatedFreelist16, 16, stack);
declare_stack_allocator_struct!(BoxAllocatedFreelist, heap);

extern {
pub fn calloc(nobj: usize, size: usize) -> *mut u8;
}
fn main() {
  //let mut global_buffer : [u8; 1024 * 4096] = [0;4096*1024];
  //let max_memory_pool_size : usize = 1024 * 1024 * 200;

  //// let global_buffer_vec : Vec<u8> = vec![0; max_memory_pool_size];
  //// //let global_buffer_vec = std::iter::repeat(0u8).take(max_memory_pool_size).collect::<Vec<u8>>();
  //// let mut global_buffer_box = global_buffer_vec.into_boxed_slice();

  //// let mut global_buffer = &mut *global_buffer_box;
  //let allocated_mem = unsafe {calloc(max_memory_pool_size, core::mem::size_of::<u8>())};
  //let global_ptr : *mut u8 = unsafe {core::mem::transmute(allocated_mem)};
  //let mut global_buffer = unsafe {core::slice::from_raw_parts_mut(global_ptr, max_memory_pool_size)};
//trace_macros!(true);
  define_heap_memory_structure!(global_buffer, 4, u8, [0; 1024 * 1024 * 200], calloc);

  let mut ags = CallocAllocatedFreelist4::<u8>::new_allocator(global_buffer);

  {
  let mut x = ags.alloc_cell(9999);
  x.slice_mut()[0] = 4;
  let mut y = ags.alloc_cell(4);
  y[0] = 5;
  ags.free_cell(y);

  let mut three = ags.alloc_cell(3);
  three[0] = 6;
  ags.free_cell(three);

  let mut z = ags.alloc_cell(4);
  z.slice_mut()[1] = 8;
  let mut reget_three = ags.alloc_cell(4);
  reget_three.slice_mut()[1] = 9;
  //y.mem[0] = 6; // <-- this is an error (use after free)
  println!("x[0] = {:?} z[0] = {:?}  z[1] = {:?} r3[0] = {:?} r3[1] = {:?}", x.mem[0], z.mem[0], z.mem[1], reget_three[0], reget_three.slice()[1]);
  let mut _z = ags.alloc_cell(1);
  }
  define_heap_memory_structure!(zero_global_buffer, 4, u8, [0; 1024 * 1024 * 20], heap);
  let mut boxallocator = BoxAllocatedFreelist::<u8>::new_allocator(1024 * 1024);
  bind_memory_buffer_to_allocator!(boxallocator, zero_global_buffer, u8, heap);
  {
    let mut x = boxallocator.alloc_cell(9999);
    x.slice_mut()[0] = 3;
    let mut y = boxallocator.alloc_cell(4);
    y[0] = 5;
    boxallocator.free_cell(y);

    let mut three = boxallocator.alloc_cell(3);
    three[0] = 6;
    boxallocator.free_cell(three);

    let mut z = boxallocator.alloc_cell(4);
    z.slice_mut()[1] = 8;
    let mut reget_three = boxallocator.alloc_cell(4);
    reget_three.slice_mut()[1] = 9;
    //y.mem[0] = 6; // <-- this is an error (use after free)
    println!("x[0] = {:?} z[0] = {:?}  z[1] = {:?} r3[0] = {:?} r3[1] = {:?}", x.mem[0], z.mem[0], z.mem[1], reget_three[0], reget_three.slice()[1]);
    let mut _z = boxallocator.alloc_cell(1);
  }

  define_heap_memory_structure!(stack_global_buffer, 16, u8, [0; 1024 * 1024 * 20], stack);
  let mut stackallocator = StackAllocatedFreelist16::<u8>::new_allocator(&mut stack_global_buffer);
  {
    let mut x = stackallocator.alloc_cell(9999);
    x.slice_mut()[0] = 3;
    let mut y = stackallocator.alloc_cell(4);
    y[0] = 5;
    stackallocator.free_cell(y);

    let mut three = stackallocator.alloc_cell(3);
    three[0] = 6;
    stackallocator.free_cell(three);

    let mut z = stackallocator.alloc_cell(4);
    z.slice_mut()[1] = 8;
    let mut reget_three = stackallocator.alloc_cell(4);
    reget_three.slice_mut()[1] = 9;
    //y.mem[0] = 6; // <-- this is an error (use after free)
    println!("x[0] = {:?} z[0] = {:?}  z[1] = {:?} r3[0] = {:?} r3[1] = {:?}", x.mem[0], z.mem[0], z.mem[1], reget_three[0], reget_three.slice()[1]);
    let mut _z = stackallocator.alloc_cell(1);
  }



  let mut halloc : HeapAllocator<u8> = HeapAllocator::<u8>{default_value: 0};
  for _i in 1..10 { // heap test
      let mut x = halloc.alloc_cell(100000);
      x[0] = 4;
      let mut y = halloc.alloc_cell(110000);
      y[0] = 5;
      let mut z = halloc.alloc_cell(120000);
      z[0] = 6;
      halloc.free_cell(y);
      println!("x[0] {:?} x[9] {:?} y[0] {:?} z[0] {:?}",
               x[0], x[9], -999, z[0]);
  }
}