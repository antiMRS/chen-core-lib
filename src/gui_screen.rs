use crate::{
    builtins::{Color, EMPTY_CHAR, Position, Size, Sprite},
    event::{Event, KeyEvent},
};
//use font8x8::{BASIC_FONTS, UnicodeFonts};
use chen_core_fonts::{BasicFont, Font};
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;

pub struct GuiConfig {
    pub title: &'static str,
    pub font: Box<dyn Font>,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            title: "ChenCore Screen",
            font: Box::new(BasicFont::new()),
        }
    }
}

///
/// Screen in separated window
///
pub struct GuiTerminal {
    window: Window,
    sprite: Sprite,
    scale: usize,
    offset_x: usize,
    offset_y: usize,
    pixel_buffer: Vec<u32>,
    pixel_buffer_raw: PixelBuffer,
    need_redraw: bool,
    current_win_size: (usize, usize),
    pub config: GuiConfig,
}

impl GuiTerminal {
    pub fn new(cols: usize, rows: usize, config: GuiConfig) -> Self {
        let scale = 1;
        let win_w = cols * CHAR_WIDTH * scale;
        let win_h = rows * CHAR_HEIGHT * scale;

        let mut window = Window::new(
            config.title,
            win_w,
            win_h,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .expect("Error while creating window");

        window.set_target_fps(60);

        let sprite = Sprite::new(cols, rows);
        let pixel_buffer = vec![0u32; win_w * win_h];
        let raw_w = cols * CHAR_WIDTH;
        let raw_h = rows * CHAR_HEIGHT;
        let mut pixel_buffer_raw = PixelBuffer::new(raw_w, raw_h);
        pixel_buffer_raw.fill(0xFF000000);

        GuiTerminal {
            window,
            sprite,
            scale,
            offset_x: 0,
            offset_y: 0,
            pixel_buffer,
            pixel_buffer_raw,
            need_redraw: true,
            current_win_size: (win_w, win_h),
            config,
        }
    }

    pub fn scale(&self) -> usize {
        self.scale
    }

    pub fn new_scale(&mut self, new: usize) {
        self.scale = new;
        self.current_win_size = (0, 0);
        self.need_redraw = true;
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn sprite_mut(&mut self) -> &mut Sprite {
        self.need_redraw = true;
        &mut self.sprite
    }

    pub fn blit(&mut self, sprite: &Sprite, pos: &Position) {
        self.sprite.blit_sprite(sprite, pos);

        let dst_x = (pos.x() * CHAR_WIDTH as i64) as usize;
        let dst_y = (pos.y() * CHAR_HEIGHT as i64) as usize;

        let sp_w = sprite.size().w() as usize;
        let sp_h = sprite.size().h() as usize;

        for cy in 0..sp_h {
            for cx in 0..sp_w {
                let ch = sprite.get_char(cx, cy);
                if ch == EMPTY_CHAR {
                    continue;
                }
                #[cfg(feature = "colored")]
                let fg = sprite.get_color(cx, cy);
                #[cfg(not(feature = "colored"))]
                let fg = Color::new(255, 255, 255);
                #[cfg(feature = "background")]
                let bg = sprite.get_bg_color(cx, cy);
                #[cfg(not(feature = "background"))]
                let bg = Color::new(0, 0, 0);

                let glyph = self
                    .config
                    .font
                    .get(ch)
                    .unwrap_or(self.config.font.get('?').unwrap());

                let pixel_x = dst_x + cx * CHAR_WIDTH;
                let pixel_y = self.pixel_buffer_raw.height() - (dst_y + cy * CHAR_HEIGHT) - 1;

                Self::render_glyph_to_buffer(
                    glyph,
                    fg,
                    bg,
                    &mut self.pixel_buffer_raw,
                    pixel_x,
                    pixel_y,
                );
            }
        }

        self.need_redraw = true;
    }

    pub fn blit_buffer(&mut self, buffer: &PixelBuffer, x: usize, y: usize) {
        self.pixel_buffer_raw.blit(buffer, x, y);
        self.need_redraw = true;
    }

    pub fn clear(&mut self) {
        self.sprite.fill(EMPTY_CHAR);
        #[cfg(feature = "colored")]
        self.sprite.fill_color(Color::default());
        self.pixel_buffer_raw.fill(0xFF000000);
        self.need_redraw = true;
    }

    fn update_on_resize(&mut self) {
        let (win_w, win_h) = self.window.get_size();

        self.current_win_size = (win_w, win_h);

        let cols = self.sprite.size().w() as usize;
        let rows = self.sprite.size().h() as usize;

        let max_scale_x = win_w / (cols * CHAR_WIDTH);
        let max_scale_y = win_h / (rows * CHAR_HEIGHT);
        let new_scale = std::cmp::max(1, std::cmp::min(max_scale_x, max_scale_y));
        self.scale = new_scale;

        let total_w = cols * CHAR_WIDTH * self.scale;
        let total_h = rows * CHAR_HEIGHT * self.scale;
        self.offset_x = (win_w.saturating_sub(total_w)) / 2;
        self.offset_y = (win_h.saturating_sub(total_h)) / 2;

        self.pixel_buffer = vec![0u32; win_w * win_h];
        self.need_redraw = true;
    }

    pub fn render(&mut self) {
        let (win_w, win_h) = self.window.get_size();

        if win_w != self.current_win_size.0 || win_h != self.current_win_size.1 {
            self.update_on_resize();
        }

        if !self.need_redraw {
            return;
        }

        let raw_w = self.pixel_buffer_raw.width();
        let raw_h = self.pixel_buffer_raw.height();
        let scale = self.scale;
        let off_x = self.offset_x;
        let off_y = self.offset_y;

        if self.pixel_buffer.len() != win_w * win_h {
            self.pixel_buffer = vec![0u32; win_w * win_h];
        }

        self.pixel_buffer.fill(0xFF000000);

        for y in 0..raw_h {
            let dest_y_base = off_y + y * scale;
            if dest_y_base >= win_h {
                break;
            }
            for x in 0..raw_w {
                let color = self.pixel_buffer_raw.buf[y * raw_w + x];
                let dest_x_base = off_x + x * scale;
                if dest_x_base >= win_w {
                    break;
                }

                for dy in 0..scale {
                    let dest_y = dest_y_base + dy;
                    if dest_y >= win_h {
                        break;
                    }
                    let row_start = dest_y * win_w;
                    for dx in 0..scale {
                        let dest_x = dest_x_base + dx;
                        if dest_x >= win_w {
                            break;
                        }
                        self.pixel_buffer[row_start + dest_x] = color;
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&self.pixel_buffer, win_w, win_h)
            .expect("Error while updating screen");

        self.need_redraw = false;
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn poll_events(&mut self) -> Option<Event> {
        self.window.update();
        if !self.window.is_open() {
            return None;
        }

        if let Some(key) = self.window.get_keys().first() {
            let key_event = match key {
                Key::Space => KeyEvent::Space,
                Key::Tab => KeyEvent::Tab,
                Key::CapsLock => KeyEvent::Caps,
                Key::Escape => KeyEvent::Esc,
                Key::Backspace => KeyEvent::Backspace,
                Key::Home => KeyEvent::Home,
                Key::End => KeyEvent::End,
                Key::Insert => KeyEvent::Insert,
                Key::PageUp => KeyEvent::PageUp,
                Key::PageDown => KeyEvent::PageDown,
                Key::Delete => KeyEvent::Delete,
                Key::NumLock => KeyEvent::NumLock,
                Key::Enter => KeyEvent::Enter,
                Key::Up => KeyEvent::ArrowUp,
                Key::Right => KeyEvent::ArrowRight,
                Key::Down => KeyEvent::ArrowDown,
                Key::Left => KeyEvent::ArrowLeft,
                Key::LeftShift | Key::RightShift => KeyEvent::Shift,
                Key::LeftCtrl | Key::RightCtrl => KeyEvent::Ctrl,
                Key::LeftAlt | Key::RightAlt => KeyEvent::Alt,
                Key::LeftSuper | Key::RightSuper => KeyEvent::Win,
                Key::F1 => KeyEvent::F(1),
                Key::F2 => KeyEvent::F(2),
                Key::F3 => KeyEvent::F(3),
                Key::F4 => KeyEvent::F(4),
                Key::F5 => KeyEvent::F(5),
                Key::F6 => KeyEvent::F(6),
                Key::F7 => KeyEvent::F(7),
                Key::F8 => KeyEvent::F(8),
                Key::F9 => KeyEvent::F(9),
                Key::F10 => KeyEvent::F(10),
                Key::F11 => KeyEvent::F(11),
                Key::F12 => KeyEvent::F(12),
                Key::Key0 => KeyEvent::Char('0'),
                Key::Key1 => KeyEvent::Char('1'),
                Key::Key2 => KeyEvent::Char('2'),
                Key::Key3 => KeyEvent::Char('3'),
                Key::Key4 => KeyEvent::Char('4'),
                Key::Key5 => KeyEvent::Char('5'),
                Key::Key6 => KeyEvent::Char('6'),
                Key::Key7 => KeyEvent::Char('7'),
                Key::Key8 => KeyEvent::Char('8'),
                Key::Key9 => KeyEvent::Char('9'),
                Key::A => KeyEvent::Char('a'),
                Key::B => KeyEvent::Char('b'),
                Key::C => KeyEvent::Char('c'),
                Key::D => KeyEvent::Char('d'),
                Key::E => KeyEvent::Char('e'),
                Key::F => KeyEvent::Char('f'),
                Key::G => KeyEvent::Char('g'),
                Key::H => KeyEvent::Char('h'),
                Key::I => KeyEvent::Char('i'),
                Key::J => KeyEvent::Char('j'),
                Key::K => KeyEvent::Char('k'),
                Key::L => KeyEvent::Char('l'),
                Key::M => KeyEvent::Char('m'),
                Key::N => KeyEvent::Char('n'),
                Key::O => KeyEvent::Char('o'),
                Key::P => KeyEvent::Char('p'),
                Key::Q => KeyEvent::Char('q'),
                Key::R => KeyEvent::Char('r'),
                Key::S => KeyEvent::Char('s'),
                Key::T => KeyEvent::Char('t'),
                Key::U => KeyEvent::Char('u'),
                Key::V => KeyEvent::Char('v'),
                Key::W => KeyEvent::Char('w'),
                Key::X => KeyEvent::Char('x'),
                Key::Y => KeyEvent::Char('y'),
                Key::Z => KeyEvent::Char('z'),
                _ => KeyEvent::None,
            };

            let modifier = if self.window.is_key_down(Key::LeftShift)
                || self.window.is_key_down(Key::RightShift)
            {
                KeyEvent::Shift
            } else if self.window.is_key_down(Key::LeftCtrl)
                || self.window.is_key_down(Key::RightCtrl)
            {
                KeyEvent::Ctrl
            } else if self.window.is_key_down(Key::LeftAlt)
                || self.window.is_key_down(Key::RightAlt)
            {
                KeyEvent::Alt
            } else if self.window.is_key_down(Key::LeftSuper)
                || self.window.is_key_down(Key::RightSuper)
            {
                KeyEvent::Win
            } else {
                KeyEvent::None
            };

            return Some(Event {
                key: key_event,
                modefier: modifier,
            });
        }
        None
    }

    fn render_glyph_to_buffer(
        glyph: [u8; 8],
        fg: Color,
        bg: Color,
        dst: &mut PixelBuffer,
        x: usize,
        y: usize,
    ) {
        let dst_w = dst.width();
        let dst_h = dst.height();
        for (row, row_m) in glyph.iter().enumerate() {
            let pixel_y = y + row;
            if pixel_y >= dst_h {
                continue;
            }
            for col in 0..8 {
                let pixel_x = x + col;
                if pixel_x >= dst_w {
                    continue;
                }
                let pixel_on = (row_m >> col) & 1 == 1;
                let color = if pixel_on { fg } else { bg };
                let argb = color_to_u32(color);
                let idx = pixel_y * dst_w + pixel_x;
                dst.buf[idx] = argb;
            }
        }
    }
}

#[cfg(feature = "colored")]
fn color_to_u32(color: Color) -> u32 {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) | 0xFF000000
}

#[cfg(not(feature = "colored"))]
fn color_to_u32(_: Color) -> u32 {
    0xFFFFFFFF
}

#[derive(Clone, Debug)]
pub struct PixelBuffer {
    pub(crate) buf: Box<[u32]>,
    pub(crate) size: Size,
}

impl PixelBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: vec![0_u32; w * h].into_boxed_slice(),
            size: Size::new(w as u64, h as u64),
        }
    }

    pub fn fill(&mut self, color: u32) {
        for v in self.buf.iter_mut() {
            *v = color;
        }
    }

    pub fn blit(&mut self, src: &PixelBuffer, dst_x: usize, dst_y: usize) {
        let src_w = src.size.w() as usize;
        let src_h = src.size.h() as usize;
        let dst_w = self.size.w() as usize;
        let dst_h = self.size.h() as usize;

        for sy in 0..src_h {
            let dy = dst_y + sy;
            if dy >= dst_h {
                break;
            }
            let src_row_start = sy * src_w;
            let dst_row_start = dy * dst_w;
            for sx in 0..src_w {
                let dx = dst_x + sx;
                if dx >= dst_w {
                    break;
                }
                let src_idx = src_row_start + sx;
                let dst_idx = dst_row_start + dx;
                self.buf[dst_idx] = src.buf[src_idx];
            }
        }
    }

    pub fn width(&self) -> usize {
        self.size.w() as usize
    }

    pub fn height(&self) -> usize {
        self.size.h() as usize
    }
}

impl std::ops::Index<usize> for PixelBuffer {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl std::ops::IndexMut<usize> for PixelBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
