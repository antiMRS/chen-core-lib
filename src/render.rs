use crate::position::Dims;
use crate::position::{Position, Size};

pub const EMPTY_CHAR: char = ' ';

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
#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn as_ascii(&self) -> u8 {
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
}

#[derive(Debug)]
pub struct Sprite {
    buf: Box<[char]>,
    size: Size,
    #[cfg(feature = "colored")]
    colors: Box<[Color]>,
}

impl Sprite {
    pub fn new(sx: usize, sy: usize) -> Self {
        let total = sx * sy;
        Self {
            buf: vec![EMPTY_CHAR; total].into_boxed_slice(),
            size: Size::new(sx as u64, sy as u64),
            #[cfg(feature = "colored")]
            colors: vec![Color::default(); total].into_boxed_slice(),
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn fill(&mut self, chr: char) {
        let total = self.size.flat() as usize;
        self.buf = vec![chr; total].into_boxed_slice();
    }

    #[cfg(feature = "colored")]
    pub fn fill_color(&mut self, color: Color) {
        let total = self.size.flat() as usize;
        self.colors = vec![color; total].into_boxed_slice();
    }

    pub fn draw(&mut self, chr: char, pos: Position) {
        let idx = pos.flat(&self.size) as usize;
        self.buf[idx] = chr;
        #[cfg(feature = "colored")]
        {
            self.colors[idx] = Color::default();
        }
    }

    #[cfg(feature = "colored")]
    pub fn draw_colored(&mut self, chr: char, pos: Position, color: Color) {
        let idx = pos.flat(&self.size) as usize;
        self.buf[idx] = chr;
        self.colors[idx] = color;
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: &Position) {
        let w = self.size.w() as i64;
        let h = self.size.h() as i64;
        let sw = sprite.size().w() as i64;
        let sh = sprite.size().h() as i64;

        for i in 0..sprite.buf.len() {
            let local_pos = Position::from_flat(i as i64, sprite.size());
            let target_x = local_pos.x() + pos.x();
            let target_y = local_pos.y() + pos.y();
            if target_x < 0 || target_y < 0 || target_x >= w || target_y >= h {
                continue;
            }
            let target_pos = Position::new(target_x, target_y);
            let target_idx = target_pos.flat(&self.size) as usize;
            let src_idx = i;

            self.buf[target_idx] = sprite.buf[src_idx];

            #[cfg(feature = "colored")]
            {
                self.colors[target_idx] = sprite.colors[src_idx];
            }
        }
    }

    pub fn get(&self, pos: &Position) -> char {
        self.buf[pos.flat(&self.size) as usize]
    }

    #[cfg(feature = "colored")]
    pub fn get_color(&self, pos: &Position) -> Color {
        self.colors[pos.flat(&self.size) as usize]
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        let total = self.size.flat() as usize;
        Self {
            buf: self.buf.clone(),
            size: self.size.clone(),
            #[cfg(feature = "colored")]
            colors: self.colors.clone(),
        }
    }
}
