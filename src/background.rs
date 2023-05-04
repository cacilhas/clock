use crate::drawable::Drawable;
use crate::error::Error;
use raylib::prelude::*;

#[derive(Debug)]
pub struct Background {
    pub rotation: f32,
    pub hour_texture: Texture2D,
    pub hour_rect: Rectangle,
    pub min_texture: Texture2D,
    pub utc_texture: Texture2D,
    pub centre: Vector2,
    pub centre_rect: Rectangle,
}

impl Background {
    pub fn new(
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
        centre: Vector2,
        rotation: f32,
    ) -> anyhow::Result<Background> {
        let (hour_texture, img) =
            Self::load_texture(handle, thr, include_bytes!("assets/hours.png"))?;
        let hour_rect = Rectangle {
            width: img.width() as f32,
            height: img.height() as f32,
            ..Default::default()
        };
        let centre_rect = Rectangle {
            x: centre.x,
            y: centre.y,
            ..hour_rect
        };
        let (min_texture, _) = Self::load_texture(handle, thr, include_bytes!("assets/mins.png"))?;
        let (utc_texture, _) = Self::load_texture(handle, thr, include_bytes!("assets/utc.png"))?;
        Ok(Self {
            hour_texture,
            hour_rect,
            min_texture,
            utc_texture,
            centre,
            centre_rect,
            rotation,
        })
    }

    fn load_texture(
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
        raw: &[u8],
    ) -> anyhow::Result<(Texture2D, Image)> {
        let image = Image::load_image_from_mem(".png", &raw.to_vec(), raw.len() as i32)
            .or_else(|err| Err(Error(err)))?;
        let texture = handle
            .load_texture_from_image(&thr, &image)
            .or_else(|err| Err(Error(err)))?;
        Ok((texture, image))
    }
}

impl Drawable for Background {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        draw.draw_texture_pro(
            &self.hour_texture,
            &self.hour_rect,
            &self.centre_rect,
            self.centre,
            self.rotation,
            Color::WHITE,
        );
        draw.draw_texture(&self.min_texture, 0, 0, Color::WHITE);
        draw.draw_texture(&self.utc_texture, 0, 0, Color::WHITE);
    }
}
