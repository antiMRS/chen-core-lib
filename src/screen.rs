use crate::{Position, Sprite};

pub struct Screen {
    buf: Sprite,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: Sprite::new(w, h),
        }
    }

    pub fn draw(&mut self, what: &Sprite, pos: &Position) {
        self.buf.draw_sprite(what, pos);
    }

    pub fn clear(&mut self) {
        self.buf.fill(crate::EMPTY_CHAR);
    }

    pub fn show(&self) {
        let mut r = String::new();

        for i in 0..self.buf.size().flat() {
            let pos = Position::from_flat(i as u64, &self.buf.size());
            if pos.x() == 0 {
                r = format!("{r}\n");
            }
            r = format!("{r}{}", self.buf.get(&pos));
        }

        println!("{}", r)
    }
}
