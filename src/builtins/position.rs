use std::fmt::Debug;

use super::Dims;
use num_traits::Signed;
use num_traits::Unsigned;
use num_traits::{Euclid, NumCast, PrimInt};

#[derive(Debug, Default, Clone, Copy)]
pub struct Position<T: Signed + PrimInt>(Dims<T, 2>);

impl<N: Signed + PrimInt> Position<N> {
    pub fn flattened<T: PrimInt + NumCast, S: PrimInt + Unsigned>(
        size: &super::extend::Size<S>,
        x: T,
        y: T,
    ) -> T {
        y * (T::from::<S>(size.w()).unwrap()) + x
    }

    pub fn from_flattened<T: PrimInt + NumCast + Euclid, S: PrimInt + Unsigned>(
        size: &super::extend::Size<S>,
        xy: T,
    ) -> (T, T) {
        let width: T = T::from(size.w()).unwrap();
        assert!(width > T::zero(), "Width needs to be greater then 0");
        let x = xy.rem_euclid(&width);
        let y = xy.div_euclid(&width);
        (x, y)
    }
}

impl<T: Signed + PrimInt + Euclid> Position<T> {
    pub fn from_flat<S: PrimInt + Unsigned>(size: &super::extend::Size<S>, xy: T) -> Self {
        let (x, y) = Self::from_flattened(size, xy);
        Self::new(x, y)
    }
}

impl<T: Signed + PrimInt> Position<T> {
    ///
    /// Creates new position by its coordinates
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Position;
    /// let pos = Position::new(1, 2);
    /// ```
    ///
    pub const fn new(x: T, y: T) -> Self {
        Self(Dims::new([x, y]))
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    ///
    /// Returns tuple of (x, y)
    ///
    pub fn as_tuple(self) -> (T, T) {
        (self.x(), self.y())
    }

    ///
    /// Returns squared distantion between two points.
    /// Used in precision operations.
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Position;
    ///
    /// let pos1 = Position::new(2, 1);
    /// let pos2 = Position::new(5, 2);
    ///
    /// assert_eq!(pos1.pdist(&pos2), 10);
    /// ```
    pub fn pdist(&self, other: &Self) -> T {
        (self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)
    }

    pub fn flat<S: PrimInt + Unsigned>(&self, size: &super::extend::Size<S>) -> T {
        Self::flattened(size, self.x(), self.y())
    }

    pub fn add(&mut self, xy: (T, T)) {
        self.0[0] = self.0[0] + xy.0;
        self.0[1] = self.0[0] + xy.1;
    }

    pub fn add_vector(&mut self, vec: super::extend::Vector<T>) {
        self.0[0] = self.0[0] + vec.i();
        self.0[1] = self.0[0] + vec.j();
    }
}

impl<T: PrimInt + Signed + Debug> std::fmt::Display for Position<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("X", &self.x())
            .field("Y", &self.y())
            .finish()
    }
}

impl<T: PrimInt + Signed> std::cmp::PartialEq for Position<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

impl<T: PrimInt + Signed> Position<T> {
    pub fn from<O: PrimInt + Signed + Into<T>>(value: Position<O>) -> Self {
        Self::new(value.x().into(), value.y().into())
    }
}
