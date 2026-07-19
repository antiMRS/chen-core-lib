use num_traits::PrimInt;
use num_traits::Signed;

use super::Dims;

///
/// Two dimensional vector
///
#[derive(Debug, Clone, Default, Copy)]
pub struct Vector<T: PrimInt + Signed>(Dims<T, 2>);

impl<T: PrimInt + Signed> Vector<T> {
    pub const fn new(w: T, h: T) -> Self {
        Self(Dims::new([w, h]))
    }

    ///
    /// Anolog for `Vector::new(0,0)`
    ///
    pub fn zero() -> Self {
        Self(Dims::new([T::zero(), T::zero()]))
    }

    pub fn i(&self) -> T {
        self.0[0]
    }

    pub fn j(&self) -> T {
        self.0[1]
    }

    ///
    /// Returns vector lenght in cube
    ///
    pub fn plenght(&self) -> T {
        self.i().pow(2) + self.j().pow(2)
    }

    ///
    /// Returns dot product of two vectors
    ///
    pub fn dot_product(&self, other: Self) -> T {
        self.i() * other.i() + self.j() * other.j()
    }

    ///
    /// Creates new vector between two point
    ///
    pub fn between(
        &self,
        from: super::extend::Position<T>,
        to: super::extend::Position<T>,
    ) -> Self {
        Self::new(to.x() - from.x(), to.y() - from.y())
    }
}

impl<T: PrimInt + Signed> std::ops::Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.i() + rhs.i(), self.j() + rhs.j())
    }
}

impl<T: PrimInt + Signed> std::ops::Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: PrimInt + Signed> std::ops::Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.i(), -self.j())
    }
}

impl<T: PrimInt + Signed + std::fmt::Debug> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("i", &self.i())
            .field("j", &self.j())
            .finish()
    }
}

impl<T: PrimInt + Signed> std::cmp::PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.i() == other.i() && self.j() == other.j()
    }
}

impl<T: PrimInt + Signed> Vector<T> {
    pub fn from<O: PrimInt + Signed + Into<T>>(value: Vector<O>) -> Self {
        Self::new(value.i().into(), value.j().into())
    }
}
