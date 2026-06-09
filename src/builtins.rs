use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub(super) struct Dims<T: Copy, const D: usize> {
    dims: [T; D],
}

impl<T: Copy, const D: usize> From<[T; D]> for Dims<T, D> {
    fn from(value: [T; D]) -> Self {
        Self { dims: value }
    }
}

impl<T: Copy, const D: usize> Clone for Dims<T, D> {
    fn clone(&self) -> Self {
        Self {
            dims: self.dims.clone(),
        }
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

#[derive(Debug, Clone, Default)]
pub struct Position(Dims<u64, 2>);

impl Position {
    pub fn new(x: u64, y: u64) -> Self {
        Self(Dims::from([x, y]))
    }

    pub fn x(&self) -> u64 {
        self.0[0]
    }

    pub fn y(&self) -> u64 {
        self.0[1]
    }

    pub fn flat_mul(mut self, w: u64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    pub fn as_tuple(self) -> (u64, u64) {
        (self.x(), self.y())
    }

    pub fn flat(&self, size: &Size) -> u64 {
        debug_assert!(
            self.x() < size.w() as u64 && self.y() < size.h() as u64,
            "Position out of bounds"
        );
        self.y() * (size.w() as u64) + self.x()
    }

    pub fn from_flat(xy: u64, size: &Size) -> Self {
        let width = size.w() as u64;
        assert!(width > 0, "Width needs to be greater then 0");
        let x = xy.rem_euclid(width);
        let y = xy.div_euclid(width);
        Position::new(x, y)
    }

    pub fn add(&mut self, x: u64, y: u64) {
        self.0[0] += x;
        self.0[1] += y;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("X", &self.x())
            .field("Y", &self.y())
            .finish()
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Vector(Dims<i64, 2>);

impl Vector {
    pub fn new(w: i64, h: i64) -> Self {
        Self(Dims::from([w, h]))
    }

    pub fn i(&self) -> i64 {
        self.0[0]
    }

    pub fn j(&self) -> i64 {
        self.0[1]
    }

    pub fn lenght2(&self) -> i64 {
        self.i().pow(2) + self.j().pow(2)
    }

    pub fn lenght(&self) -> i64 {
        self.lenght2().isqrt()
    }

    pub fn flat_mul(mut self, w: i64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    pub fn dot_product(&self, other: &Vector) -> i64 {
        self.i() * other.i() + self.j() * other.j()
    }

    pub fn between(from: &Position, to: &Position) -> Self {
        Self::new(
            (to.x() as i64) - (from.x() as i64),
            (to.y() as i64) - (from.y() as i64),
        )
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("i", &self.i())
            .field("j", &self.j())
            .finish()
    }
}

impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.i() == other.i() && self.j() == other.j()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Size(Dims<usize, 2>);

impl Size {
    pub fn new(x: usize, y: usize) -> Self {
        Self(Dims::from([x, y]))
    }

    pub fn w(&self) -> usize {
        self.0[0]
    }

    pub fn h(&self) -> usize {
        self.0[1]
    }

    pub fn flat_mul(mut self, w: usize) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    pub fn flat(&self) -> usize {
        self.w() * self.h()
    }
}

impl std::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.w() == other.w() && self.h() == other.h()
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("W", &self.w())
            .field("H", &self.h())
            .finish()
    }
}
