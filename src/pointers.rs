use raylib::prelude::*;

use crate::{clock::ClockValues, drawable::Drawable};

#[derive(Debug)]
pub struct Pointers {
    pub background: Color,
    pub foreground: Color,
    pub secscolour: Color,
    pub hour: f32,
    pub min: f32,
    pub sec: f32,
}

impl Default for Pointers {
    fn default() -> Self {
        Self {
            background: Color::new(0x00, 0x00, 0x00, 0x00),
            foreground: Color::BLACK,
            secscolour: Color::RED,
            hour: 50.0,
            min: 90.0,
            sec: 95.0,
        }
    }
}

pub struct PointerDrawer<'a> {
    angles: &'a ClockValues,
    centre: Vector2,
    pointers: &'a Pointers,
}

impl<'a> PointerDrawer<'a> {
    pub fn new(
        pointers: &'a Pointers,
        angles: &'a ClockValues,
        centre: Vector2,
    ) -> PointerDrawer<'a> {
        PointerDrawer {
            angles,
            centre,
            pointers,
        }
    }

    fn draw_hour(&self, draw: &mut impl RaylibDraw) {
        let x = self.centre.x + self.pointers.hour * self.angles.hour.cos();
        let y = self.centre.y + self.pointers.hour * self.angles.hour.sin();
        draw.draw_line_ex(
            self.centre,
            Vector2::new(x, y),
            5.0,
            self.pointers.foreground,
        );
    }

    fn draw_min(&self, draw: &mut impl RaylibDraw) {
        let x = self.centre.x + self.pointers.min * self.angles.min.cos();
        let y = self.centre.y + self.pointers.min * self.angles.min.sin();
        draw.draw_line_ex(
            self.centre,
            Vector2::new(x, y),
            2.0,
            self.pointers.foreground,
        );
    }

    fn draw_sec(&self, draw: &mut impl RaylibDraw) {
        let x = self.centre.x + self.pointers.sec * self.angles.sec.cos();
        let y = self.centre.y + self.pointers.sec * self.angles.sec.sin();
        draw.draw_line_ex(
            self.centre,
            Vector2::new(x, y),
            1.0,
            self.pointers.secscolour,
        );
    }
}

impl<'a> Drawable for PointerDrawer<'a> {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        self.draw_hour(draw);
        self.draw_min(draw);
        self.draw_sec(draw);
    }
}
