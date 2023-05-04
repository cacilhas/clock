use raylib::prelude::Color;

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
