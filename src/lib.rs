mod buffer;
mod event;
#[cfg(feature = "use_gui")]
mod gui_screen;
mod position;
mod render;
mod screen;

pub mod utils;

pub mod builtins {
    pub use crate::position::*;
    pub use crate::render::*;
}

pub mod system {
    pub use crate::event::*;
    #[cfg(feature = "use_gui")]
    pub use crate::gui_screen::GuiConfig;
    #[cfg(feature = "use_gui")]
    pub use crate::gui_screen::GuiTerminal;
    #[cfg(feature = "use_gui")]
    pub use crate::gui_screen::PixelBuffer;
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
        let screen = screen::Terminal::new("Test terminal", 100, 100);
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
    fn color() {}

    #[test]
    fn geometry_transform() {
        let cube1 = Geometry::new(vec![pos!(0, 0), pos!(0, 5), pos!(8, 5), pos!(8, 0)]); // rectangle 8 x 5
        let cube2 = Geometry::new_square(5); // rectangle 5 x 5

        // pivot
        assert_eq!(cube1.pivot(), pos!(4, 2));
        assert_eq!(cube2.pivot(), pos!(2, 2));

        //rotation
        {
            let mut cube1 = cube1.clone();
            let mut cube2 = cube2.clone();
            cube1.rotate(90);
            cube2.rotate(90);
        }

        // intersection and addition
        {
            let cube1_pos = pos!(0, 0);
            let cube2_pos = pos!(4, 2);

            println!(
                "{:?}",
                cube1
                    .clone()
                    .intersection(&cube1_pos, cube2.clone(), &cube2_pos)
            );
        }
    }

    #[cfg(feature = "use_gui")]
    #[test]
    fn intersection_test() {
        let cube1_geo = Geometry::new(vec![pos!(0, 0), pos!(0, 5), pos!(8, 5), pos!(8, 0)]); // rectangle 8 x 5
        let cube2_geo = Geometry::new_square(5); // rectangle 5 x 5
        let cube1_p = Position::new(6, 5);
        let cube2_p = Position::new(2, 3);

        let mut cubes_sp = Sprite::new(20, 20);

        cubes_sp.geometry_draw(&cube1_geo, &cube1_p, 'H');
        cubes_sp.geometry_paint(&cube1_geo, &cube1_p, Color::new(255, 255, 255));

        cubes_sp.geometry_draw(&cube2_geo, &cube2_p, 'G');
        cubes_sp.geometry_paint(&cube2_geo, &cube2_p, Color::new(255, 255, 255));

        let cube_int = cube1_geo
            .clone()
            .intersection(&cube1_p, cube2_geo, &cube2_p)
            .unwrap_or(Geometry::new_square(0));
        let mut cube_int_sp = Sprite::new(20, 20);
        cube_int_sp.geometry_paint(&cube_int, &pos!(0, 0), Color::new(0, 0, 255));
        cube_int_sp.geometry_draw(&cube_int, &pos!(0, 0), 'X');

        let mut oses = Sprite::new(100, 100);
        oses.fill_color(Color::new(200, 200, 200));
        oses.fill_char_with_f(|x, y| if x == 50 || y == 50 { '=' } else { EMPTY_CHAR });

        let mut screen = system::GuiTerminal::new(100, 100, system::GuiConfig::default());

        screen.blit(&oses, &pos!(0, 0));
        screen.blit(&cubes_sp, &pos!(50, 50));
        screen.blit(&cube_int_sp, &pos!(50, 50));

        screen.render();

        loop {
            let _ = screen.poll_events();
            screen.render();
            std::thread::sleep(std::time::Duration::from_millis(10));
            if !screen.is_open() {
                break;
            }
        }
    }

    #[cfg(feature = "use_gui")]
    #[test]
    fn pixel_buffer() {
        let sp = crate::utils::char_test();
        let buf = sp.buffer(&font8x8::BASIC_FONTS);
        println!("{:?}", buf)
    }
}
