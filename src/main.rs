extern crate image;
extern crate raylib;

use std::{
    f32::consts::TAU,
    ffi::c_void,
    mem,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use image::GenericImageView;
use raylib::prelude::*;


fn main() {
    let (w, h) = (200.0, 200.0);
    let (cx, cy) = (w/2.0, h/2.0);
    let centre = Vector2::new(cx, cy);
    let (mut rl, thr) = raylib::init()
        .size(w as i32, h as i32)
        .title("Clock")
        .build();
    rl.set_target_fps(15);
    let foreground = Color::new(0x00, 0x00, 0x00, 0xff);
    let secscolor  = Color::new(0xff, 0x00, 0x00, 0xff);
    let background = Color::new(0xff, 0xff, 0xff, 0x00);
    let angle_offset = -TAU / 4.0;
    let hour_pointer = 50.0;
    let min_pointer  = 90.0;
    let sec_pointer  = 95.0;

    // Hour numbers
    let raw = include_bytes!("assets/hours.png");
    let img = image::load_from_memory(raw).unwrap();
    let (iw, ih) = img.dimensions();
    let raw = img.to_rgba8().into_raw();
    let mut img: Image = unsafe { mem::zeroed() };
    img.width = iw as i32;
    img.height = ih as i32;
    img.mipmaps = 1;
    img.format = PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32;
    img.data = raw.as_ptr() as *mut c_void;
    mem::forget(raw);

    let backhours = rl.load_texture_from_image(&thr, &img).unwrap();

    while !rl.window_should_close() {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 86_400_000;
        let time = (time as f32) / 1_000.0;
        let secs = time % 60.0;
        let mins = (time / 60.0) % 60.0;
        let hours = (time / 3600.0) % 12.0;
        let secs_angle  = secs  * TAU / 60.0 + angle_offset;
        let mins_angle  = mins  * TAU / 60.0 + angle_offset;
        let hours_angle = hours * TAU / 12.0 + angle_offset;
        let mut draw = rl.begin_drawing(&thr);

        draw.clear_background(background);
        draw.draw_texture(&backhours, 0, 0, foreground);

        for r in 98..100 {
            draw.draw_circle_lines(cx as i32, cy as i32, r as f32, foreground);

            let x = cx + hour_pointer * hours_angle.cos();
            let y = cy + hour_pointer * hours_angle.sin();
            let end = Vector2::new(x, y);
            draw.draw_line_ex(centre, end, 5.0, foreground);

            let x = cx + min_pointer * mins_angle.cos();
            let y = cy + min_pointer * mins_angle.sin();
            let end = Vector2::new(x, y);
            draw.draw_line_ex(centre, end, 2.0, foreground);

            let x = cx + sec_pointer * secs_angle.cos();
            let y = cy + sec_pointer * secs_angle.sin();
            let end = Vector2::new(x, y);
            draw.draw_line_ex(centre, end, 1.0, secscolor);
        }
    }
}


