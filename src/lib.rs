mod builtins;
mod render;
mod screen;

pub use builtins::*;
pub use render::*;
pub use screen::*;

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn main_test() {
        let mut screen = Screen::new(100, 100);
        screen.show();
    }
}
