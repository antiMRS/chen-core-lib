use crate::builtins::Dims;
use crate::builtins::{Position, Size};

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

#[derive(Debug, Clone)]
pub struct Sprite {
    buf: Box<[char]>,
    size: Size,
}

impl Sprite {
    pub fn new(sx: usize, sy: usize) -> Self {
        Self {
            buf: vec![EMPTY_CHAR; sx * sy].into_boxed_slice(),
            size: Size::new(sx, sy),
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn fill(&mut self, chr: char) {
        self.buf = vec![chr; self.size.flat()].into_boxed_slice();
    }

    pub fn draw(&mut self, chr: char, pos: Position) {
        self.buf[pos.flat(&self.size) as usize] = chr;
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: &Position) {
        for (i, chr) in sprite.buf.iter().enumerate() {
            let local_pos = Position::from_flat(i as u64, sprite.size());
            let target_pos = Position::new(local_pos.x() + pos.x(), local_pos.y() + pos.y());
            if (target_pos.x() as usize) < self.size.w()
                && (target_pos.y() as usize) < self.size.h()
            {
                self.draw(*chr, target_pos);
            }
        }
    }

    pub fn get(&self, pos: &Position) -> char {
        self.buf[pos.flat(&self.size) as usize]
    }
}
