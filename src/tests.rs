#[cfg(test)]

use super::{Allocator, SliceWrapperMut,
            StackAllocator, AllocatedStackMemory, AllocatorStackState};
extern crate core;
#[test]
fn integration_test() {
  let mut global_buffer : [u8; 65536] = [0; 65536];
  let mut cells_backing_store : [AllocatedStackMemory<u8>; 8]
      = [AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None},
         AllocatedStackMemory::<u8>{mem:&mut[], next:None}];
  let mut ags = StackAllocator::<u8, AllocatorStackState<u8> > {
      system_resources :  AllocatorStackState::<u8> {
          _cells : &mut [],
      },
      glob : None,
  };
  let mut cells = &mut cells_backing_store[..];
  let (mut sentinel, mut normal_cells) = cells.split_at_mut(1);
  sentinel[0].mem = &mut global_buffer;
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
  assert_eq!(x[0], 4);
  assert_eq!(z[0], 5);
  assert_eq!(z[1], 8);
  assert_eq!(reget_three[0], 0);
  assert_eq!(reget_three[1], 9);
  let mut _z = ags.alloc_cell(1);
  }

}


