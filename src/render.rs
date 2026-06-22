use std::fmt::Debug;

use crate::builtins::{Dims, Geometry, Position, Size};

pub const EMPTY_CHAR: char = ' ';

#[derive(Clone)]
#[repr(transparent)]
pub struct Buffer<T: Default + Copy> {
    buf: Box<[T]>,
}

impl<T: Default + Copy> Buffer<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: vec![T::default(); w * h].into_boxed_slice(),
        }
    }

    pub fn new_filled(w: usize, h: usize, what: T) -> Self {
        Self {
            buf: vec![what; w * h].into_boxed_slice(),
        }
    }

    pub fn get(&self, size: &Size, pos: &Position) -> T {
        self.buf[pos.flat(size) as usize]
    }

    pub fn set(&mut self, size: &Size, pos: &Position, what: T) {
        let idx = pos.flat(size) as usize;
        self.buf[idx] = what;
    }

    pub fn fill(&mut self, size: &Size, what: T) {
        let total = size.flat() as usize;
        self.buf = vec![what; total].into_boxed_slice();
    }

    pub fn draw(&mut self, size: &Size, other: &Buffer<T>, other_size: &Size, pos: &Position) {
        let w = size.w() as i64;
        let h = size.h() as i64;

        for i in 0..other.buf.len() {
            let local_pos = Position::from_flat(i as i64, other_size);
            let target_x = local_pos.x() + pos.x();
            let target_y = local_pos.y() + pos.y();
            if target_x < 0 || target_y < 0 || target_x >= w || target_y >= h {
                continue;
            }
            let target_pos = Position::new(target_x, target_y);
            let target_idx = target_pos.flat(size) as usize;
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
                        self.set(size, &Position::new(x, y), what);
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
                self.set(size, &Position::new(x, y), what.clone());
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

#[derive(Debug)]
pub struct UDim(Dims<f64, 2>);

impl UDim {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Dims::from([x, y]))
    }
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
}

impl std::fmt::Display for UDim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UDim")
            .field("X", &self.x())
            .field("Y", &self.y())
            .finish()
    }
}

impl Clone for UDim {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    rgb: Dims<u8, 3>,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb: Dims::from([r, g, b]),
        }
    }
    pub fn default() -> Self {
        Self {
            rgb: Dims::default(),
        }
    }
    pub fn r(&self) -> u8 {
        self.rgb[0]
    }
    pub fn g(&self) -> u8 {
        self.rgb[1]
    }
    pub fn b(&self) -> u8 {
        self.rgb[2]
    }

    pub fn as_legacy(&self) -> u8 {
        match (self.r(), self.g(), self.b()) {
            (0, 0, 0) => 30,
            (255, 0, 0) => 31,
            (0, 255, 0) => 32,
            (255, 128, 0) => 33,
            (0, 0, 255) => 34,
            (255, 0, 255) => 35,
            (0, 255, 255) => 36,
            (255, 255, 255) => 37,
            _ => 0,
        }
    }
    pub fn as_ascii(&self) -> u8 {
        let r_idx = (self.rgb[0] as u16 + 25) / 51;
        let g_idx = (self.rgb[1] as u16 + 25) / 51;
        let b_idx = (self.rgb[2] as u16 + 25) / 51;

        16 + (r_idx * 36 + g_idx * 6 + b_idx) as u8
    }
}

#[cfg(feature = "styled")]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CharStyle {
    #[default]
    Normal,
    Bold,
    Underline,
}

#[cfg(feature = "styled")]
impl CharStyle {
    pub fn as_ascii(&self) -> u8 {
        match self {
            CharStyle::Normal => 0,
            CharStyle::Bold => 1,
            CharStyle::Underline => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    chars: Buffer<char>,
    size: Size,
    #[cfg(feature = "colored")]
    colors: Buffer<Color>,
    #[cfg(feature = "styled")]
    styles: Buffer<CharStyle>,
}

impl Sprite {
    pub fn new(sx: usize, sy: usize) -> Self {
        Self {
            chars: Buffer::new_filled(sx, sy, EMPTY_CHAR),
            size: Size::new(sx as u64, sy as u64),
            #[cfg(feature = "colored")]
            colors: Buffer::new(sx, sy),
            #[cfg(feature = "styled")]
            styles: Buffer::new(sx, sy),
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: &Position) {
        self.chars
            .draw(&self.size, &sprite.chars, &sprite.size, pos);
        #[cfg(feature = "colored")]
        self.colors
            .draw(&self.size, &sprite.colors, &sprite.size, pos);
        #[cfg(feature = "styled")]
        self.styles
            .draw(&self.size, &sprite.styles, &sprite.size, pos);
    }

    // ====================== chars ========================

    pub fn get_char(&self, pos: &Position) -> char {
        self.chars.get(&self.size, pos)
    }

    pub fn draw(&mut self, chr: char, pos: &Position) {
        self.chars.set(&self.size, pos, chr)
    }

    pub fn fill(&mut self, chr: char) {
        self.chars.fill(&self.size, chr);
    }

    pub fn geometry_draw(&mut self, geom: &Geometry, pos: &Position, what: char) {
        self.chars.geometry_draw(&self.size, geom, pos, what);
    }

    pub fn geometry_draw_filled(&mut self, geom: &Geometry, pos: &Position, what: char) {
        self.chars.geometry_draw_filled(&self.size, geom, pos, what);
    }

    // ====================== colors ========================

    #[cfg(feature = "colored")]
    pub fn get_color(&self, pos: &Position) -> Color {
        self.colors.get(&self.size, pos)
    }

    #[cfg(feature = "colored")]
    pub fn geometry_paint(&mut self, geom: &Geometry, pos: &Position, what: Color) {
        self.colors.geometry_draw(&self.size, geom, pos, what);
    }
    #[cfg(feature = "colored")]
    pub fn geometry_paint_filled(&mut self, geom: &Geometry, pos: &Position, what: Color) {
        self.colors
            .geometry_draw_filled(&self.size, geom, pos, what);
    }

    #[cfg(feature = "colored")]
    pub fn fill_color(&mut self, color: Color) {
        self.colors.fill(&self.size, color);
    }

    #[cfg(feature = "colored")]
    pub fn paint(&mut self, color: Color, pos: &Position) {
        self.colors.set(&self.size, pos, color)
    }

    #[cfg(feature = "colored")]
    pub fn draw_colored(&mut self, chr: char, pos: &Position, color: Color) {
        self.chars.set(&self.size, pos, chr);
        self.colors.set(&self.size, pos, color);
    }

    // ====================== styles ========================

    #[cfg(feature = "styled")]
    pub fn get_style(&self, pos: &Position) -> CharStyle {
        self.styles.get(&self.size, pos)
    }
}
