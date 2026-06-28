use fontdue::{Font as _Font, FontSettings};
use std::{cell::RefCell, collections::HashMap};

use crate::{Font, Glyph, align_left_bottom, to_glyph};

pub struct BasicFont {
    cache: RefCell<HashMap<char, Glyph>>,
    font: _Font,
}

impl BasicFont {
    const FONT: &[u8] = include_bytes!("microsoftsansserif.ttf");
    const FONT_RATIO: f32 = 9.5;

    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
            font: _Font::from_bytes(Self::FONT, FontSettings::default()).unwrap(),
        }
    }
}

impl Font for BasicFont {
    fn get(&self, chr: char) -> Option<Glyph> {
        if !self.cache.borrow().contains_key(&chr) {
            let (metrics, bitmap) = self.font.rasterize(chr, Self::FONT_RATIO);
            let gl = to_glyph(metrics, &bitmap, 60);
            let gl = align_left_bottom(gl);
            self.cache.borrow_mut().insert(chr, gl);
            Some(gl)
        } else {
            Some(self.cache.borrow()[&chr])
        }
    }
}
