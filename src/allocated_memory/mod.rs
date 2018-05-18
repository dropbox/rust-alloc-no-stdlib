extern crate core;
#[macro_use]
mod index_macro;
mod test;
use core::default::Default;
pub use core::ops::IndexMut;
pub use core::ops::Index;
pub use core::ops::Range;
use core::ops;
pub trait SliceWrapper<T> {
    fn slice(& self) -> & [T];
    fn len(&self) -> usize{
        self.slice().len()
    }
}

pub trait SliceWrapperMut<T> : SliceWrapper<T> {
  fn slice_mut (&mut self) -> & mut [T];
}

pub trait AllocatedBasicSlice<T>
    : SliceWrapperMut<T> + SliceWrapper<T> + Default {
}


pub trait AllocatedSlice<T>
    : SliceWrapperMut<T> + SliceWrapper<T> + Default + ops::IndexMut<usize> + ops::Index<usize> + ops::IndexMut<ops::Range<usize>> + ops::Index<ops::Range<usize>> + ops::Deref + ops::DerefMut {
}

impl<T, U> AllocatedSlice<T> for U where U : SliceWrapperMut<T> + SliceWrapper<T> + Default + ops::IndexMut<usize> + ops::Index<usize> + ops::IndexMut<ops::Range<usize>> + ops::Index<ops::Range<usize>> + ops::Deref + ops::DerefMut {

}
