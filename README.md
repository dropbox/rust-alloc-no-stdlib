# Framework for allocating memory in #![no_std] modules.

[![crates.io](http://meritbadge.herokuapp.com/alloc-no-stdlib)](https://crates.io/crates/alloc-no-stdlib)
[![Build Status](https://travis-ci.org/dropbox/alloc-no-stdlib.svg?branch=master)](https://travis-ci.org/dropbox/alloc-no-stdlib)


## Requirements
 * Rust 1.6

## Documentation
Currently there is no standard way to allocate memory from within a module that is no_std.
This provides a mechanism to describe a memory allocation that can be satisfied entirely on
the stack, by unsafely linking to calloc, or by unsafely referencing a mutable global variable.
This library currently will leak memory if free_cell isn't specifically invoked on memory.

However, if linked by a library that actually can depend on the stdlib then that library
can simply pass in a few allocators and use the standard Box allocation and will free automatically.

This library should also make it possible to entirely jail a rust application that needs dynamic
allocations by preallocating a maximum limit of data upfront using calloc and
using seccomp to disallow future syscalls.

## Usage

There are 3 modes for allocating memory, each with advantages and disadvantages

### On the stack
This is possible without the stdlib at all
However, this eats into the natural ulimit on the stack depth and generally
limits the program to only a few megs of dynamically allocated data

Example:

```

// First define a struct to hold all the array on the stack.
declare_stack_allocator_struct!(StackAllocatedFreelist4, 4, stack);
// since generics cannot be used, the actual struct to hold the memory must be defined with a macro
...

  // in the code where the memory must be used, first the array needs to be readied
  define_allocator_memory_pool!(stack_buffer, 4, u8, [0; 65536], stack);
  // then an allocator needs to be made and pointed to the stack_buffer on the stack
  // the final argument tells the system if free'd data should be zero'd before being
  // reused by a subsequent call to alloc_cell
  let mut ags = StackAllocatedFreelist4::<u8>::new_allocator(&mut stack_buffer, bzero);
  {
    // now we can get memory dynamically
    let mut x = ags.alloc_cell(9999);
    x.slice_mut()[0] = 4;
    // get more memory
    let mut y = ags.alloc_cell(4);
    y[0] = 5;
    // and free it, consuming the buffer
    ags.free_cell(y);

    //y.mem[0] = 6; // <-- this is an error: won't compile (use after free)
    assert_eq!(x[0], 4);
```

### On the heap
This does a single big allocation on the heap, after which no further usage of the stdlib
will happen. This can be useful for a jailed application that wishes to restrict syscalls
at this point

```
declare_stack_allocator_struct!(HeapAllocatedFreelist, heap);
...
  define_allocator_memory_pool!(heap_global_buffer, 4096, u8, [0; 6 * 1024 * 1024], heap);
  let mut ags = HeapAllocatedFreelist::<u8>::new_allocator(4096, &mut heap_global_buffer, bzero);
  {
    let mut x = ags.alloc_cell(9999);
    x.slice_mut()[0] = 4;
    let mut y = ags.alloc_cell(4);
    y[0] = 5;
    ags.free_cell(y);

    //y.mem[0] = 6; // <-- this is an error (use after free)
  }
```

### With calloc
This is the most efficient way to get a zero'd dynamically sized buffer without the stdlib
It does invoke the C calloc function and hence must invoke unsafe code.
In this version, the number of cells are fixed to the parameter specified in the struct definition
(4096 in this example)

```
declare_stack_allocator_struct!(CallocAllocatedFreelist4096, 4096, calloc);
...

  // the buffer is defined with 200 megs of zero'd memory from calloc
  define_allocator_memory_pool!(calloc_global_buffer, 4096, u8, [0; 200 * 1024 * 1024], calloc);
  // and assigned to a new_allocator
  let mut ags = CallocAllocatedFreelist4096::<u8>::new_allocator(calloc_global_buffer, bzero);
  {
    let mut x = ags.alloc_cell(9999);
    x.slice_mut()[0] = 4;
    let mut y = ags.alloc_cell(4);
    y[0] = 5;
    ags.free_cell(y);
    //y.mem[0] = 6; // <-- this is an error (use after free)
  }
```

### With a static, mutable buffer
If a single buffer of data is needed for the entire span of the application
Then the simplest way to do so without a zero operation on
the memory and without using the stdlib is to simply have a global allocated
structure. Accessing mutable static variables requires unsafe code; however,
so this code will invoke an unsafe block.


Make sure to only reference global_buffer in a single place, at a single time in the code
If it is used from two places or at different times, undefined behavior may result,
since multiple allocators may get access to global_buffer.


```
declare_stack_allocator_struct!(GlobalAllocatedFreelist, 16, global);
define_allocator_memory_pool!(global_buffer, 16, u8, [0; 1024 * 1024 * 100], global);

...
  // this references a global buffer
  let mut ags = GlobalAllocatedFreelist::<u8>::new_allocator(bzero);
  bind_global_buffers_to_allocator!(ags, global_buffer, u8);
  {
    let mut x = ags.alloc_cell(9999);
    x.slice_mut()[0] = 4;
    let mut y = ags.alloc_cell(4);
    y[0] = 5;
    ags.free_cell(y);

    //y.mem[0] = 6; // <-- this is an error (use after free)
  }
```


## Contributors
- Daniel Reiter Horn
