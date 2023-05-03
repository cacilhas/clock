extern crate raylib;

use std::{
    f32::consts::TAU,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Local};
use raylib::prelude::*;

fn main() {
    let (w, h) = (200.0, 200.0);
    let (cx, cy) = (w / 2.0, h / 2.0);
    let centre = Vector2::new(cx, cy);
    let (mut rl, thr) = raylib::init()
        .size(w as i32, h as i32)
        .title("Clock")
        .build();
    rl.set_target_fps(15);
    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    let foreground = Color::BLACK;
    let secscolour = Color::RED;
    let background = Color::new(0x00, 0x00, 0x00, 0x00);
    let angle_offset = -TAU / 4.0;
    let hour_pointer = 50.0;
    let min_pointer = 90.0;
    let sec_pointer = 95.0;

    let tz: DateTime<Local> = SystemTime::now().into();
    let tz = tz.offset().local_minus_utc() as f32 / 3600.0;
    let rotation = -360.0 * tz / 12.0; // Raylib DrawTexturePro uses degrees instead of radians

    // Hour numbers
    let raw = include_bytes!("assets/hours.png").to_vec();
    let img = Image::load_image_from_mem(".png", &raw, raw.len() as i32).unwrap();
    let backhours = rl.load_texture_from_image(&thr, &img).unwrap();
    let backhours_rect = Rectangle {
        width: img.width() as f32,
        height: img.height() as f32,
        ..Default::default()
    };
    let backhours_center_rect = Rectangle {
        x: cx,
        y: cy,
        width: img.width() as f32,
        height: img.height() as f32,
    };

    // Minute numbers
    let raw = include_bytes!("assets/mins.png").to_vec();
    let img = Image::load_image_from_mem(".png", &raw, raw.len() as i32).unwrap();
    let backmins = rl.load_texture_from_image(&thr, &img).unwrap();

    // UTC
    let raw = include_bytes!("assets/utc.png").to_vec();
    let img = Image::load_image_from_mem(".png", &raw, raw.len() as i32).unwrap();
    let backutc = rl.load_texture_from_image(&thr, &img).unwrap();

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
        let secs_angle = secs * TAU / 60.0 + angle_offset;
        let mins_angle = mins * TAU / 60.0 + angle_offset;
        let hours_angle = hours * TAU / 12.0 + angle_offset;

        let mut draw = rl.begin_drawing(&thr);
        {
            let camera = Camera2D {
                zoom: 1.0,
                ..Default::default()
            };
            let mut draw = draw.begin_mode2D(&camera);

            draw.clear_background(background);
            draw.draw_texture_pro(
                &backhours,
                &backhours_rect,
                &backhours_center_rect,
                centre,
                rotation,
                Color::WHITE,
            );
            draw.draw_texture(&backmins, 0, 0, Color::WHITE);
            draw.draw_texture(&backutc, 0, 0, Color::WHITE);

            for r in 98..100 {
                draw.draw_circle_lines(cx as i32, cy as i32, r as f32, foreground);
            }

            let x = cx + hour_pointer * hours_angle.cos();
            let y = cy + hour_pointer * hours_angle.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 5.0, foreground);

            let x = cx + min_pointer * mins_angle.cos();
            let y = cy + min_pointer * mins_angle.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 2.0, foreground);

            let x = cx + sec_pointer * secs_angle.cos();
            let y = cy + sec_pointer * secs_angle.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 1.0, secscolour);
        }
    }
}
