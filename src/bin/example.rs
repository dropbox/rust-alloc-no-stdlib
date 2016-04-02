#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate core;

mod heap_alloc;

pub use heap_alloc::HeapAllocator;

mod tests;


//use alloc::AllocatedSlice;
use alloc::SliceWrapper;
use alloc::SliceWrapperMut;
use alloc::AllocatedStackMemory;
use alloc::Allocator;
use alloc::StackAllocator;
use alloc::AllocatorStackState;




extern {
pub fn calloc(nobj: usize, size: usize) -> *mut u8;
}
fn main() {
  //let mut global_buffer : [u8; 1024 * 4096] = [0;4096*1024];
  let max_memory_pool_size : usize = 1024 * 1024 * 200;

/*
  let global_buffer_vec : Vec<u8> = vec![0; max_memory_pool_size];
  //let global_buffer_vec = std::iter::repeat(0u8).take(max_memory_pool_size).collect::<Vec<u8>>();
  let mut global_buffer_box = global_buffer_vec.into_boxed_slice();

  let mut global_buffer = &mut *global_buffer_box;
*/
  let allocated_mem = unsafe {calloc(max_memory_pool_size, core::mem::size_of::<u8>())};
  let global_ptr : *mut u8 = unsafe {core::mem::transmute(allocated_mem)};
  let mut global_buffer = unsafe {core::slice::from_raw_parts_mut(global_ptr, max_memory_pool_size)};
  /*
    for item in &*global_buffer {
      assert_eq!(*item, 0u8);
    }
  */
  let mut cells_backing_store = static_array!(AllocatedStackMemory::<u8>{mem:&mut[], next:None}; 4096);
  let mut ags = StackAllocator::<u8, AllocatorStackState<u8> > {
      system_resources :  AllocatorStackState::<u8> {
          _cells : &mut [],
      },
      glob : None,
  };
  let mut cells = &mut cells_backing_store[..];
  let (mut sentinel, mut normal_cells) = cells.split_at_mut(1);
  sentinel[0].mem = global_buffer;
  ags.glob = Some(&mut sentinel[0]);
  for _i in 0..normal_cells.len() {
      let n_cells = core::mem::replace(&mut normal_cells, &mut []);
      let (mut cell0, mut cell1) = n_cells.split_at_mut(1);
      ags.free_cell(&mut cell0[0]);
      normal_cells = cell1;
  }
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