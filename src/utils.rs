use crate::builtins::{Color, Position, Sprite};

pub fn char_test() -> Sprite {
    let mut sp = Sprite::new(16, 16);

    for ch in '\0'..(255_u8 as char) {
        let xy = ch as u8 as usize;
        let (x, y) = Position::from_flattened(sp.size(), xy);
        sp.set_char(ch, x, y);
    }
    #[cfg(feature = "colored")]
    sp.fill_color(Color::new(255, 255, 255));

    sp
}
