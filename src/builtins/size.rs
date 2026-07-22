use num_traits::{PrimInt, Unsigned};

use super::Dims;

///
/// Shows the size of the object
///
#[derive(Debug, Clone, Copy, Default)]
pub struct Size<T: PrimInt + Unsigned>(Dims<T, 2>);

impl<T: PrimInt + Unsigned> Size<T> {
    ///
    /// Creates new Size object
    ///
    /// # Example
    ///
    /// ```
    /// # use chen_core_lib::builtins::Size;
    /// let size = Size::new(10, 10);
    /// ```
    ///
    pub const fn new(x: T, y: T) -> Self {
        Self(Dims::new([x, y]))
    }

    pub fn w(&self) -> T {
        self.0[0]
    }

    pub fn h(&self) -> T {
        self.0[1]
    }

    pub fn flat(&self) -> T {
        self.0[1] * self.0[0]
    }
}

impl<T: PrimInt + Unsigned> std::cmp::PartialEq for Size<T> {
    fn eq(&self, other: &Self) -> bool {
        self.w() == other.w() && self.h() == other.h()
    }
}

impl<T: PrimInt + Unsigned + std::fmt::Debug> std::fmt::Display for Size<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("W", &self.w())
            .field("H", &self.h())
            .finish()
    }
}

impl<T: PrimInt + Unsigned> Size<T> {
    pub fn from<O: PrimInt + Unsigned + Into<T>>(value: Size<O>) -> Self {
        Self::new(value.w().into(), value.h().into())
    }
}
