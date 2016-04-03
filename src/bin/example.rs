#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate core;
use core::ops;
mod heap_alloc;

pub use heap_alloc::HeapAllocator;

mod tests;


//use alloc::AllocatedSlice;
use alloc::SliceWrapper;
use alloc::SliceWrapperMut;
use alloc::AllocatedStackMemory;
use alloc::Allocator;
use alloc::StackAllocator;

struct StackAllocatedFreelist4<'a, T : 'a> {
   freelist : [&'a mut [T]; 4],
}


impl<'a, T: 'a> alloc::SliceWrapper<&'a mut[T]> for StackAllocatedFreelist4<'a, T> {
    fn slice(& self) -> & [&'a mut[T]] {
        return & self.freelist;
    }
}

impl<'a, T: 'a> alloc::SliceWrapperMut<&'a mut [T]> for StackAllocatedFreelist4<'a, T> {
    fn slice_mut(& mut self) ->&mut [&'a mut [T]] {
        return &mut self.freelist;
    }
}

impl<'a, T: 'a> ops::Index<usize> for StackAllocatedFreelist4<'a, T> {
    type Output = [T];
    fn index<'b> (&'b self, _index : usize) -> &'b [T] {
        return &self.freelist[_index];
    }
}

impl<'a, T: 'a> ops::IndexMut<usize> for StackAllocatedFreelist4<'a, T> {
    fn index_mut<'b>(&'b mut self, _index : usize) -> &'b mut [T] {
        return &mut self.freelist[_index];
    }
}


extern {
pub fn calloc(nobj: usize, size: usize) -> *mut u8;
}
fn main() {
  //let mut global_buffer : [u8; 1024 * 4096] = [0;4096*1024];
  let max_memory_pool_size : usize = 1024 * 1024 * 200;

  //// let global_buffer_vec : Vec<u8> = vec![0; max_memory_pool_size];
  //// //let global_buffer_vec = std::iter::repeat(0u8).take(max_memory_pool_size).collect::<Vec<u8>>();
  //// let mut global_buffer_box = global_buffer_vec.into_boxed_slice();

  //// let mut global_buffer = &mut *global_buffer_box;
  let allocated_mem = unsafe {calloc(max_memory_pool_size, core::mem::size_of::<u8>())};
  let global_ptr : *mut u8 = unsafe {core::mem::transmute(allocated_mem)};
  let mut global_buffer = unsafe {core::slice::from_raw_parts_mut(global_ptr, max_memory_pool_size)};
  let mut ags = StackAllocator::<u8, StackAllocatedFreelist4<u8> > {
      nop : &mut [],
      system_resources :  StackAllocatedFreelist4::<u8> {
          freelist : static_array!(&mut[]; 4),
      },
      free_list_start : 4,
      free_list_overflow_count : 0,
  };
  ags.free_cell(AllocatedStackMemory::<u8>{mem:global_buffer});

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