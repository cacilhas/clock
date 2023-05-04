extern crate kodumaro_clock;

use kodumaro_clock::prelude::*;
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
    let pointers = Pointers::default();
    let clock = Clock::default();

    // Hour numbers
    let (backhours, img) =
        load_texture(&mut rl, &thr, include_bytes!("../assets/hours.png")).unwrap();
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
    let (backmins, _) = load_texture(&mut rl, &thr, include_bytes!("../assets/mins.png")).unwrap();

    // UTC
    let (backutc, _) = load_texture(&mut rl, &thr, include_bytes!("../assets/utc.png")).unwrap();

    rl.set_window_title(&thr, "Kodumaro Clock");

    while !rl.window_should_close() {
        if rl.is_key_released(KeyboardKey::KEY_Q) {
            break;
        }

        let angles = clock.get_angles().unwrap();
        let mut draw = rl.begin_drawing(&thr);
        {
            let camera = Camera2D {
                zoom: 1.0,
                ..Default::default()
            };
            let mut draw = draw.begin_mode2D(&camera);

            draw.clear_background(pointers.background);
            draw.draw_texture_pro(
                &backhours,
                &backhours_rect,
                &backhours_center_rect,
                centre,
                clock.rotation,
                Color::WHITE,
            );
            draw.draw_texture(&backmins, 0, 0, Color::WHITE);
            draw.draw_texture(&backutc, 0, 0, Color::WHITE);

            for r in 98..100 {
                draw.draw_circle_lines(cx as i32, cy as i32, r as f32, pointers.foreground);
            }

            let x = cx + pointers.hour * angles.hour.cos();
            let y = cy + pointers.hour * angles.hour.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 5.0, pointers.foreground);

            let x = cx + pointers.min * angles.min.cos();
            let y = cy + pointers.min * angles.min.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 2.0, pointers.foreground);

            let x = cx + pointers.sec * angles.sec.cos();
            let y = cy + pointers.sec * angles.sec.sin();
            draw.draw_line_ex(centre, Vector2::new(x, y), 1.0, pointers.secscolour);
        }
    }
}

fn load_texture(
    rl: &mut RaylibHandle,
    thr: &RaylibThread,
    raw: &[u8],
) -> anyhow::Result<(Texture2D, Image)> {
    let image = Image::load_image_from_mem(".png", &raw.to_vec(), raw.len() as i32)
        .or_else(|err| Err(Error(err)))?;
    let texture = rl
        .load_texture_from_image(&thr, &image)
        .or_else(|err| Err(Error(err)))?;
    Ok((texture, image))
}
