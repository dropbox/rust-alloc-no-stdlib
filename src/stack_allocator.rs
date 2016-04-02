extern crate core;
use allocated_memory;
use allocated_memory::AllocatedStackMemory;

pub trait Allocator<T> {
    type AllocatedMemory : allocated_memory::traits::AllocatedSlice<T>;
    fn alloc_cell(&mut self, len : usize) -> Self::AllocatedMemory;
    fn free_cell(&mut self, data : Self::AllocatedMemory);
}


pub trait SystemMemoryPool<'a, T> {
    fn add_slices(&mut self, input : &mut Option<& 'a mut AllocatedStackMemory<'a, T> >);
    fn add_memory(&mut self, input : &mut Option<& 'a mut AllocatedStackMemory<'a, T> >, min_length : usize);
}




pub struct StackAllocator<'a, T :'a, U : SystemMemoryPool<'a, T> > {
    pub system_resources : U,
    pub glob : Option<&'a mut AllocatedStackMemory <'a, T> >,
}


fn remove_cur<'a, T>(mut iter : &mut Option<&mut AllocatedStackMemory<'a, T> >) -> &'a mut AllocatedStackMemory<'a, T> {
    match *iter {
        Some(ref mut glob_next) => {
            let rest : Option<&'a mut AllocatedStackMemory<'a, T> >;
            match glob_next.next {
                Some(ref mut root_cell) => rest = core::mem::replace(&mut root_cell.next, None),
                None => rest = None,
            }
            match core::mem::replace(&mut glob_next.next, rest) {
                Some(mut root_cell) => return root_cell,
                None => panic!("Empty list"),
            }
        },
        None => panic!("List not initialized"),
    }
}


fn remove_search_recursive<'a, T : 'a> (mut node : &mut Option<&mut AllocatedStackMemory <'a, T> >,
                                        searchmin : usize,
                                        searchmax : usize) -> &'a mut AllocatedStackMemory<'a, T> {
    let mut found : bool = false;
    {
        match *node {
            Some(ref mut cur_cell) => {
                match cur_cell.next {
                    Some(ref item) => {
                        let len = item.mem.len();
                        if (len >= searchmin && (searchmax ==0 || len < searchmax))
                            || len == 0 || !item.next.is_some() {
                               found = true;
                        }
                    },
                    None => panic!("Too many outstanding allocated items"),
                 }
                 if !found {
                    return remove_search_recursive(&mut cur_cell.next, searchmin, searchmax);
                 }
            },
            None => panic!("Too many outstanding allocated items"),
        }
    }
    assert!(found);
    return remove_cur(node);
}

fn return_slice_to<'a, T> (mut node : &mut Option<&mut AllocatedStackMemory <'a, T> >,
                 mut val : & 'a mut AllocatedStackMemory<'a, T>) {
        match *node {
            Some(ref mut glob_next) => {
                match core::mem::replace(&mut glob_next.next ,None) {
                    Some(mut x) => {
                        let _discard = core::mem::replace(&mut val.next, Some(x));
                    },
                    None => {},
                }
                glob_next.next = Some(val);
             },
             None => panic!("Allocator not initialized"),
        }
}

impl<'a, T, U : SystemMemoryPool<'a, T> > StackAllocator <'a, T, U> {
    fn alloc_cell_directive(self : &mut StackAllocator<'a, T, U>, len : usize,
                            best_match: bool) -> &'a mut AllocatedStackMemory<'a, T> {
        let lower_bound : usize;
        let  upper_bound: usize;
        if best_match {
            lower_bound = len;
            upper_bound = len * 3 / 2;
        } else {
            lower_bound = 0;
            upper_bound = 0;
        }

        let mut retval = remove_search_recursive(&mut self.glob, lower_bound, upper_bound);
        if retval.mem.len() < len {
            let oom : bool;
            match self.glob {
                Some(ref mut sentinel) => {
                    oom = sentinel.mem.len() < len;
                },
                None => panic!("Uninitialized allocator"),
            }
            if oom {
                self.system_resources.add_memory(&mut self.glob, len);
            }
            match self.glob {
                Some(ref mut sentinel) => {
                    let mut current_mem = core::mem::replace(&mut sentinel.mem, &mut[]);
                    if current_mem.len() < len {
                        panic!("OOM");
                    }
                    let (mut new_chunk, mut remaining_mem) = current_mem.split_at_mut(len);
                    sentinel.mem = remaining_mem;
                    retval.mem = new_chunk;
                },
                None => panic!("Uninitalized allocator"),
            }
        }
        return retval;
    }
    fn no_slices_left(self :&StackAllocator<'a, T, U>) -> bool {
        match self.glob {
            Some(ref glob_next) => return !glob_next.next.is_some(),
            None => return true,
        }
    }
}
impl<'a, T, U : SystemMemoryPool<'a, T> >
    Allocator<T> for StackAllocator <'a, T, U> {
    type AllocatedMemory = &'a mut AllocatedStackMemory<'a, T>;
    fn alloc_cell(self : &mut StackAllocator<'a, T, U>,
                            len : usize) -> &'a mut AllocatedStackMemory<'a, T> {
        if self.no_slices_left() {
            self.system_resources.add_slices(&mut self.glob);
        }
        return self.alloc_cell_directive(len, true);
    }
    fn free_cell(self : &mut StackAllocator<'a, T, U>,
                 mut val : & 'a mut AllocatedStackMemory<'a, T>) {
        return_slice_to(&mut self.glob, val)
    }
}

pub struct AllocatorStackState<'a, T : 'a>{
  pub _cells : &'a mut [AllocatedStackMemory<'a, T>],
}

impl<'a, T> SystemMemoryPool<'a, T> for AllocatorStackState<'a, T> {
    fn add_slices(self : &mut AllocatorStackState<'a, T>, _x : & mut Option<&'a mut AllocatedStackMemory<'a, T> >) {
        panic!("Out of Slices");
    }
    fn add_memory(self : &mut AllocatorStackState<'a, T>, _x : & mut Option<&'a mut AllocatedStackMemory<'a, T> >, _min_len : usize) {
        panic!("Out of Memory");
    }
}