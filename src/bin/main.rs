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
    let background = Background::new(&mut rl, &thr, centre, clock.rotation).unwrap();
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
            background.draw(&mut draw);

            for r in 98..100 {
                draw.draw_circle_lines(cx as i32, cy as i32, r as f32, pointers.foreground);
            }

            PointerDrawer::new(&pointers, &angles, centre).draw(&mut draw);
        }
    }
}
