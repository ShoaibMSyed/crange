#![feature(min_const_generics)]

#[cfg(test)]
mod tests;

use std::ops::{Index, IndexMut};

pub struct Range<const LEN: usize>;

impl<T, const LEN: usize> Index<Range<LEN>> for [T] {
    type Output = [T; LEN];

    fn index(&self, _: Range<LEN>) -> &[T; LEN] {
        let self_len = self.len();
        match self.get_range::<LEN>() {
            Some(array) => array,
            None => panic!("range index out of bounds: the len is {} but the range len is {}", self_len, LEN),
        }
    }
}

impl<T, const LEN: usize> IndexMut<Range<LEN>> for [T] {
    fn index_mut(&mut self, _: Range<LEN>) -> &mut [T; LEN] {
        let self_len = self.len();
        match self.get_range_mut::<LEN>() {
            Some(array) => array,
            None => panic!("range index out of bounds: the len is {} but the range len is {}", self_len, LEN),
        }
    }
}

pub trait RangeGet<T> {
    fn get_range<const LEN: usize>(&self) -> Option<&[T; LEN]> { None }
    fn get_range_mut<const LEN: usize>(&mut self) -> Option<&mut [T; LEN]> { None }
}

impl<T> RangeGet<T> for [T] {
    fn get_range<const LEN: usize>(&self) -> Option<&[T; LEN]> {
        if self.len() < LEN {
            None
        } else {
            Some(unsafe { &*(self as *const [T] as *const [T; LEN]) })
        }
    }

    fn get_range_mut<const LEN: usize>(&mut self) -> Option<&mut [T; LEN]> {
        if self.len() < LEN {
            None
        } else {
            Some(unsafe { &mut *(self as *mut [T] as *mut [T; LEN]) })
        }
    }
}