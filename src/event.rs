#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum KeyEvent {
    Char(char),
    Space,
    Alt,
    Ctrl,
    Win,
    Tab,
    Caps,
    Shift,
    Esc,
    F(u8),
    Backspace,
    Home,
    End,
    Insert,
    PageUp,
    PageDown,
    Delete,
    NumLock,
    NumPad(u8),
    Enter,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Event {
    pub key: KeyEvent,
    pub modefier: KeyEvent,
}
