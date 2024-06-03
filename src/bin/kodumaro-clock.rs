extern crate kodumaro_clock;

use std::{error, mem, ptr};

use kodumaro_clock::prelude::*;
use raylib::prelude::*;
use x11::xlib::{self, Atom, Display, Window, XWindowAttributes};

fn main() -> Result<(), Box<dyn error::Error>> {
    let (w, h) = (200.0, 200.0);
    let (cx, cy) = (w / 2.0, h / 2.0);
    let centre = Vector2::new(cx, cy);
    let (mut handle, thr) = raylib::init()
        .size(w as i32, h as i32)
        .title("kodumaro-clock") // WM_CLASS
        .transparent()
        .undecorated()
        .build();
    handle.set_target_fps(15);
    handle.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    let pointers = Pointers::default();
    let clock = Clock::default();
    let background = Background::new(&mut handle, &thr, centre, clock.rotation)?;
    handle.set_window_title(&thr, "Kodumaro Clock");

    unsafe {
        let display = x11::xlib::XOpenDisplay(ptr::null());
        if !display.is_null() {
            let window = handle.get_window_handle();
            set_wm_window_type(display, window as _);
        }
    }

    while !handle.window_should_close() {
        if handle.is_key_released(KeyboardKey::KEY_Q) {
            break;
        }

        let angles = clock.get_angles()?;
        let mut draw = handle.begin_drawing(&thr);
        {
            let camera = Camera2D {
                zoom: 1.0,
                ..Default::default()
            };
            let mut draw = draw.begin_mode2D(camera);

            draw.clear_background(pointers.background);
            background.draw(&mut draw);

            for r in 98..100 {
                draw.draw_circle_lines(cx as i32, cy as i32, r as f32, pointers.foreground);
            }

            PointerDrawer::new(&pointers, &angles, centre).draw(&mut draw);
        }
    }

    Ok(())
}

unsafe fn set_wm_window_type(handle: *mut Display, window: Window) {
    let xlib = libc::dlopen("libX11.so".as_ptr() as *const i8, libc::RTLD_NOW);
    if xlib.is_null() {
        return;
    }
    let display = xlib::XOpenDisplay(ptr::null());
    if display.is_null() {
        return;
    }
    let mut attr: XWindowAttributes = mem::zeroed();
    xlib::XGetWindowAttributes(handle, window, &mut attr);

    let atom: Atom = xlib::XInternAtom(handle, "_NET_WM_STATE".as_ptr() as *const i8, 0);
    let below: Atom = xlib::XInternAtom(handle, "_NET_WM_STATE_BELOW".as_ptr() as *const i8, 0);
    let hidden: Atom = xlib::XInternAtom(handle, "_NET_WM_STATE_HIDDEN".as_ptr() as *const i8, 0);
    let data = &[below, hidden] as *const _ as *mut u8;
    xlib::XChangeProperty(
        handle,
        window,
        atom,
        Atom::from(xlib::XA_ATOM),
        32,
        0,
        data,
        1,
    );
}
