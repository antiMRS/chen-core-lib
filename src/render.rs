use std::fmt::Debug;

use crate::position::Dims;
use crate::position::{Position, Size};

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

#[cfg(feature = "colored")]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    rgb: Dims<u8, 3>,
}

#[cfg(feature = "colored")]
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
        let total = sx * sy;
        Self {
            chars: Buffer::new(sx, sy),
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

    pub fn fill(&mut self, chr: char) {
        self.chars.fill(&self.size, chr);
    }

    #[cfg(feature = "colored")]
    pub fn fill_color(&mut self, color: Color) {
        self.colors.fill(&self.size, color);
    }

    pub fn draw(&mut self, chr: char, pos: &Position) {
        self.chars.set(&self.size, pos, chr)
    }

    #[cfg(feature = "colored")]
    pub fn draw_colored(&mut self, chr: char, pos: &Position, color: Color) {
        self.chars.set(&self.size, pos, chr);
        self.colors.set(&self.size, pos, color);
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: &Position) {
        self.chars
            .draw(&self.size, &sprite.chars, &sprite.size, pos);
    }

    pub fn get_char(&self, pos: &Position) -> char {
        self.chars.get(&self.size, pos)
    }

    #[cfg(feature = "colored")]
    pub fn get_color(&self, pos: &Position) -> Color {
        self.colors.get(&self.size, pos)
    }
    #[cfg(feature = "styled")]
    pub fn get_style(&self, pos: &Position) -> CharStyle {
        self.styles.get(&self.size, pos)
    }
}
