use crate::builtins::{Geometry, Position, Size};
use std::fmt::Debug;

#[derive(Clone)]
#[repr(transparent)]
pub struct Buffer<T: Default + Copy> {
    pub(crate) buf: Box<[T]>,
}

impl<T: Default + Copy> Buffer<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: vec![T::default(); w * h].into_boxed_slice(),
        }
    }

    //pub fn len(&self) -> usize {
    //    self.buf.len()
    //}

    pub fn new_filled(w: usize, h: usize, what: T) -> Self {
        Self {
            buf: vec![what; w * h].into_boxed_slice(),
        }
    }

    pub fn get(&self, size: &Size, x: usize, y: usize) -> T {
        self.buf[Position::flattened(size, x, y)]
    }

    pub fn set(&mut self, size: &Size, x: usize, y: usize, what: T) {
        let idx = Position::flattened(size, x, y);
        self.buf[idx] = what;
    }

    pub fn fill(&mut self, size: &Size, what: T) {
        let total = size.flat() as usize;
        self.buf = vec![what; total].into_boxed_slice();
    }

    pub fn fill_with_f<F>(&mut self, size: &Size, mut f: F)
    where
        F: FnMut(u64, u64) -> T,
    {
        for y in 0..size.h() {
            for x in 0..size.w() {
                let ch = f(x, y);
                self.buf[Position::flattened(size, x, y) as usize] = ch;
            }
        }
    }

    pub fn draw(&mut self, size: &Size, other: &Self, other_size: &Size, x: usize, y: usize) {
        let w = size.w() as i64;
        let h = size.h() as i64;

        for i in 0..other.buf.len() {
            let local_pos = Position::from_flat(other_size, i as i64);
            let target_x = local_pos.x() + x as i64;
            let target_y = local_pos.y() + y as i64;
            if target_x < 0 || target_y < 0 || target_x >= w || target_y >= h {
                continue;
            }
            let target_idx = Position::flattened(size, target_x, target_y) as usize;
            let src_idx = i;

            self.buf[target_idx] = other.buf[src_idx];
        }
    }
}

impl<T: Clone + Copy + Default> Buffer<T> {
    pub fn geometry_draw(&mut self, size: &Size, geom: &Geometry, pos: &Position, what: T) {
        let verts = geom.vertices_global(pos);

        let n = verts.len();
        if n < 2 {
            return;
        }
        for i in 0..n {
            let p1 = &verts[i];
            let p2 = &verts[(i + 1) % n];
            self.draw_line(size, p1, p2, what);
        }
    }

    pub fn geometry_draw_filled(&mut self, size: &Size, geom: &Geometry, pos: &Position, what: T) {
        let verts = geom.vertices_global(pos);
        let n = verts.len();
        if n < 3 {
            return;
        }

        let mut min_y = verts[0].y();
        let mut max_y = verts[0].y();
        for v in &verts {
            if v.y() < min_y {
                min_y = v.y();
            }
            if v.y() > max_y {
                max_y = v.y();
            }
        }

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..n {
                let p1 = &verts[i];
                let p2 = &verts[(i + 1) % n];

                if p1.y() == p2.y() {
                    continue;
                }

                if (p1.y() <= y && p2.y() > y) || (p2.y() <= y && p1.y() > y) {
                    let x = p1.x() + (y - p1.y()) * (p2.x() - p1.x()) / (p2.y() - p1.y());
                    intersections.push(x);
                }
            }

            intersections.sort();

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i];
                    let x2 = intersections[i + 1];
                    for x in x1..=x2 {
                        self.set(size, x as usize, y as usize, what);
                    }
                }
            }
        }
    }

    fn draw_line(&mut self, size: &Size, p1: &Position, p2: &Position, what: T) {
        let x1 = p1.x();
        let y1 = p1.y();
        let x2 = p2.x();
        let y2 = p2.y();

        let w = size.w() as i64;
        let h = size.h() as i64;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x1;
        let mut y = y1;

        loop {
            if x >= 0 && x < w && y >= 0 && y < h {
                self.set(size, x as usize, y as usize, what);
            }
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl<T: Debug + Default + Copy> Debug for Buffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.buf)
    }
}
