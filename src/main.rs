use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use keyboard::Keycode;
use sdl2::{EventPump, keyboard};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[cfg(target_family = "wasm")]
pub mod emscripten;

struct Core {
    x: i32,
    y: i32,
    running: bool,
    event_pump: EventPump,
    canvas: WindowCanvas
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("Hello world", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl.event_pump().unwrap();

    let core = Core { x: 0, y: 0, running: true, event_pump, canvas };
    let core = Rc::new(RefCell::new(core));

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop(core.clone()));

    #[cfg(not(target_family = "wasm"))]
    {
        let mut loop_func = main_loop(core.clone());

        while core.borrow().running {
            loop_func();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

fn main_loop(core: Rc<RefCell<Core>>) -> impl FnMut() {
    move || {
        let core = &mut *core.borrow_mut();
        let event_pump = &mut core.event_pump;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    core.running = false;
                },
                _ => ()
            }
        }

        let x = &mut core.x;
        let y = &mut core.y;

        move_player(event_pump, x, y);

        let canvas = &mut core.canvas;

        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(*x, *y, 10, 10)).unwrap();

        canvas.present();
    }
}

fn move_player(event_pump: &mut EventPump, x: &mut i32, y: &mut i32) {
    let keys: HashSet<_> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

    if keys.contains(&Keycode::Up) {
        *y -= 10;
    }
    if keys.contains(&Keycode::Down) {
        *y += 10;
    }
    if keys.contains(&Keycode::Left) {
        *x -= 10;
    }
    if keys.contains(&Keycode::Right) {
        *x += 10;
    }
}
