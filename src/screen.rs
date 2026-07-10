// terminal.rs
use crate::builtins::{Color, EMPTY_CHAR, Position, Sprite};
use crate::event::Event;

use std::io::{self, Write};
use std::process::Command;

#[cfg(feature = "styled")]
use crate::render::CharStyle;

///
/// Terminal screen
///
pub struct Terminal {
    buf: Sprite,
}

impl Terminal {
    ///
    /// Creates new terminal of size w * h with title title
    ///
    /// # Example
    /// ```
    /// # use rube_core_lib::system::Terminal;
    /// let term = Terminal::new("Test", 10, 10);
    /// ```
    ///
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

    ///
    /// Draws sprite on a screen.
    /// Skips cells with EMPTY_CHAR.
    ///
    pub fn blit(&mut self, sprite: &Sprite, pos: &Position) {
        self.buf.blit_sprite(sprite, pos);
    }

    ///
    /// Fills screen with EMPTY_CHAR and Color::black
    ///
    pub fn clear(&mut self) {
        self.buf.fill(EMPTY_CHAR);
        #[cfg(feature = "colored")]
        self.buf.fill_color(Color::new(0, 0, 0));
        #[cfg(feature = "styled")]
        self.buf.fill_style(CharStyle::Normal);
        #[cfg(feature = "background")]
        self.buf.fill_bg(Color::new(0, 0, 0));
    }

    ///
    /// Draws buffer in console
    ///
    pub fn render(&self) {
        #[cfg(windows)]
        {
            let _ = Command::new("cmd").args(["/c", "cls"]).status();
        }
        #[cfg(unix)]
        {
            let _ = Command::new("clear").status();
        }

        let w = self.buf.size().w() as usize;
        let h = self.buf.size().h() as usize;
        let mut stdout = io::stdout();

        {
            for y in 0..h {
                for x in 0..w {
                    let ch = self.buf.get_char(x, h - y - 1);
                    #[cfg(feature = "colored")]
                    let color = self.buf.get_color(x, h - y - 1);
                    #[cfg(all(feature = "background", feature = "colored"))]
                    let bg = self.buf.get_bg_color(x, h - y - 1);
                    #[cfg(feature = "styled")]
                    let style = self.buf.get_style(x, h - y - 1);

                    #[cfg(any(feature = "styled", feature = "colored", feature = "background"))]
                    let _ = write!(stdout, "\x1b[0m");

                    // writing style
                    #[cfg(feature = "styled")]
                    let _ = write!(stdout, "\x1b[{}m", style.as_ascii());

                    // writing background
                    #[cfg(all(feature = "background", feature = "colored"))]
                    {
                        #[cfg(feature = "terminal_color_rgb")]
                        {
                            let _ = write!(stdout, "\x1b[48;2;{};{};{}m", bg.r(), bg.g(), bg.b());
                        }

                        #[cfg(feature = "terminal_color_cubes")]
                        {
                            let _ = write!(stdout, "\x1b[48;5;{}m", bg.as_ascii());
                        }

                        #[cfg(feature = "terminal_color_legacy")]
                        {
                            let _ = write!(stdout, "\x1b[{}m", bg.as_legacy());
                        }
                    }

                    #[cfg(feature = "colored")]
                    {
                        #[cfg(feature = "terminal_color_rgb")]
                        {
                            let _ = write!(
                                stdout,
                                "\x1b[38;2;{};{};{}m",
                                color.r(),
                                color.g(),
                                color.b()
                            );
                        }

                        #[cfg(feature = "terminal_color_cubes")]
                        {
                            let _ = write!(stdout, "\x1b[38;5;{}m", color.as_ascii());
                        }

                        #[cfg(feature = "terminal_color_legacy")]
                        {
                            let _ = write!(stdout, "\x1b[{}m", color.as_legacy());
                        }
                    }
                    let _ = write!(stdout, "{}", ch);
                }
                let _ = write!(stdout, "\n");
            }
        }

        let _ = stdout.flush();
    }

    pub fn is_open(&self) -> bool {
        true
    }

    pub fn poll_events(&self) -> Option<Event> {
        None
    }
}
