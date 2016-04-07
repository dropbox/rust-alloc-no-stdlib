extern crate core;
use core::ops;
use core::default::Default;

pub trait SliceWrapper<T> {
  fn slice(& self) -> & [T];
}

pub trait SliceWrapperMut<T> {
  fn slice_mut (&mut self) -> & mut [T];
}

pub trait AllocatedSlice<T>
    : SliceWrapperMut<T> + SliceWrapper<T> + ops::IndexMut<usize> + ops::Index<usize> + Default {
}

impl<T, U> AllocatedSlice<T> for U where U : SliceWrapperMut<T> + SliceWrapper<T> + ops::IndexMut<usize> + ops::Index<usize> + Default {

}
