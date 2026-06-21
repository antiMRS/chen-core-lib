// terminal.rs
use crate::builtins::{Position, Size};
use crate::render::{EMPTY_CHAR, Sprite};

use std::io::{self, Write};
use std::process::Command;

pub struct Terminal {
    buf: Sprite,
}

impl Terminal {
    #[cfg(windows)]
    pub fn new(title: &str, w: usize, h: usize) -> Self {
        // Установка заголовка окна (только Windows)
        let _ = Command::new("title").arg(title).status();

        unsafe {
            Terminal {
                buf: Sprite::new(w, h),
            }
        }
    }
    pub fn blit(&mut self, sprite: &Sprite, pos: &Position) {
        self.buf.draw_sprite(sprite, pos);
    }
    pub fn clear(&mut self) {
        self.buf.fill(EMPTY_CHAR);
    }

    pub fn render(&self) {
        #[cfg(windows)]
        {
            let _ = Command::new("cmd").args(&["/c", "cls"]).status();
        }
        #[cfg(unix)]
        {
            let _ = Command::new("clear").status();
        }

        let w = self.buf.size().w() as usize;
        let h = self.buf.size().h() as usize;
        let mut stdout = io::stdout();

        for y in 0..h {
            for x in 0..w {
                let pos = Position::new(x as i64, y as i64);
                let ch = self.buf.get(&pos);
                let _ = write!(stdout, "{}", ch);
            }
            let _ = writeln!(stdout);
        }
        let _ = stdout.flush();
    }
}
