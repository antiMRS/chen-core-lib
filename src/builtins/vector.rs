use super::Dims;
use super::Position;

///
/// Two dimensional vector
///
#[derive(Debug, Clone, Default, Copy)]
pub struct Vector(Dims<i64, 2>);

impl Vector {
    pub const fn new(w: i64, h: i64) -> Self {
        Self(Dims::new([w, h]))
    }

    ///
    /// Anolog for `Vector::new(0,0)`
    ///
    pub const fn zero() -> Self {
        Self(Dims::new([0, 0]))
    }

    pub fn i(&self) -> i64 {
        self.0[0]
    }

    pub fn j(&self) -> i64 {
        self.0[1]
    }

    ///
    /// Returns vector lenght in cube
    ///
    pub fn plenght(&self) -> i64 {
        self.i().pow(2) + self.j().pow(2)
    }

    ///
    /// Returns vector lenght
    ///
    /// Analog for `Vector::plenght(vec).isqrt()`
    ///
    pub fn lenght(&self) -> i64 {
        self.plenght().isqrt()
    }

    ///
    /// Multiplicates vector `i` and `j` for `w`
    ///
    pub fn flat_mul(mut self, w: i64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    ///
    /// Returns dot product of two vectors
    ///
    pub fn dot_product(&self, other: Vector) -> i64 {
        self.i() * other.i() + self.j() * other.j()
    }

    ///
    /// Returns vector between two points
    ///
    pub fn between(from: &Position, to: &Position) -> Self {
        Self::new(to.x() - from.x(), to.y() - from.y())
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.i() + rhs.i(), self.j() + rhs.j())
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.i(), -self.j())
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
