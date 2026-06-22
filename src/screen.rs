// terminal.rs
use crate::builtins::{Position, Size};
#[cfg(feature = "colored")]
use crate::render::Color;
use crate::render::{EMPTY_CHAR, Sprite};

use std::io::{self, Write};
use std::process::Command;

pub struct Terminal {
    buf: Sprite,
}

impl Terminal {
    #[cfg(windows)]
    pub fn new(title: &str, w: usize, h: usize) -> Self {
        let _ = Command::new("title").arg(title).status();
        Self {
            buf: Sprite::new(w, h),
        }
    }

    #[cfg(not(windows))]
    pub fn new(_title: &str, w: usize, h: usize) -> Self {
        Self {
            buf: Sprite::new(w, h),
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

        #[cfg(not(feature = "colored"))]
        {
            for y in 0..h {
                for x in 0..w {
                    let pos = Position::new(x as i64, y as i64);
                    let ch = self.buf.get(&pos);
                    let _ = write!(stdout, "{}", ch);
                }
                let _ = writeln!(stdout);
            }
        }

        #[cfg(feature = "colored")]
        {
            for y in 0..h {
                for x in 0..w {
                    let pos = Position::new(x as i64, y as i64);
                    let ch = self.buf.get(&pos);
                    let color = self.buf.get_color(&pos);
                    //println!("{}", color.as_ascii());
                    let _ = write!(stdout, "\x1b[0;{}m{}\x1b[0m", color.as_ascii(), ch);
                }
                let _ = writeln!(stdout, "\x1b[0m");
            }
        }

        let _ = stdout.flush();
    }
}
