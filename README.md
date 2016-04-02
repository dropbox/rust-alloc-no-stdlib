Framework for allocating memory in #![no_std] modules.

### Requirements
 * Rust 1.6

### Documentation
Currently there is no standard way to allocate memory from within a module that is no_std.
This provides a mechanism to describe a memory allocation that can be satisfied entirely on
the stack, by unsafely linking to calloc, or by unsafely referencing a mutable global variable.
This library currently will leak memory if free_cell isn't specifically invoked on memory.

However, if linked by a library that actually can depend on the stdlib then that library
can simply pass in a few allocators and use the standard Box allocation and will free automatically.

This library should also make it possible to entirely jail a rust application that needs dynamic
allocations by preallocating a maximum limit of data upfront using calloc and
using seccomp to disallow future syscalls.

#### Contributors
- Daniel Reiter Horn
