use super::Dims;

///
/// Shows the size of the object
///
#[derive(Debug, Clone, Default)]
pub struct Size(Dims<u64, 2>);

impl Size {
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
    pub const fn new(x: u64, y: u64) -> Self {
        Self(Dims::new([x, y]))
    }

    pub fn w(&self) -> u64 {
        self.0[0]
    }

    pub fn h(&self) -> u64 {
        self.0[1]
    }

    ///
    /// Multiplies width and height by w
    ///
    /// # Example
    /// ```
    /// # use chen_core_lib::builtins::Size;
    /// let size = Size::new(4, 5);
    /// assert_eq!(size.flat_mul(3), Size::new(12, 15));
    /// ```
    ///
    pub fn flat_mul(mut self, w: u64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    ///
    /// Returns flattened size
    ///
    pub fn flat(&self) -> u64 {
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
