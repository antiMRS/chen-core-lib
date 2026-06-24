use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
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
    /// # use RubyCoreLib::builtins::Position;
    /// let pos = Position::new(1, 2);
    /// ```
    ///
    pub fn new(x: i64, y: i64) -> Self {
        Self(Dims::from([x, y]))
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
    /// # use RubyCoreLib::builtins::Position;
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
    /// Adds x, y to pos
    ///
    /// # Example
    /// ```no_run
    ///
    /// let pos = Position::new(2, 3);
    /// assert_eq!(pos.add(2, 3), Position::new(4, 6))
    ///
    /// ```
    ///
    pub fn add(&mut self, x: i64, y: i64) {
        self.0[0] += x;
        self.0[1] += y;
    }

    ///
    /// Returns distantion between two points and rounds up.
    ///
    /// # Example
    /// ```
    /// # use RubyCoreLib::builtins::Position;
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
    /// # use RubyCoreLib::builtins::Position;
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

/*

Coordinate System
  0
 0 +===================
   =
   =
   =
   =
   =
   =
   =

*/

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
    /// # use RubyCoreLib::builtins::Size;
    /// let size = Size::new(10, 10);
    /// ```
    ///
    pub fn new(x: u64, y: u64) -> Self {
        Self(Dims::from([x, y]))
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
    /// # use RubyCoreLib::builtins::Size;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    dots: Box<[Position]>,
}

use std::cmp::max;
use std::cmp::min;

use num_traits::{Euclid, Num, NumCast, PrimInt};

impl Geometry {
    ///
    /// Creates new geometry by points
    ///
    /// # Example
    /// ```
    /// # use RubyCoreLib::builtins::{Geometry, Position};
    /// let geo = Geometry::new(vec![Position::new(0, 0), Position::new(0, 6), Position::new(5, 6), Position::new(5, 0)]);
    /// ```
    /// This creates a geometry in the form of a 5 by 6 rectangle.
    ///
    pub fn new(dots: Vec<Position>) -> Self {
        Self {
            dots: dots.into_boxed_slice(),
        }
    }

    pub fn new_square(a: u64) -> Self {
        let a = a as i64;
        let vertices = vec![
            Position::new(0, 0),
            Position::new(a, 0),
            Position::new(a, a),
            Position::new(0, a),
        ];
        Geometry::new(vertices)
    }

    pub fn anti_negate(&mut self) {
        todo!("Moves all the points so that there are no negative coordinates")
    }

    pub fn len(&self) -> usize {
        self.dots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    pub fn size(&self) -> Size {
        if self.is_empty() {
            return Size::default();
        }
        let mut min_x = self.dots[0].x();
        let mut max_x = self.dots[0].x();
        let mut min_y = self.dots[0].y();
        let mut max_y = self.dots[0].y();

        for v in self.dots.iter() {
            min_x = min(min_x, v.x());
            max_x = max(max_x, v.x());
            min_y = min(min_y, v.y());
            max_y = max(max_y, v.y());
        }

        let width = (max_x - min_x) as u64;
        let height = (max_y - min_y) as u64;
        Size::new(width, height)
    }

    pub fn square(&self) -> u64 {
        if self.dots.len() < 3 {
            return 0;
        }
        let mut area: i64 = 0;
        let n = self.dots.len();
        for i in 0..n {
            let p1 = &self.dots[i];
            let p2 = &self.dots[(i + 1) % n];
            area += p1.x() * p2.y() - p2.x() * p1.y();
        }
        (area.abs() / 2) as u64
    }

    pub fn vertices_global(&self, pos: &Position) -> Vec<Position> {
        self.dots
            .iter()
            .map(|v| Position::new(pos.x() + v.x(), pos.y() + v.y()))
            .collect()
    }

    pub fn bounding_box(&self, pos: &Position) -> (Position, Size) {
        if self.is_empty() {
            return (Position::new(0, 0), Size::default());
        }
        let mut min_x = self.dots[0].x();
        let mut max_x = self.dots[0].x();
        let mut min_y = self.dots[0].y();
        let mut max_y = self.dots[0].y();

        for v in self.dots.iter() {
            min_x = min(min_x, v.x());
            max_x = max(max_x, v.x());
            min_y = min(min_y, v.y());
            max_y = max(max_y, v.y());
        }

        let top_left = Position::new(pos.x() + min_x, pos.y() + min_y);
        let width = (max_x - min_x) as u64;
        let height = (max_y - min_y) as u64;
        (top_left, Size::new(width, height))
    }

    pub fn are_in(&self, self_pos: &Position, pos: &Position) -> bool {
        if self.dots.len() < 3 {
            return false;
        }

        let px = pos.x() - self_pos.x();
        let py = pos.y() - self_pos.y();

        let mut inside = false;
        let n = self.dots.len();
        for i in 0..n {
            let v1 = &self.dots[i];
            let v2 = &self.dots[(i + 1) % n];

            if Self::point_on_segment(px, py, v1.x(), v1.y(), v2.x(), v2.y()) {
                return true;
            }

            let intersect = ((v1.y() > py) != (v2.y() > py))
                && (px < (v2.x() - v1.x()) * (py - v1.y()) / (v2.y() - v1.y()) + v1.x());
            if intersect {
                inside = !inside;
            }
        }
        inside
    }

    fn point_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
        let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
        if cross != 0 {
            return false;
        }
        let dot = (px - x1) * (x2 - x1) + (py - y1) * (y2 - y1);
        if dot < 0 {
            return false;
        }
        let squared_len = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
        dot <= squared_len
    }

    pub fn collide(&self, self_pos: &Position, other: &Geometry, other_pos: &Position) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        let (self_bb, self_size) = self.bounding_box(self_pos);
        let (other_bb, other_size) = other.bounding_box(other_pos);
        if !Self::bbox_intersect(&self_bb, &self_size, &other_bb, &other_size) {
            return false;
        }

        let self_global = self.vertices_global(self_pos);
        let other_global = other.vertices_global(other_pos);
        if Self::edges_intersect(&self_global, &other_global) {
            return true;
        }

        for v in &self_global {
            if other.are_in(other_pos, v) {
                return true;
            }
        }

        for v in &other_global {
            if self.are_in(self_pos, v) {
                return true;
            }
        }

        false
    }

    fn bbox_intersect(a_pos: &Position, a_size: &Size, b_pos: &Position, b_size: &Size) -> bool {
        let a_left = a_pos.x();
        let a_right = a_pos.x() + a_size.w() as i64;
        let a_top = a_pos.y();
        let a_bottom = a_pos.y() + a_size.h() as i64;

        let b_left = b_pos.x();
        let b_right = b_pos.x() + b_size.w() as i64;
        let b_top = b_pos.y();
        let b_bottom = b_pos.y() + b_size.h() as i64;

        !(a_right <= b_left || a_left >= b_right || a_bottom <= b_top || a_top >= b_bottom)
    }

    fn edges_intersect(a: &[Position], b: &[Position]) -> bool {
        let n = a.len();
        let m = b.len();
        for i in 0..n {
            let a1 = &a[i];
            let a2 = &a[(i + 1) % n];
            for j in 0..m {
                let b1 = &b[j];
                let b2 = &b[(j + 1) % m];
                if Self::segments_intersect(
                    a1.x(),
                    a1.y(),
                    a2.x(),
                    a2.y(),
                    b1.x(),
                    b1.y(),
                    b2.x(),
                    b2.y(),
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn segments_intersect(
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
        x4: i64,
        y4: i64,
    ) -> bool {
        let o1 = Self::orientation(x1, y1, x2, y2, x3, y3);
        let o2 = Self::orientation(x1, y1, x2, y2, x4, y4);
        let o3 = Self::orientation(x3, y3, x4, y4, x1, y1);
        let o4 = Self::orientation(x3, y3, x4, y4, x2, y2);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == 0 && Self::on_segment(x1, y1, x3, y3, x2, y2) {
            return true;
        }
        if o2 == 0 && Self::on_segment(x1, y1, x4, y4, x2, y2) {
            return true;
        }
        if o3 == 0 && Self::on_segment(x3, y3, x1, y1, x4, y4) {
            return true;
        }
        if o4 == 0 && Self::on_segment(x3, y3, x2, y2, x4, y4) {
            return true;
        }

        false
    }

    fn orientation(px: i64, py: i64, qx: i64, qy: i64, rx: i64, ry: i64) -> i32 {
        let val = (qy - py) * (rx - qx) - (qx - px) * (ry - qy);
        if val == 0 {
            0
        } else if val > 0 {
            1
        } else {
            2
        }
    }

    fn on_segment(px: i64, py: i64, qx: i64, qy: i64, rx: i64, ry: i64) -> bool {
        qx <= max(px, rx) && qx >= min(px, rx) && qy <= max(py, ry) && qy >= min(py, ry)
    }
}

impl Geometry {
    pub fn range(&self) -> u64 {
        let pivot = self.pivot();
        let mut maxx = 0;
        let _ = self
            .dots
            .iter()
            .map(|p| {
                let d = pivot.dist(p);
                if d > maxx {
                    maxx = d;
                }
            })
            .collect::<Vec<()>>();
        maxx
    }

    pub fn pivot(&self) -> Position {
        if self.is_empty() {
            return Position::new(0, 0);
        }
        let mut sum_x = 0i64;
        let mut sum_y = 0i64;
        for v in self.dots.iter() {
            sum_x += v.x();
            sum_y += v.y();
        }
        let n = self.dots.len() as i64;
        Position::new(sum_x / n, sum_y / n)
    }

    pub fn rotate(&mut self, deg: usize) {
        if self.is_empty() {
            return;
        }
        let pivot = self.pivot();
        let angle = (deg as f64).to_radians();
        let cos = angle.cos();
        let sin = angle.sin();

        for v in self.dots.iter_mut() {
            let dx = v.x() - pivot.x();
            let dy = v.y() - pivot.y();

            let new_x = dx as f64 * cos - dy as f64 * sin;
            let new_y = dx as f64 * sin + dy as f64 * cos;

            *v = Position::new(
                pivot.x() + new_x.round() as i64,
                pivot.y() + new_y.round() as i64,
            );
        }
    }

    pub fn intersection(
        self,
        self_pos: &Position,
        other: Geometry,
        other_pos: &Position,
    ) -> Option<Geometry> {
        if self.is_empty() || other.is_empty() {
            return None;
        }

        let (self_bb, self_size) = self.bounding_box(self_pos);
        let (other_bb, other_size) = other.bounding_box(other_pos);
        if !Self::bbox_intersect(&self_bb, &self_size, &other_bb, &other_size) {
            return None;
        }

        let mut subject = self.vertices_global(self_pos);
        let object = other.vertices_global(other_pos);

        for i in 0..object.len() {
            let p1 = &object[i];
            let p2 = &object[(i + 1) % object.len()];

            let mut output = Vec::new();
            let n = subject.len();
            if n == 0 {
                break;
            }

            let mut prev = subject[n - 1].clone();
            let mut prev_inside = Self::is_left(p1, p2, &prev) >= 0;

            for j in 0..n {
                let current = &subject[j];
                let current_inside = Self::is_left(p1, p2, current) >= 0;

                if current_inside {
                    if !prev_inside {
                        if let Some(intersect) =
                            Self::segment_intersection_point(&prev, current, p1, p2)
                        {
                            output.push(intersect);
                        }
                    }
                    output.push(current.clone());
                } else {
                    if prev_inside {
                        if let Some(intersect) =
                            Self::segment_intersection_point(&prev, current, p1, p2)
                        {
                            output.push(intersect);
                        }
                    }
                }

                prev = current.clone();
                prev_inside = current_inside;
            }

            subject = output;
            if subject.is_empty() {
                return None;
            }
        }

        Some(Geometry::new(subject))
    }

    fn is_left(p1: &Position, p2: &Position, p: &Position) -> i64 {
        (p2.x() - p1.x()) * (p.y() - p1.y()) - (p2.y() - p1.y()) * (p.x() - p1.x())
    }

    fn segment_intersection_point(
        a1: &Position,
        a2: &Position,
        b1: &Position,
        b2: &Position,
    ) -> Option<Position> {
        let denom = (a1.x() - a2.x()) as f64 * (b1.y() - b2.y()) as f64
            - (a1.y() - a2.y()) as f64 * (b1.x() - b2.x()) as f64;
        if denom.abs() < 1e-9 {
            return None;
        }

        let t = ((a1.x() - b1.x()) as f64 * (b1.y() - b2.y()) as f64
            - (a1.y() - b1.y()) as f64 * (b1.x() - b2.x()) as f64)
            / denom;

        if t < 0.0 || t > 1.0 {
            return None;
        }

        let x = a1.x() as f64 + t * (a2.x() - a1.x()) as f64;
        let y = a1.y() as f64 + t * (a2.y() - a1.y()) as f64;
        Some(Position::new(x.round() as i64, y.round() as i64))
    }
}
