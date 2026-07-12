use super::Dims;
use super::Size;
use super::Vector;
use num_traits::{Euclid, NumCast, PrimInt};

///
/// Stores position of point
///
#[derive(Debug, Clone, Default)]
pub struct Position(Dims<i64, 2>);

impl Position {
    pub fn flattened<T: PrimInt + NumCast>(size: &Size, x: T, y: T) -> T {
        y * (T::from::<u64>(size.w()).unwrap()) + x
    }

    pub fn from_flattened<T: PrimInt + NumCast + Euclid>(size: &Size, xy: T) -> (T, T) {
        let width: T = T::from(size.w()).unwrap();
        assert!(width > T::zero(), "Width needs to be greater then 0");
        let x = xy.rem_euclid(&width);
        let y = xy.div_euclid(&width);
        (x, y)
    }
}

impl Position {
    ///
    /// Creates new position by its coordinates
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Position;
    /// let pos = Position::new(1, 2);
    /// ```
    ///
    pub const fn new(x: i64, y: i64) -> Self {
        Self(Dims::new([x, y]))
    }

    pub fn x(&self) -> i64 {
        self.0[0]
    }

    pub fn y(&self) -> i64 {
        self.0[1]
    }

    ///
    /// Multiplies x and y by w
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Position;
    /// let pos = Position::new(2, 3);
    /// assert_eq!(pos.flat_mul(2), Position::new(4, 6))
    ///
    /// ```
    ///
    pub fn flat_mul(mut self, w: i64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    ///
    /// Returns tuple of (x, y)
    ///
    pub fn as_tuple(self) -> (i64, i64) {
        (self.x(), self.y())
    }

    ///
    /// Returns pos in flat to use in indexing
    ///
    pub fn flat(&self, size: &Size) -> i64 {
        self.y() * (size.w() as i64) + self.x()
    }

    ///
    /// Restores position from flattened
    ///
    pub fn from_flat(xy: i64, size: &Size) -> Self {
        let width = size.w() as i64;
        assert!(width > 0, "Width needs to be greater then 0");
        let x = xy.rem_euclid(width);
        let y = xy.div_euclid(width);
        Position::new(x, y)
    }

    ///
    /// Adds vector to position
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::{Position, Vector};
    /// let mut pos = Position::new(2, 3);
    /// let vec = Vector::new(2, 3);
    /// pos.add(vec);
    /// assert_eq!(pos, Position::new(4, 6))
    ///
    /// ```
    ///
    pub fn add(&mut self, vec: Vector) {
        self.0[0] += vec.i();
        self.0[1] += vec.j();
    }

    ///
    /// Returns distantion between two points and rounds up.
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Position;
    ///
    /// let pos1 = Position::new(2, 1);
    /// let pos2 = Position::new(5, 2);
    ///
    /// assert_eq!(pos1.dist(&pos2), 3);
    /// ```
    pub fn dist(&self, other: &Position) -> u64 {
        ((self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)).isqrt() as u64
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
    pub fn pdist(&self, other: &Position) -> i64 {
        (self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)
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
