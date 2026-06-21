// terminal.rs
use crate::builtins::{Position, Size};
use crate::render::{EMPTY_CHAR, Sprite};

use std::io::{self, Write};
use std::os::windows::thread;
use std::process::Command;
use std::time::Duration;

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

    #[cfg(unix)]
    pub fn new(title: &str, size: Size) -> Self {
        // На Linux заголовок окна не устанавливаем (можно было бы через xterm, но это сложно)
        let _ = title; // игнорируем

        // Попытка получить окно терминала через переменную окружения WINDOWID
        use std::env;
        let window_id = env::var("WINDOWID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        // Открываем соединение с X-сервером
        use x11::xlib;
        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());
            let window = window_id as xlib::Window;
            let poller = if display.is_null() {
                // Если X11 недоступен, создаём «пустой» опросчик (заглушку).
                // Для этого передаём нулевые указатели, но методы будут возвращать значения по умолчанию.
                EventPoller::new(display)
            } else {
                EventPoller::new(display)
            };
            Terminal {
                buf: Sprite::new(size.w() as usize, size.h() as usize),
                poller,
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
