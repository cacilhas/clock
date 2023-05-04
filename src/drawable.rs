use raylib::prelude::RaylibDraw;

pub trait Drawable {
    fn draw(&self, draw: &mut impl RaylibDraw);
}
