use fontdue::Metrics;
mod basic_font;

type Glyph = [u8; 8];
#[allow(unused)]
const EMPTY_GLYPH: Glyph = [0; 8];

pub trait Font {
    fn get(&self, chr: char) -> Option<Glyph>;
}

fn to_glyph(metrics: Metrics, bitmap: &[u8], waytru: u8) -> Glyph {
    let mut glyph = [0u8; 8];
    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let rest = bitmap[y * metrics.width + x];
            if rest > waytru && x < 8 && y < 8 {
                glyph[y] |= 1 << x;
            }
        }
    }

    glyph
}

pub fn index_glyph(gl: &Glyph, x: usize, y: usize) -> bool {
    (gl[y] >> x) & 1 == 1
}

pub fn align_left_bottom(glyph: [u8; 8]) -> [u8; 8] {
    let mut result = [0u8; 8];

    let mut bottom = 0;
    let mut has_nonzero = false;
    for i in (0..8).rev() {
        if glyph[i] != 0 {
            bottom = i;
            has_nonzero = true;
            break;
        }
    }
    if !has_nonzero {
        return result;
    }

    let mut left = 0;
    for &byte in &glyph {
        if byte != 0 {
            let pos = 7 - byte.leading_zeros() as usize;
            if pos > left {
                left = pos;
            }
        }
    }

    let shift_down = 7 - bottom;
    let shift_left = 7 - left;

    for i in 0..=bottom {
        let new_row = glyph[i] << shift_left;
        result[i + shift_down] = new_row;
    }

    result
}

#[cfg(test)]
mod test {
    pub use crate::*;

    #[test]
    fn main() {
        let font = BasicFont::new();

        let glyph = font.get('$').unwrap();

        for y in 0..8 {
            for x in 0..8 {
                if index_glyph(&glyph, x, y) {
                    print!("N")
                } else {
                    print!(" ")
                }
            }
            println!("")
        }
    }
}

pub use basic_font::BasicFont;
