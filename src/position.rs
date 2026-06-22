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

#[derive(Debug, Clone, Default)]
pub struct Position(Dims<i64, 2>);

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self(Dims::from([x, y]))
    }

    pub fn x(&self) -> i64 {
        self.0[0]
    }

    pub fn y(&self) -> i64 {
        self.0[1]
    }

    pub fn flat_mul(mut self, w: i64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

    pub fn as_tuple(self) -> (i64, i64) {
        (self.x(), self.y())
    }

    pub fn flat(&self, size: &Size) -> i64 {
        debug_assert!(
            self.x() < size.w() as i64 && self.y() < size.h() as i64,
            "Position out of bounds"
        );
        self.y() * (size.w() as i64) + self.x()
    }

    pub fn from_flat(xy: i64, size: &Size) -> Self {
        let width = size.w() as i64;
        assert!(width > 0, "Width needs to be greater then 0");
        let x = xy.rem_euclid(width);
        let y = xy.div_euclid(width);
        Position::new(x, y)
    }

    pub fn add(&mut self, x: i64, y: i64) {
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

#[derive(Debug, Clone, Default)]
pub struct Size(Dims<u64, 2>);

impl Size {
    pub fn new(x: u64, y: u64) -> Self {
        Self(Dims::from([x, y]))
    }

    pub fn w(&self) -> u64 {
        self.0[0]
    }

    pub fn h(&self) -> u64 {
        self.0[1]
    }

    pub fn flat_mul(mut self, w: u64) -> Self {
        self.0[0] *= w;
        self.0[1] *= w;
        self
    }

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

#[derive(Debug, Clone)]
pub struct Geometry {
    dots: Box<[Position]>,
}

use std::cmp::max;
use std::cmp::min;

impl Geometry {
    pub fn new(dots: Vec<Position>) -> Self {
        Self {
            dots: dots.into_boxed_slice(),
        }
    }
    /// Creates a square geometry with side length `a`.
    /// Vertices are in counter-clockwise order: (0,0), (a,0), (a,a), (0,a).
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

    /// Returns the number of vertices.
    pub fn len(&self) -> u64 {
        self.dots.len() as u64
    }

    /// Returns true if the geometry has no vertices.
    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    /// Returns the bounding box size of the geometry (without position).
    /// The size is the width and height of the minimal axis-aligned rectangle
    /// that contains all vertices.
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

    /// Returns the area of the polygon using the gause formula.
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

    /// Returns the vertices in global coordinates (relative to `pos`).
    pub fn vertices_global(&self, pos: &Position) -> Vec<Position> {
        self.dots
            .iter()
            .map(|v| Position::new(pos.x() + v.x(), pos.y() + v.y()))
            .collect()
    }

    /// Returns the bounding box of the geometry placed at `pos` as (top-left corner, size).
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

    /// Checks if a point (in global coordinates) lies inside the geometry placed at `self_pos`.
    /// Uses the ray casting algorithm (even-odd rule).
    pub fn are_in(&self, self_pos: &Position, pos: &Position) -> bool {
        if self.dots.len() < 3 {
            return false;
        }

        // Transform point to local coordinates
        let px = pos.x() - self_pos.x();
        let py = pos.y() - self_pos.y();

        let mut inside = false;
        let n = self.dots.len();
        for i in 0..n {
            let v1 = &self.dots[i];
            let v2 = &self.dots[(i + 1) % n];

            // Check if the point is exactly on a vertex or edge
            if Self::point_on_segment(px, py, v1.x(), v1.y(), v2.x(), v2.y()) {
                return true;
            }

            // Ray casting: check if edge crosses the horizontal ray to the right (positive x)
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

    /// Checks if two geometries (with their positions) intersect.
    /// Works for arbitrary simple polygons (convex or concave) by checking:
    /// - Any edge intersection between the two polygons.
    /// - Any vertex of one inside the other.
    pub fn collide(&self, self_pos: &Position, other: &Geometry, other_pos: &Position) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        // Quick bounding box check
        let (self_bb, self_size) = self.bounding_box(self_pos);
        let (other_bb, other_size) = other.bounding_box(other_pos);
        if !Self::bbox_intersect(&self_bb, &self_size, &other_bb, &other_size) {
            return false;
        }

        // Check edge intersections
        let self_global = self.vertices_global(self_pos);
        let other_global = other.vertices_global(other_pos);
        if Self::edges_intersect(&self_global, &other_global) {
            return true;
        }

        // Check if any vertex of self is inside other
        for v in &self_global {
            if other.are_in(other_pos, v) {
                return true;
            }
        }

        // Check if any vertex of other is inside self
        for v in &other_global {
            if self.are_in(self_pos, v) {
                return true;
            }
        }

        false
    }

    /// Checks if two axis-aligned bounding boxes intersect.
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

        // General case
        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special cases (collinear)
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
