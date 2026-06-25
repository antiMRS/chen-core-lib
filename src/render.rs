use crate::builtins::{Dims, Geometry, Position, Size};

pub const EMPTY_CHAR: char = ' ';

use crate::buffer::Buffer;

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

    pub fn add_color(&mut self, other: Color) {
        self.rgb[0] = self.rgb[0].saturating_add(other.rgb[0]);
        self.rgb[1] = self.rgb[1].saturating_add(other.rgb[1]);
        self.rgb[2] = self.rgb[2].saturating_add(other.rgb[2]);
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

    pub fn draw_sprite(&mut self, sprite: &Sprite, x: usize, y: usize) {
        self.chars
            .draw(&self.size, &sprite.chars, &sprite.size, x, y);
        #[cfg(feature = "colored")]
        self.colors
            .draw(&self.size, &sprite.colors, &sprite.size, x, y);
        #[cfg(feature = "styled")]
        self.styles
            .draw(&self.size, &sprite.styles, &sprite.size, pos);
    }

    pub fn blit_sprite(&mut self, sprite: &Sprite, pos: &Position) {
        let w = self.size.w() as i64;
        let h = self.size.h() as i64;

        for i in 0..sprite.chars.buf.len() {
            let local_pos = Position::from_flat(i as i64, &sprite.size);
            let target_x = local_pos.x() + pos.x();
            let target_y = local_pos.y() + pos.y();
            if target_x < 0 || target_y < 0 || target_x >= w || target_y >= h {
                continue;
            }
            let target_pos = Position::new(target_x, target_y);
            let target_idx = target_pos.flat(&self.size) as usize;
            let src_idx = i;

            if sprite.chars.buf[src_idx] != EMPTY_CHAR {
                self.chars.buf[target_idx] = sprite.chars.buf[src_idx];
                #[cfg(feature = "colored")]
                {
                    self.colors.buf[target_idx] = sprite.colors.buf[src_idx];
                }
                #[cfg(feature = "styled")]
                {
                    self.styles.buf[target_idx] = sprite.styles.buf[src_idx];
                }
            }
        }
    }

    // ====================== chars ========================

    pub fn get_char(&self, x: usize, y: usize) -> char {
        self.chars.get(&self.size, x, y)
    }

    pub fn draw(&mut self, chr: char, x: usize, y: usize) {
        self.chars.set(&self.size, x, y, chr)
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

    pub fn fill_with_f<T>(&mut self, f: T)
    where
        T: FnMut(u64, u64) -> char,
    {
        self.chars.fill_with_f(&self.size, f);
    }

    // ====================== colors ========================

    #[cfg(feature = "colored")]
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors.get(&self.size, x, y)
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
    pub fn paint(&mut self, color: Color, x: usize, y: usize) {
        self.colors.set(&self.size, x, y, color)
    }

    #[cfg(feature = "colored")]
    pub fn draw_colored(&mut self, chr: char, x: usize, y: usize, color: Color) {
        self.chars.set(&self.size, x, y, chr);
        self.colors.set(&self.size, x, y, color);
    }

    // ====================== styles ========================

    #[cfg(feature = "styled")]
    pub fn get_style(&self, pos: &Position) -> CharStyle {
        self.styles.get(&self.size, pos)
    }
}

// ====================== alpha ========================

#[cfg(feature = "alpha")]
impl Sprite {
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn set_alpha(&mut self, a: u8) {
        self.alpha = a;
    }
}
