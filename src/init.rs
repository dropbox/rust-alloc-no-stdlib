#[macro_export]
macro_rules! static_array {
    (@accum (0, $($_ignored:expr),*) -> ($($body:tt)*))
        => {static_array!(@as_expr [$($body)*])};
    (@accum (1, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (0, $($expr),*) -> ($($body)* $($expr,)*))};
    (@accum (2, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (0, $($expr),*) -> ($($body)* $($expr,)* $($expr,)*))};
    (@accum (4, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (2, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (8, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (4, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (16, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (8, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (32, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (16, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (64, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (32, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (128, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (64, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (256, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (128, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (512, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (256, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (1024, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (512, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (2048, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (1024, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (4096, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (2048, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (8192, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (4096, $($expr,)* $($expr),*) -> ($($body)*))};

    (@as_expr $expr:expr) => {$expr};

    [$expr:expr; $n:tt] => { static_array!(@accum ($n, $expr) -> ()) };
}


#[macro_export]
macro_rules! define_stack_allocator_traits(
    ($name : ident, global) => {
        impl<'a, T: 'a> Default for $name<'a, T> {
            fn default() -> Self {
                return $name::<'a, T>{freelist : &mut[],};
            }
        }
        define_stack_allocator_traits!($name, generic);
    };
    ($name : ident, $freelist_size : tt, stack) => {
        impl<'a, T: 'a> Default for $name<'a, T> {
            fn default() -> Self {
                return $name::<'a, T>{freelist : static_array!(&mut[]; $freelist_size)};
            }
        }
        define_stack_allocator_traits!($name, generic);
    };
    ($name : ident, heap) => {
        impl<'a, T: 'a> Default for $name<'a, T> {
            fn default() -> Self {
                let v : Vec<&mut [T]> = Vec::new();
                let b = v.into_boxed_slice();
                return $name::<'a, T>{freelist : b};
            }
        }
        define_stack_allocator_traits!($name, generic);
    };
    ($name : ident, $freelist_size : tt, calloc) => {
        impl<'a, T: 'a> Default for $name<'a, T> {
            fn default() -> Self {
                return $name::<'a, T>{freelist : static_array!(&mut[]; $freelist_size)};
            }
        }
        define_stack_allocator_traits!($name, generic);
    };
    ($name : ident, generic) => {
        impl<'a, T: 'a> SliceWrapper<&'a mut[T]> for $name<'a, T> {
            fn slice(& self) -> & [&'a mut[T]] {
                return & self.freelist;
            }
        }
        impl<'a, T: 'a> SliceWrapperMut<&'a mut [T]> for $name<'a, T> {
            fn slice_mut(& mut self) ->&mut [&'a mut [T]] {
                return &mut self.freelist;
            }
        }
        impl<'a, T: 'a> ops::Index<usize> for $name<'a, T> {
            type Output = [T];
            fn index<'b> (&'b self, _index : usize) -> &'b [T] {
                return &self.freelist[_index];
            }
        }

        impl<'a, T: 'a> ops::IndexMut<usize> for $name<'a, T> {
            fn index_mut<'b>(&'b mut self, _index : usize) -> &'b mut [T] {
                return &mut self.freelist[_index];
            }
        }
    };
);

#[macro_export]
macro_rules! declare_stack_allocator_struct(
    (@as_expr $expr : expr) => {$expr};
    (@new_method $name : ident, $freelist_size : tt) => {
        impl<'a, T: 'a> $name<'a, T> {
          fn new_allocator(global_buffer : &'a mut [T]) -> StackAllocator<'a, T, $name<'a, T> > {
              let mut retval = StackAllocator::<T, $name<T> > {
                  nop : &mut [],
                  system_resources : $name::<T>::default(),
                  free_list_start : declare_stack_allocator_struct!(@as_expr $freelist_size),
                  free_list_overflow_count : 0,
              };
              retval.free_cell(AllocatedStackMemory::<T>{mem:global_buffer});
              return retval;
          }
        }
    };
    ($name :ident, $freelist_size : tt, calloc) => {
        struct $name<'a, T : 'a> {
            freelist : [&'a mut [T]; declare_stack_allocator_struct!(@as_expr $freelist_size)],
        }
        define_stack_allocator_traits!($name,
                                       $freelist_size,
                                       calloc);
        declare_stack_allocator_struct!( @new_method $name, $freelist_size);
    };
    ($name :ident, heap) => {
        struct $name<'a, T : 'a> {freelist : Box<[&'a mut [T]]>,}
        define_stack_allocator_traits!($name, heap);
        impl<'a, T: 'a> $name<'a, T> {
          fn make_freelist(freelist_size : usize) -> Box<[&'a mut[T]]> {
              let mut retval = Vec::<&'a mut[T]>::with_capacity(freelist_size);
              for _i in 0..freelist_size {
                  retval.push(&mut[]);
              }
              return retval.into_boxed_slice();
          }
          fn new_allocator(freelist_size : usize,
                           memory_pool : &'a mut Box<[T]>) -> StackAllocator<'a, T, $name<'a, T> > {
              let mut retval = StackAllocator::<T, $name<T> > {
                  nop : &mut [],
                  system_resources : $name::<T> {
                      freelist : Self::make_freelist(freelist_size),
                  },
                  free_list_start : freelist_size,
                  free_list_overflow_count : 0
              };
              retval.free_cell(AllocatedStackMemory::<T>{mem:&mut*memory_pool});
              return retval;
          }
        }
    };
    ($name :ident, $freelist_size : tt, stack) => {
        struct $name<'a, T : 'a> {
            freelist : [&'a mut [T];declare_stack_allocator_struct!(@as_expr $freelist_size)],
            // can't borrow here: make it on stack-- heap : core::cell::RefCell<[T; $heap_size]>
        }
        define_stack_allocator_traits!($name,
                                       $freelist_size,
                                       stack);
        declare_stack_allocator_struct!( @new_method $name, $freelist_size);
    };
    ($name :ident, $freelist_size : expr, global) => {
       struct $name <'a, T: 'a> {freelist : &'a mut [&'a mut [T]]}
       define_stack_allocator_traits!($name, global);
       impl<'a, T: 'a> $name<'a, T> {
          fn new_allocator() -> StackAllocator<'a, T, $name<'a, T> > {
              return StackAllocator::<T, $name<T> > {
                  nop : &mut [],
                  system_resources : $name::<T>::default(),
                  free_list_start : 0,
                  free_list_overflow_count : 0
              };
          }
       }
    };
);
#[macro_export]
macro_rules! bind_global_buffers_to_allocator(
    ($allocator : expr, $buffer : ident, $T : ty) => {
        $allocator.free_list_start = unsafe{$buffer::freelist.len()};
        $allocator.system_resources.freelist = unsafe{&mut $buffer::freelist};
        $allocator.free_cell(AllocatedStackMemory::<$T>{mem:unsafe{&mut $buffer::heap}});
    };
);

#[macro_export]
macro_rules! define_allocator_memory_pool(
    (@as_expr $expr:expr) => {$expr};


    ($name : ident, $freelist_size : tt, $T : ty, [0; $heap_size : expr], calloc) => {
       unsafe fn $name<T : Sized>(num_elements : usize) -> *mut T {
           let retval = calloc(num_elements, core::mem::size_of::<T>());
           return core::mem::transmute(retval);
       }

       let mut $name : &mut [$T] = unsafe{core::slice::from_raw_parts_mut(
           $name::<$T>($heap_size), $heap_size)};
    };
    ($name : ident, $freelist_size : tt, $T : ty, [$default_value : expr; $heap_size : expr], heap) => {
       let mut $name : Box<[$T]> = (vec![$default_value; $heap_size]).into_boxed_slice();
    };
    ($name : ident, $freelist_size : tt, $T : ty, [$default_value : expr; $heap_size : expr], stack) => {
       let mut $name : [$T; $heap_size] = [$default_value; $heap_size];
    };
    ($name : ident, $freelist_size : tt, $T : ty, [$default_value : expr; $heap_size : expr], global) => {
       pub mod $name {
           pub static mut freelist : [&'static mut [$T];
                                  define_allocator_memory_pool!(@as_expr $freelist_size)]
               = static_array!(&mut[]; $freelist_size);
           pub static mut heap : [$T; $heap_size] = [$default_value; $heap_size];
       }
    };

);


/*
#[macro_export]
macro_rules! initialize_allocator(
    (@as_expr $expr:expr) => {$expr};


    ($name : ident, $freelist_size : tt, $T : ty, calloc) => {
        StackAllocator::<$T, $name<$T> > {
            nop : &mut [],
            system_resources : $name::<$T> {
                freelist : static_array!(&mut[]; $freelist_size),
            },
            free_list_start : $freelist_size,
            free_list_overflow_count : 0,
        }
    };
);
*/