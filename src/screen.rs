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

        #[cfg(not(feature = "colored"))]
        {
            for y in 0..h {
                for x in 0..w {
                    #[cfg(feature = "styled")]
                    let style = self.buf.get_style(x, h - y - 1);
                    let ch = self.buf.get_char(x, h - y - 1);
                    #[cfg(feature = "styled")]
                    let _ = write!(stdout, "\x1b[{};1m{}", style.as_ascii(), ch);
                    #[cfg(not(feature = "styled"))]
                    let _ = write!(stdout, "{}", ch);
                }
                let _ = writeln!(stdout);
            }
        }

        #[cfg(feature = "colored")]
        {
            for y in 0..h {
                for x in 0..w {
                    let ch = self.buf.get_char(x, h - y - 1);
                    let color = self.buf.get_color(x, h - y - 1);
                    #[cfg(feature = "styled")]
                    let style = self.buf.get_style(x, h - y - 1);

                    #[cfg(feature = "terminal_color_rgb")]
                    {
                        #[cfg(feature = "styled")]
                        let _ = write!(
                            stdout,
                            "\x1b[{};38;2;{};{};{}m{}\x1b[0m",
                            style.as_ascii(),
                            color.r(),
                            color.g(),
                            color.b(),
                            ch
                        );
                        #[cfg(not(feature = "styled"))]
                        let _ = write!(
                            stdout,
                            "\x1b[38;2;{};{};{}m{}\x1b[0m",
                            color.r(),
                            color.g(),
                            color.b(),
                            ch
                        );
                    }

                    #[cfg(feature = "terminal_color_cubes")]
                    {
                        #[cfg(feature = "styled")]
                        let _ = write!(
                            stdout,
                            "\x1b[{};38;5;{}m{}\x1b[0m",
                            style.as_ascii(),
                            color.as_ascii(),
                            ch
                        );
                        #[cfg(not(feature = "styled"))]
                        let _ = write!(stdout, "\x1b[38;5;{}m{}\x1b[0m", color.as_ascii(), ch);
                    }
                    #[cfg(feature = "terminal_color_legacy")]
                    {
                        #[cfg(feature = "styled")]
                        let _ = write!(
                            stdout,
                            "\x1b[{};{}m{}\x1b[0m",
                            style.as_ascii(),
                            color.as_legacy(),
                            ch
                        );
                        #[cfg(not(feature = "styled"))]
                        let _ = write!(stdout, "\x1b[{}m{}\x1b[0m", color.as_legacy(), ch);
                    }
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
