extern crate raylib;

use std::{f32::consts::TAU, io, time::SystemTime};

use chrono::{DateTime, Local, NaiveTime, Utc};
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
    let hour_pointer = 50.0;
    let min_pointer = 90.0;
    let sec_pointer = 95.0;

    let tz: DateTime<Local> = SystemTime::now().into();
    let tz = tz.offset().local_minus_utc() as f32 / 3600.0;
    let rotation = -360.0 * tz / 12.0; // Raylib DrawTexturePro uses degrees instead of radians

    // Hour numbers
    let (backhours, img) = load_texture(&mut rl, &thr, include_bytes!("assets/hours.png")).unwrap();
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
    let (backmins, _) = load_texture(&mut rl, &thr, include_bytes!("assets/mins.png")).unwrap();

    // UTC
    let (backutc, _) = load_texture(&mut rl, &thr, include_bytes!("assets/utc.png")).unwrap();

    rl.set_window_title(&thr, "Kodumaro Clock");

    while !rl.window_should_close() {
        let (hours, mins, secs) = get_time().unwrap();
        let secs_angle = get_angle(secs, 60.0);
        let mins_angle = get_angle(mins, 60.0);
        let hours_angle = get_angle(hours, 12.0);

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

fn load_texture(
    rl: &mut RaylibHandle,
    thr: &RaylibThread,
    raw: &[u8],
) -> Result<(Texture2D, Image), String> {
    let img = Image::load_image_from_mem(".png", &raw.to_vec(), raw.len() as i32).unwrap();
    Ok((rl.load_texture_from_image(&thr, &img)?, img))
}

fn get_time() -> io::Result<(f32, f32, f32)> {
    let midnight =
        NaiveTime::from_hms_opt(0, 0, 0).ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
    let time = Utc::now().time() - midnight;
    let time = (time.num_milliseconds() as f32) / 1_000.0;
    let secs = time % 60.0;
    let mins = (time / 60.0) % 60.0;
    let hours = (time / 3600.0) % 12.0;
    Ok((hours, mins, secs))
}

fn get_angle(value: f32, max_value: f32) -> f32 {
    let offset = -TAU / 4.0;
    value * TAU / max_value + offset
}
