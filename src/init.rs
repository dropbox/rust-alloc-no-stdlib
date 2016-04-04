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
    (@accum (1024, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (256, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (2048, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (1024, $($expr,)* $($expr),*) -> ($($body)*))};
    (@accum (4096, $($expr:expr),*) -> ($($body:tt)*))
        => {static_array!(@accum (2048, $($expr,)* $($expr),*) -> ($($body)*))};

    (@as_expr $expr:expr) => {$expr};

    [$expr:expr; $n:tt] => { static_array!(@accum ($n, $expr) -> ()) };
}


#[macro_export]
macro_rules! define_stack_allocator_traits(
    ($name : ident, global) => {
        define_stack_allocator_traits!($name, calloc);
    };
    ($name : ident, stack) => {
        define_stack_allocator_traits!($name, calloc);
    };
    ($name : ident, heap) => {
        define_stack_allocator_traits!($name, calloc);
    };
    ($name : ident, calloc) => {
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
    ($name :ident, $freelist_size : expr, $heap_size : expr, calloc) => {
        struct $name<'a, T : 'a> {freelist : [&'a mut [T]; $freelist_size]}
        define_stack_allocator_traits!($name, calloc);
    };
    ($name :ident, $freelist_size : expr, $heap_size : expr, heap) => {
        struct $name<'a, T : 'a> {freelist : Box<[&'a mut [T]]>, heap : Box<[T]>}
        define_stack_allocator_traits!($name, heap);
    };
    ($name :ident, $freelist_size : expr, $heap_size : expr, stack) => {
        struct $name<'a, T : 'a> {
            freelist : [&'a mut [T];$freelist_size],
            // can't borrow here: make it on stack-- heap : core::cell::RefCell<[T; $heap_size]>
        }
        define_stack_allocator_traits!($name, stack);
    };
    ($name :ident, $freelist_size : expr, $heap_size : expr, global) => {
       struct $name <'a, T: 'a> {freelist : [&'a mut [T]]}
        define_stack_allocator_traits!($name, global);
    };
);
