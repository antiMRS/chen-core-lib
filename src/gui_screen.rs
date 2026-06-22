use crate::{
    builtins::{Color, Position, Size, Sprite},
    event::{Event, KeyEvent},
};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use minifb::{Key, Window, WindowOptions};

pub struct GuiTerminal {
    window: Window,
    sprite: Sprite,
    char_width: usize,
    char_height: usize,
    scale: usize,
    pixel_buffer: Vec<u32>,
    need_redraw: bool,
}

impl GuiTerminal {
    pub fn new(title: &str, cols: usize, rows: usize, char_size: usize, scale: usize) -> Self {
        let char_w = char_size;
        let char_h = char_size;
        let win_w = cols * char_w * scale;
        let win_h = rows * char_h * scale;

        let mut window = Window::new(
            title,
            win_w,
            win_h,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .expect("Error while creating window");

        window.set_target_fps(60);

        let sprite = Sprite::new(cols, rows);

        let pixel_buffer = vec![0u32; win_w * win_h];

        GuiTerminal {
            window,
            sprite,
            char_width: char_w,
            char_height: char_h,
            scale,
            pixel_buffer,
            need_redraw: true,
        }
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn sprite_mut(&mut self) -> &mut Sprite {
        self.need_redraw = true;
        &mut self.sprite
    }

    pub fn blit(&mut self, sprite: &Sprite, pos: &Position) {
        self.sprite.draw_sprite(sprite, pos);
        self.need_redraw = true;
    }

    pub fn clear(&mut self) {
        self.sprite.fill(' ');
        #[cfg(feature = "colored")]
        {
            self.sprite.fill_color(Color::default());
        }
        self.need_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let w = self.sprite.size().w() as usize;
        let h = self.sprite.size().h() as usize;
        let cw = self.char_width;
        let ch = self.char_height;
        let scale = self.scale;
        let win_w = self.window.get_size().0;
        let win_h = self.window.get_size().1;

        let font = BASIC_FONTS;

        self.pixel_buffer.fill(0xFF000000);

        for y in 0..h {
            for x in 0..w {
                let pos = Position::new(x as i64, y as i64);
                let chr = self.sprite.get_char(&pos);

                // Цвета
                #[cfg(feature = "colored")]
                let fg = self.sprite.get_color(&pos);
                #[cfg(not(feature = "colored"))]
                let fg = Color::new(255, 255, 255);

                let bg = Color::new(0, 0, 0);

                let glyph = font.get(chr).unwrap_or_else(|| font.get('?').unwrap());

                for row in 0..8 {
                    for col in 0..8 {
                        let pixel_on = (glyph[row] >> (7 - col)) & 1 == 1;
                        let color = if pixel_on { fg } else { bg };
                        let argb = color_to_u32(color);

                        let px = (x * cw + col) * scale;
                        let py = (y * ch + row) * scale;

                        for dy in 0..scale {
                            for dx in 0..scale {
                                let final_x = px + dx;
                                let final_y = py + dy;
                                if final_x < win_w && final_y < win_h {
                                    let idx = final_y * win_w + final_x;
                                    self.pixel_buffer[idx] = argb;
                                }
                            }
                        }
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
    0xFFFFFFFF // белый
}
