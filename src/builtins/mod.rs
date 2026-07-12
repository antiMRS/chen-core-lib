mod buffer;
mod geometry;
mod position;
mod size;
mod sprite;
mod udim;
mod vector;

use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub(super) struct Dims<T: Copy, const D: usize> {
    dims: [T; D],
}

impl<T: Copy, const D: usize> Dims<T, D> {
    pub const fn new(v: [T; D]) -> Self {
        Self { dims: v }
    }
}

impl<T: Copy, const D: usize> From<[T; D]> for Dims<T, D> {
    fn from(value: [T; D]) -> Self {
        Self { dims: value }
    }
}

impl<T: Copy, const D: usize> Clone for Dims<T, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy, const D: usize> Copy for Dims<T, D> {}

impl<T: Copy, const D: usize> Index<usize> for Dims<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dims[index]
    }
}

impl<T: Copy, const D: usize> IndexMut<usize> for Dims<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dims[index]
    }
}

impl<T: Copy + Default, const D: usize> Default for Dims<T, D> {
    fn default() -> Self {
        Self {
            dims: [T::default(); D],
        }
    }
}

pub(super) use buffer::Buffer;

// ----------------------------------------------------------
//   Mod Head

pub const EMPTY_CHAR: char = ' ';

pub use self::{
    geometry::Geometry,
    position::Position,
    size::Size,
    sprite::{Color, Sprite},
    udim::UDim,
    vector::Vector,
};
