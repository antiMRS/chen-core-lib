mod event;
mod position;
mod render;
mod screen;

pub mod builtins {
    pub use crate::position::*;
    pub use crate::render::*;
}

pub mod system {
    pub use crate::event::*;
    pub use crate::screen::Terminal;
}

#[cfg(test)]
mod test {
    use builtins::*;

    macro_rules! pos {
        ($x:expr, $y:expr) => {
            Position::new($x, $y)
        };
    }

    use crate::*;
    #[test]
    fn main_test() {
        let mut screen = screen::Terminal::new("Test terminal", 100, 100);
        screen.render();
    }

    #[test]
    fn simple_geometry_test() {
        let sq_geom = Geometry::new_square(5);
        let sq_geom_pos = Position::new(5, 7);

        // Simple geometry tests
        assert!(sq_geom.are_in(&sq_geom_pos, &pos!(10, 10)));
        assert!(!sq_geom.are_in(&sq_geom_pos, &pos!(-15, 0)));

        // Collision
        assert!(sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(4, 4))); // collides
        assert!(!sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(0, 0))); // not collides
        assert!(sq_geom.collide(&sq_geom_pos, &Geometry::new_square(1), &pos!(7, 9))); // in
        // Touches
        assert!(!sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(5, 3))); // down
        assert!(!sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(5, 12))); // up
        assert!(!sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(1, 7))); // left
        assert!(!sq_geom.collide(&sq_geom_pos, &Geometry::new_square(4), &pos!(10, 7))); // right

        assert_eq!(sq_geom.square(), 5 * 5);
        assert_eq!(sq_geom.len(), 4);
        assert_eq!(sq_geom.size(), Size::new(5, 5));
    }

    #[test]
    fn geometry_test() {
        let geom = Geometry::new(vec![
            pos!(0, 0),
            pos!(0, 5),
            pos!(3, 7),
            pos!(5, 5),
            pos!(5, 0),
        ]); // house
        let geom_pos = pos!(5, 7);

        assert_eq!(geom.size(), Size::new(5, 7));
        assert_eq!(geom.square(), 30);
        assert!(geom.are_in(&geom_pos, &pos!(7, 8)));
        assert!(!geom.are_in(&geom_pos, &pos!(0, 0)));
        assert!(geom.collide(&geom_pos, &Geometry::new_square(3), &pos!(4, 6)));
        assert!(geom.collide(&geom_pos, &geom, &pos!(3, 6)));
        assert!(geom.collide(&geom_pos, &Geometry::new_square(1), &pos!(8, 9)));
    }

    #[test]
    fn color() {
        let color = Color::new(255, 255, 255);
        assert_eq!(color.as_ascii(), 37);
        assert_ne!(color.as_ascii(), 0);
    }
}
